use itertools::Itertools;
use ndarray::prelude::*;
use nom::IResult;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Observation {
    Empty,
    Galaxy,
}

#[derive(Debug, PartialEq)]
struct InputData(Array2<Observation>);

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let data: Vec<Vec<Observation>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Observation::Empty,
                    '#' => Observation::Galaxy,
                    _ => unreachable!("Invalid observation"),
                })
                .collect()
        })
        .collect();
    IResult::Ok((
        "",
        InputData(Array2::<Observation>::from_shape_fn(
            (data.len(), data[0].len()),
            |(row, col)| data[row][col],
        )),
    ))
}

const fn manhattan((r_a, c_a): (usize, usize), (r_b, c_b): (usize, usize)) -> usize {
    r_a.abs_diff(r_b) + c_a.abs_diff(c_b)
}

struct Offsets {
    row: Vec<usize>,
    col: Vec<usize>,
}

impl Offsets {
    fn new(scale: usize, rows: &[usize], cols: &[usize]) -> Self {
        Self {
            row: (0..=rows.len())
                .scan(0, |acc, cur| {
                    if !rows.contains(&cur) {
                        *acc += scale - 1;
                    }
                    Some(*acc)
                })
                .collect(),
            col: (0..=cols.len())
                .scan(0, |acc, cur| {
                    if !cols.contains(&cur) {
                        *acc += scale - 1;
                    }
                    Some(*acc)
                })
                .collect(),
        }
    }

    fn resize(&self, (row, col): (usize, usize)) -> (usize, usize) {
        (self.row[row] + row, self.col[col] + col)
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let galaxies: Vec<(usize, usize)> = input
        .0
        .indexed_iter()
        .filter(|(_, &o)| o == Observation::Galaxy)
        .map(|(pos, _)| pos)
        .collect();
    let rows: Vec<usize> = galaxies.iter().map(|&(row, _)| row).collect();
    let cols: Vec<usize> = galaxies.iter().map(|&(_, col)| col).collect();
    let offsets: Offsets = Offsets::new(2, &rows, &cols);
    Ok(galaxies
        .iter()
        .map(|&pos| offsets.resize(pos))
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| manhattan(a, b))
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let galaxies: Vec<(usize, usize)> = input
        .0
        .indexed_iter()
        .filter(|(_, &o)| o == Observation::Galaxy)
        .map(|(pos, _)| pos)
        .collect();
    let rows: Vec<usize> = galaxies.iter().map(|&(row, _)| row).collect();
    let cols: Vec<usize> = galaxies.iter().map(|&(_, col)| col).collect();
    let offsets: Offsets = Offsets::new(1_000_000, &rows, &cols);
    Ok(galaxies
        .iter()
        .map(|&pos| offsets.resize(pos))
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| manhattan(a, b))
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_parser() {
        use Observation::*;

        assert_parser!(
            parse,
            INPUT,
            InputData(array![
                [Empty, Empty, Empty, Galaxy, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Galaxy, Empty, Empty],
                [Galaxy, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Galaxy, Empty, Empty, Empty],
                [Empty, Galaxy, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Galaxy],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Galaxy, Empty, Empty],
                [Galaxy, Empty, Empty, Empty, Galaxy, Empty, Empty, Empty, Empty, Empty]
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 374);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 82000210);
    }
}
