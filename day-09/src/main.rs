use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct InputData(Vec<Vec<i64>>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        character::complete::{i64, line_ending, space1},
        combinator::map,
        multi::separated_list1,
    };
    let mut parser = map(
        separated_list1(line_ending, separated_list1(space1, i64)),
        InputData,
    );
    parser(input)
}

fn extrapolate_forward(data: &[i64]) -> i64 {
    if data.iter().all(|&i| i == 0) {
        0
    } else {
        let new_data: Vec<i64> = data.iter().tuple_windows().map(|(a, b)| b - a).collect();
        data.last().unwrap() + extrapolate_forward(&new_data)
    }
}

fn extrapolate_backward(data: &[i64]) -> i64 {
    if data.iter().all(|&i| i == 0) {
        0
    } else {
        let new_data: Vec<i64> = data.iter().tuple_windows().map(|(a, b)| b - a).collect();
        data.first().unwrap() - extrapolate_backward(&new_data)
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<i64> {
    Ok(input.0.iter().map(|v| extrapolate_forward(v)).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<i64> {
    Ok(input.0.iter().map(|v| extrapolate_backward(v)).sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData(vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45]
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 114);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 2);
    }
}
