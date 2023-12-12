use std::collections::HashMap;

#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    pub fn go_dir(&self, dir: Direction) -> String {
        match dir {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, line_ending},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{delimited, pair, separated_pair},
    };
    let right = map(tag("R"), |_| Direction::Right);
    let left = map(tag("L"), |_| Direction::Left);
    let directions = many1(alt((right, left)));
    let node = map(
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
        |(left, right): (&str, &str)| Node {
            left: left.to_string(),
            right: right.to_string(),
        },
    );
    let node = separated_pair(alphanumeric1, tag(" = "), node);
    let nodes = map(separated_list1(line_ending, node), |v| {
        v.into_iter().map(|(s, n)| (s.to_string(), n)).collect()
    });
    let mut parser = map(
        separated_pair(directions, pair(line_ending, line_ending), nodes),
        |(directions, nodes)| InputData { directions, nodes },
    );
    parser(input)
}

fn calc_cycles(
    label: impl Into<String>,
    nodes: &HashMap<String, Node>,
    directions: &[Direction],
    ending: fn(&str) -> bool,
) -> usize {
    let mut count: usize = 0;
    let mut current_label: String = label.into();
    let mut looper = directions.iter().cycle();
    while !ending(&current_label) {
        count += 1;
        current_label = nodes
            .get(&current_label)
            .unwrap()
            .go_dir(*looper.next().unwrap());
    }
    count
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(InputData { directions, nodes }: &InputData) -> AocResult<usize> {
    Ok(calc_cycles("AAA", nodes, directions, |label| {
        label == "ZZZ"
    }))
}

#[allow(clippy::unnecessary_wraps)]
fn part2(InputData { directions, nodes }: &InputData) -> AocResult<usize> {
    Ok(nodes
        .keys()
        .filter(|label| label.ends_with('A'))
        .map(|label| calc_cycles(label, nodes, directions, |l| l.ends_with('Z')))
        .fold(1_usize, lcm))
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT_1,
            InputData {
                directions: vec![Direction::Right, Direction::Left],
                nodes: HashMap::from([
                    (
                        "ZZZ".to_string(),
                        Node {
                            left: "ZZZ".to_string(),
                            right: "ZZZ".to_string()
                        }
                    ),
                    (
                        "AAA".to_string(),
                        Node {
                            left: "BBB".to_string(),
                            right: "CCC".to_string()
                        }
                    ),
                    (
                        "BBB".to_string(),
                        Node {
                            left: "DDD".to_string(),
                            right: "EEE".to_string()
                        }
                    ),
                    (
                        "CCC".to_string(),
                        Node {
                            left: "ZZZ".to_string(),
                            right: "GGG".to_string()
                        }
                    ),
                    (
                        "DDD".to_string(),
                        Node {
                            left: "DDD".to_string(),
                            right: "DDD".to_string()
                        }
                    ),
                    (
                        "EEE".to_string(),
                        Node {
                            left: "EEE".to_string(),
                            right: "EEE".to_string()
                        }
                    ),
                    (
                        "GGG".to_string(),
                        Node {
                            left: "GGG".to_string(),
                            right: "GGG".to_string()
                        }
                    )
                ])
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT_1, 2);
        assert_part!(parse, part1, INPUT_2, 6);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT_1, 2);
        assert_part!(parse, part2, INPUT_2, 6);
        assert_part!(parse, part2, INPUT_3, 6);
    }
}
