#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct RaceStat {
    time: u64,
    record: u64,
}

impl RaceStat {
    fn margins(&self) -> usize {
        let mut margin_iter = (1..=self.time).filter(|held_time| {
            let run_time = self.time - held_time;
            let distance = held_time * run_time;
            distance > self.record
        });
        let start = margin_iter.next().unwrap_or(0).saturating_sub(1);
        let end = margin_iter.next_back().unwrap_or(0);
        (end - start) as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
struct InputData(Vec<RaceStat>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space1, u64},
        combinator::map,
        multi::separated_list1,
        sequence::{pair, preceded, separated_pair},
    };
    let times = preceded(pair(tag("Time:"), space1), separated_list1(space1, u64));
    let distances = preceded(pair(tag("Distance:"), space1), separated_list1(space1, u64));
    let mut parser = map(
        separated_pair(times, line_ending, distances),
        |(times, distances)| {
            InputData(
                times
                    .into_iter()
                    .zip(distances)
                    .map(|(time, record)| RaceStat { time, record })
                    .collect(),
            )
        },
    );
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(InputData(races): &InputData) -> AocResult<usize> {
    Ok(races.iter().map(|race| race.margins()).product())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(InputData(races): &InputData) -> AocResult<usize> {
    let mut time: String = "".to_string();
    let mut record: String = "".to_string();
    for r in races {
        time += &r.time.to_string();
        record += &r.record.to_string();
    }
    let race = RaceStat {
        time: time.parse()?,
        record: record.parse()?,
    };
    Ok(race.margins())
}

aoc_main!(parse, part1, part2);

#[test]
fn test() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_parser!(
        parse,
        input,
        InputData(vec![
            RaceStat { time: 7, record: 9 },
            RaceStat {
                time: 15,
                record: 40,
            },
            RaceStat {
                time: 30,
                record: 200,
            }
        ])
    );
    assert_part!(parse, part1, input, 288);
    assert_part!(parse, part2, input, 71503);
}
