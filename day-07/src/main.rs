use std::cmp::Ordering;

use itertools::Itertools;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct JokerCard(Card);

impl Ord for JokerCard {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Card::Jack, Card::Jack) => Ordering::Equal,
            (Card::Jack, _) => Ordering::Less,
            (_, Card::Jack) => Ordering::Greater,
            (s, o) => s.cmp(&o),
        }
    }
}

impl PartialOrd for JokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
struct HandStats {
    variants: usize,
    max: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn by_stats(HandStats { variants, max }: HandStats) -> Self {
        match (variants, max) {
            (1, 5) => HandType::FiveOfAKind,
            (2, 4) => HandType::FourOfAKind,
            (2, 3) => HandType::FullHouse,
            (3, 3) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, 2) => HandType::OnePair,
            (5, 1) => HandType::HighCard,
            _ => unreachable!("Impossible hand"),
        }
    }
}

impl From<HandStats> for HandType {
    fn from(value: HandStats) -> Self {
        Self::by_stats(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand([Card; 5]);

impl Hand {
    fn stats(&self) -> HandStats {
        let variant_counter = self.0.iter().counts();
        let max = variant_counter.values().max().copied().unwrap();
        HandStats {
            max,
            variants: variant_counter.len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JokerHand(Hand);

impl JokerHand {
    fn stats(&self) -> HandStats {
        let mut variant_counter = self.0 .0.iter().counts();
        let mut max = 0;
        if let Some(joker_count) = variant_counter.remove(&Card::Jack) {
            if joker_count == 5 {
                return HandStats {
                    max: 5,
                    variants: 1,
                };
            } else {
                max += joker_count
            }
        }
        max += variant_counter.values().max().copied().unwrap();
        HandStats {
            max,
            variants: variant_counter.len(),
        }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hi = self.0 .0.iter().map(|&c| JokerCard(c));
        let other_hi = other.0 .0.iter().map(|&c| JokerCard(c));
        self_hi.cmp(other_hi)
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JokerRound {
    hand: Hand,
    bet: u64,
}

impl From<&(Hand, u64)> for JokerRound {
    fn from(&(hand, bet): &(Hand, u64)) -> Self {
        Self { hand, bet }
    }
}

impl Ord for JokerRound {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_jh = JokerHand(self.hand);
        let other_jh = JokerHand(other.hand);
        HandType::from(self_jh.stats())
            .cmp(&other_jh.stats().into())
            .then_with(|| self_jh.cmp(&other_jh))
    }
}

impl PartialOrd for JokerRound {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Round {
    hand: Hand,
    bet: u64,
}

impl From<&(Hand, u64)> for Round {
    fn from(&(hand, bet): &(Hand, u64)) -> Self {
        Self { hand, bet }
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> Ordering {
        HandType::from(self.hand.stats())
            .cmp(&other.hand.stats().into())
            .then_with(|| self.hand.cmp(&other.hand))
    }
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq)]
struct InputData(Vec<(Hand, u64)>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, space1, u64},
        combinator::map,
        multi::{count, separated_list1},
        sequence::separated_pair,
    };
    let ace = map(tag("A"), |_| Card::Ace);
    let king = map(tag("K"), |_| Card::King);
    let queen = map(tag("Q"), |_| Card::Queen);
    let jack = map(tag("J"), |_| Card::Jack);
    let ten = map(tag("T"), |_| Card::Ten);
    let nine = map(tag("9"), |_| Card::Nine);
    let eight = map(tag("8"), |_| Card::Eight);
    let seven = map(tag("7"), |_| Card::Seven);
    let six = map(tag("6"), |_| Card::Six);
    let five = map(tag("5"), |_| Card::Five);
    let four = map(tag("4"), |_| Card::Four);
    let three = map(tag("3"), |_| Card::Three);
    let two = map(tag("2"), |_| Card::Two);
    let card = alt((
        ace, king, queen, jack, ten, nine, eight, seven, six, five, four, three, two,
    ));
    let cards = map(count(card, 5), |cards| Hand(cards.try_into().unwrap()));
    let round = separated_pair(cards, space1, u64);
    let mut parser = map(separated_list1(line_ending, round), |rounds| {
        InputData(rounds)
    });
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(InputData(rounds): &InputData) -> AocResult<u64> {
    Ok(rounds
        .iter()
        .map(Round::from)
        .sorted()
        .enumerate()
        .map(|(i, round)| (i as u64 + 1) * (round.bet))
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(InputData(rounds): &InputData) -> AocResult<u64> {
    Ok(rounds
        .iter()
        .map(JokerRound::from)
        .sorted()
        .enumerate()
        .map(|(i, round)| (i as u64 + 1) * (round.bet))
        .sum())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_parser!(
        parse,
        input,
        InputData(vec![
            (
                Hand([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
                765
            ),
            (
                Hand([Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]),
                684
            ),
            (
                Hand([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                28
            ),
            (
                Hand([Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]),
                220
            ),
            (
                Hand([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
                483
            )
        ])
    );
    assert_part!(parse, part1, input, 6440);
    assert_part!(parse, part2, input, 5905);
}
