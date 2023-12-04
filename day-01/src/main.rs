use nom::IResult;
#[allow(clippy::wildcard_imports)]
use utils::*;

struct ShrinkStartStr<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> ShrinkStartStr<'a> {
    const fn new(s: &'a str) -> ShrinkStartStr<'a> {
        ShrinkStartStr { s, i: 0 }
    }
}

impl<'a> Iterator for ShrinkStartStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let sub_str = self.s.get(self.i..);
        self.i += 1;
        sub_str
    }
}

struct ExpandEndStr<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> ExpandEndStr<'a> {
    const fn new(s: &'a str) -> ExpandEndStr<'a> {
        ExpandEndStr { s, i: s.len() }
    }
}

impl<'a> Iterator for ExpandEndStr<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let sub_str = self.s.get(self.i..);
        self.i -= 1;
        sub_str
    }
}

#[derive(Debug, PartialEq)]
struct InputData {
    lines: Vec<String>,
}

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    IResult::Ok((
        "",
        InputData {
            lines: input
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect(),
        },
    ))
}

fn get_number(s: &str) -> Option<u32> {
    #[allow(clippy::option_if_let_else)]
    if let Some(i) = s.chars().next().and_then(|c| c.to_digit(10)) {
        Some(i)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u32> {
    let mut total = 0_u32;
    for line in &input.lines {
        let first = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect("No digits in string");
        let last = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect("No digits in string");
        total += (first * 10) + last;
    }
    Ok(total)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u32> {
    let mut total = 0_u32;
    for line in &input.lines {
        let first = ShrinkStartStr::new(line)
            .find_map(get_number)
            .expect("No number in string");
        let last = ExpandEndStr::new(line)
            .find_map(get_number)
            .expect("No number in string");
        total += (first * 10) + last;
    }
    Ok(total)
}

aoc_main!(parse, part1, part2);

#[test]
fn test_part1() {
    let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    assert_part!(parse, part1, input, 142);
}

#[test]
fn test_part2() {
    let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    assert_part!(parse, part2, input, 281);
}
