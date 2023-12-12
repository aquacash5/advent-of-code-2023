use cached::proc_macro::cached;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[cached(size = 100)]
fn solutions_rec(report: Report, cond_index: usize, check_index: usize, block_size: u64) -> usize {
    if cond_index == report.conditions.len() {
        if report.finished_checks(check_index, block_size) {
            return 1;
        } else {
            return 0;
        }
    }
    match report.conditions[cond_index] {
        Condition::Operational => report.check_operational(cond_index, check_index, block_size),
        Condition::Damaged => report.check_damaged(cond_index, check_index, block_size),
        Condition::Unknown => {
            let first = report.check_operational(cond_index, check_index, block_size);
            let second = report.check_damaged(cond_index, check_index, block_size);
            first + second
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Report {
    conditions: Vec<Condition>,
    checks: Vec<u64>,
}

impl Report {
    fn finished_checks(&self, check_index: usize, block_size: u64) -> bool {
        check_index >= self.checks.len() - 1
            && if let Some(&check) = self.checks.get(check_index) {
                block_size == check
            } else {
                block_size == 0
            }
    }

    fn check_is_finished(&self, check_index: usize, block_size: u64) -> bool {
        check_index >= self.checks.len() || self.checks[check_index] == block_size
    }

    fn check_is_valid_block(&self, check_index: usize, block_size: u64) -> bool {
        check_index < self.checks.len() && block_size < self.checks[check_index]
    }

    fn check_operational(&self, cond_index: usize, check_index: usize, block_size: u64) -> usize {
        if self.check_is_finished(check_index, block_size) {
            solutions_rec(self.clone(), cond_index + 1, check_index + 1, 0)
        } else if block_size == 0 {
            solutions_rec(self.clone(), cond_index + 1, check_index, 0)
        } else {
            0
        }
    }

    fn check_damaged(&self, cond_index: usize, check_index: usize, block_size: u64) -> usize {
        if self.check_is_valid_block(check_index, block_size) {
            solutions_rec(self.clone(), cond_index + 1, check_index, block_size + 1)
        } else {
            0
        }
    }

    fn solutions(&self) -> usize {
        solutions_rec(self.clone(), 0, 0, 0)
    }
}

#[derive(Debug, PartialEq)]
struct InputData(Vec<Report>);

fn parse(input: &str) -> ParseResult<InputData> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, space1, u64},
        combinator::map,
        multi::{many1, separated_list1},
        sequence::separated_pair,
    };
    let operational = map(tag("."), |_| Condition::Operational);
    let damaged = map(tag("#"), |_| Condition::Damaged);
    let unknown = map(tag("?"), |_| Condition::Unknown);
    let conditions = many1(alt((operational, damaged, unknown)));
    let check = separated_list1(tag(","), u64);
    let report = map(
        separated_pair(conditions, space1, check),
        |(conditions, checks)| Report { conditions, checks },
    );
    let mut parser = map(separated_list1(line_ending, report), InputData);
    parser(input)
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    Ok(input.0.iter().map(|report| report.solutions()).sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    Ok(input
        .0
        .iter()
        .map(|report| {
            let mut conditions: Vec<Condition> = vec![];
            for _ in 0..4 {
                conditions.extend(report.conditions.iter());
                conditions.push(Condition::Unknown);
            }
            conditions.extend(report.conditions.iter());

            let mut checks: Vec<u64> = vec![];
            for _ in 0..5 {
                checks.extend(report.checks.iter());
            }
            Report { conditions, checks }
        })
        .map(|report| report.solutions())
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const LINE_1: &str = "?.#.### 1,1,3";

    #[test]
    fn test_line1() {
        assert_part!(parse, part1, LINE_1, 1);
    }

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_parser() {
        use Condition::*;

        assert_parser!(
            parse,
            INPUT,
            InputData(vec![
                Report {
                    conditions: vec![
                        Unknown,
                        Unknown,
                        Unknown,
                        Operational,
                        Damaged,
                        Damaged,
                        Damaged
                    ],
                    checks: vec![1, 1, 3]
                },
                Report {
                    conditions: vec![
                        Operational,
                        Unknown,
                        Unknown,
                        Operational,
                        Operational,
                        Unknown,
                        Unknown,
                        Operational,
                        Operational,
                        Operational,
                        Unknown,
                        Damaged,
                        Damaged,
                        Operational
                    ],
                    checks: vec![1, 1, 3]
                },
                Report {
                    conditions: vec![
                        Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged,
                        Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown
                    ],
                    checks: vec![1, 3, 1, 6]
                },
                Report {
                    conditions: vec![
                        Unknown,
                        Unknown,
                        Unknown,
                        Unknown,
                        Operational,
                        Damaged,
                        Operational,
                        Operational,
                        Operational,
                        Damaged,
                        Operational,
                        Operational,
                        Operational
                    ],
                    checks: vec![4, 1, 1]
                },
                Report {
                    conditions: vec![
                        Unknown,
                        Unknown,
                        Unknown,
                        Unknown,
                        Operational,
                        Damaged,
                        Damaged,
                        Damaged,
                        Damaged,
                        Damaged,
                        Damaged,
                        Operational,
                        Operational,
                        Damaged,
                        Damaged,
                        Damaged,
                        Damaged,
                        Damaged,
                        Operational
                    ],
                    checks: vec![1, 6, 5]
                },
                Report {
                    conditions: vec![
                        Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                        Unknown, Unknown, Unknown, Unknown
                    ],
                    checks: vec![3, 2, 1]
                }
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 21);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 525152);
    }
}
