#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    const fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

impl From<Vec<Kind>> for Round {
    fn from(value: Vec<Kind>) -> Self {
        let mut round = Self::new(0, 0, 0);
        for kind in value {
            match kind {
                Kind::Red(i) => round.red += i,
                Kind::Green(i) => round.green += i,
                Kind::Blue(i) => round.blue += i,
            }
        }
        round
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
enum Kind {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, PartialEq)]
struct InputData {
    games: Vec<Game>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, u32},
        combinator::map,
        multi::separated_list0,
        sequence::{delimited, pair, terminated},
    };

    let game_id = delimited(tag("Game "), u32, tag(": "));
    let red = map(terminated(u32, tag(" red")), Kind::Red);
    let green = map(terminated(u32, tag(" green")), Kind::Green);
    let blue = map(terminated(u32, tag(" blue")), Kind::Blue);
    let color = alt((red, green, blue));
    let round = map(separated_list0(tag(", "), color), Round::from);
    let rounds = separated_list0(tag("; "), round);
    let game = map(pair(game_id, rounds), |(id, rounds)| Game { id, rounds });
    let games = separated_list0(line_ending, game);
    let mut parser = map(games, |games| InputData { games });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u32> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    Ok(input
        .games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|&Round { red, green, blue }| {
                red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE
            })
        })
        .map(|Game { id, .. }| *id)
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u32> {
    Ok(input
        .games
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for &Round { red, green, blue } in &game.rounds {
                max_red = max_red.max(red);
                max_green = max_green.max(green);
                max_blue = max_blue.max(blue);
            }

            max_red * max_green * max_blue
        })
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                games: vec![
                    Game {
                        id: 1,
                        rounds: vec![
                            Round {
                                red: 4,
                                green: 0,
                                blue: 3
                            },
                            Round {
                                red: 1,
                                green: 2,
                                blue: 6
                            },
                            Round {
                                red: 0,
                                green: 2,
                                blue: 0
                            }
                        ]
                    },
                    Game {
                        id: 2,
                        rounds: vec![
                            Round {
                                red: 0,
                                green: 2,
                                blue: 1
                            },
                            Round {
                                red: 1,
                                green: 3,
                                blue: 4
                            },
                            Round {
                                red: 0,
                                green: 1,
                                blue: 1
                            }
                        ]
                    },
                    Game {
                        id: 3,
                        rounds: vec![
                            Round {
                                red: 20,
                                green: 8,
                                blue: 6
                            },
                            Round {
                                red: 4,
                                green: 13,
                                blue: 5
                            },
                            Round {
                                red: 1,
                                green: 5,
                                blue: 0
                            }
                        ]
                    },
                    Game {
                        id: 4,
                        rounds: vec![
                            Round {
                                red: 3,
                                green: 1,
                                blue: 6
                            },
                            Round {
                                red: 6,
                                green: 3,
                                blue: 0
                            },
                            Round {
                                red: 14,
                                green: 3,
                                blue: 15
                            }
                        ]
                    },
                    Game {
                        id: 5,
                        rounds: vec![
                            Round {
                                red: 6,
                                green: 3,
                                blue: 1
                            },
                            Round {
                                red: 1,
                                green: 2,
                                blue: 2
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 8);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 2286);
    }
}
