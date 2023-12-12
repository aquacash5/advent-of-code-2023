use std::collections::{btree_set::Intersection, BTreeSet};

#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: u8,
    numbers: BTreeSet<u8>,
    winning: BTreeSet<u8>,
}

impl<'a> Card {
    pub fn matching_numbers(&'a self) -> Intersection<'a, u8> {
        self.winning.intersection(&self.numbers)
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    cards: Vec<Card>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space1, u8},
        combinator::map,
        multi::separated_list1,
        sequence::{delimited, pair, separated_pair, tuple},
    };

    let card_id = delimited(pair(tag("Card"), space1), u8, pair(tag(":"), space1));
    let winning_numbers = map(separated_list1(space1, u8), |v| v.into_iter().collect());
    let card_numbers = map(separated_list1(space1, u8), |v| v.into_iter().collect());
    let numbers = separated_pair(
        winning_numbers,
        tuple((space1, tag("|"), space1)),
        card_numbers,
    );
    let card = map(pair(card_id, numbers), |(id, (winning, numbers))| Card {
        id,
        numbers,
        winning,
    });
    let cards = separated_list1(line_ending, card);
    let mut parser = map(cards, |cards| InputData { cards });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .cards
        .iter()
        .map(|card| card.matching_numbers().count())
        .map(|i| u32::try_from(i).expect("Count fits in u32"))
        .map(|i| i.checked_sub(1).map_or(0, |j| 2_u64.pow(j)))
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut values: Vec<(usize, Card)> = input.cards.iter().cloned().map(|c| (1, c)).collect();
    let mut total: usize = 0;

    for i in 0..values.len() {
        let (card_count, card) = values[i].clone();
        let win_count = card.matching_numbers().count();
        for j in 1..=win_count {
            values[i + j].0 += card_count;
        }
        total += card_count;
    }

    Ok(total)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData {
                cards: vec![
                    Card {
                        id: 1,
                        winning: [41, 48, 83, 86, 17].into(),
                        numbers: [83, 86, 6, 31, 17, 9, 48, 53].into()
                    },
                    Card {
                        id: 2,
                        winning: [13, 32, 20, 16, 61].into(),
                        numbers: [61, 30, 68, 82, 17, 32, 24, 19].into()
                    },
                    Card {
                        id: 3,
                        winning: [1, 21, 53, 59, 44].into(),
                        numbers: [69, 82, 63, 72, 16, 21, 14, 1].into()
                    },
                    Card {
                        id: 4,
                        winning: [41, 92, 73, 84, 69].into(),
                        numbers: [59, 84, 76, 51, 58, 5, 54, 83].into()
                    },
                    Card {
                        id: 5,
                        winning: [87, 83, 26, 28, 32].into(),
                        numbers: [88, 30, 70, 12, 93, 22, 82, 36].into()
                    },
                    Card {
                        id: 6,
                        winning: [31, 18, 13, 56, 72].into(),
                        numbers: [74, 77, 10, 23, 35, 67, 36, 11].into()
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 13);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 30);
    }
}
