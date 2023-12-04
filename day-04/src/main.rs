use std::collections::BTreeSet;

#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: u8,
    winning_numbers: Vec<u8>,
    card_numbers: Vec<u8>,
}

#[derive(Debug, PartialEq)]
struct InputData {
    cards: Vec<Card>,
}

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, u8},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair},
    };

    let card_id = delimited(pair(tag("Card"), many1(tag(" "))), u8, tag(":"));
    let winning_numbers = many1(preceded(many1(tag(" ")), u8));
    let card_numbers = many1(preceded(many1(tag(" ")), u8));
    let numbers = separated_pair(winning_numbers, tag(" |"), card_numbers);
    let card = pair(card_id, numbers);
    let cards = separated_list1(line_ending, card);
    let mut parser = map(cards, |cards| InputData {
        cards: cards
            .into_iter()
            .map(|(id, (winning_numbers, card_numbers))| Card {
                id,
                winning_numbers: winning_numbers.try_into().unwrap(),
                card_numbers: card_numbers.try_into().unwrap(),
            })
            .collect(),
    });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(input
        .cards
        .iter()
        .map(
            |Card {
                 winning_numbers,
                 card_numbers,
                 ..
             }| {
                let winning: BTreeSet<u8> = winning_numbers.iter().cloned().collect();
                let numbers: BTreeSet<u8> = card_numbers.iter().cloned().collect();
                let total = winning.intersection(&numbers).count();
                if total > 0 {
                    2_u64.pow(total as u32 - 1)
                } else {
                    0
                }
            },
        )
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    let mut values: Vec<(u64, Card)> = input.cards.iter().cloned().map(|card| (1, card)).collect();
    let mut total: u64 = 0;

    for i in 0..values.len() - 1 {
        let (
            card_count,
            Card {
                winning_numbers,
                card_numbers,
                ..
            },
        ) = values[i].clone();
        let winning: BTreeSet<u8> = winning_numbers.iter().cloned().collect();
        let numbers: BTreeSet<u8> = card_numbers.iter().cloned().collect();
        let win_count = winning.intersection(&numbers).count();
        for j in 1..=win_count {
            values[i + j].0 += card_count;
        }
        total += card_count;
    }
    total += values.last().unwrap().0;

    Ok(total)
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_parser!(
        parse,
        input,
        InputData {
            cards: vec![
                Card {
                    id: 1,
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
                },
                Card {
                    id: 2,
                    winning_numbers: vec![13, 32, 20, 16, 61],
                    card_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19]
                },
                Card {
                    id: 3,
                    winning_numbers: vec![1, 21, 53, 59, 44],
                    card_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
                },
                Card {
                    id: 4,
                    winning_numbers: vec![41, 92, 73, 84, 69],
                    card_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83]
                },
                Card {
                    id: 5,
                    winning_numbers: vec![87, 83, 26, 28, 32],
                    card_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36]
                },
                Card {
                    id: 6,
                    winning_numbers: vec![31, 18, 13, 56, 72],
                    card_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11]
                }
            ]
        }
    );
    assert_part!(parse, part1, input, 13);
    assert_part!(parse, part2, input, 30);
}
