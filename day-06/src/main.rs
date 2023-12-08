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
        usize::try_from(end - start).unwrap_or(0)
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
    Ok(races.iter().map(RaceStat::margins).product())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(InputData(races): &InputData) -> AocResult<usize> {
    let mut time: String = String::new();
    let mut record: String = String::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData(vec![
                RaceStat { time: 7, record: 9 },
                RaceStat {
                    time: 15,
                    record: 40
                },
                RaceStat {
                    time: 30,
                    record: 200
                }
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 288);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 71503);
    }
}
