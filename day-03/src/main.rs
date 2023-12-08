use itertools::Itertools;
use ndarray::iter::IndexedIter;
#[allow(clippy::wildcard_imports)]
use ndarray::prelude::*;
use nom::IResult;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column_start: usize,
    column_end: usize,
}

struct PartNumberIterator<'a> {
    part_number: Option<(Position, u64)>,
    indexed_iter: IndexedIter<'a, u8, Ix2>,
}

impl<'a> PartNumberIterator<'a> {
    fn new(array: &'a Array2<u8>) -> Self {
        PartNumberIterator {
            part_number: None,
            indexed_iter: array.indexed_iter(),
        }
    }
}

fn ascii_to_u64(c: u8) -> u64 {
    (c as char).to_digit(10).unwrap().into()
}

impl<'a> Iterator for PartNumberIterator<'a> {
    type Item = (Position, u64);

    fn next(&mut self) -> Option<Self::Item> {
        for ((row_0, col_0), &item) in self.indexed_iter.by_ref() {
            if let Some((pos, total)) = self.part_number {
                let Position {
                    row, column_start, ..
                } = pos;
                match (row_0 == row, item.is_ascii_digit()) {
                    (true, true) => {
                        let pos_1 = Position {
                            row,
                            column_start,
                            column_end: col_0,
                        };
                        self.part_number = Some((pos_1, (total * 10) + ascii_to_u64(item)));
                    }
                    (false, true) => {
                        let pos_1 = Position {
                            row: row_0,
                            column_start: col_0,
                            column_end: col_0,
                        };
                        self.part_number = Some((pos_1, ascii_to_u64(item)));
                        return Some((pos, total));
                    }
                    (true | false, false) => {
                        self.part_number = None;
                        return Some((pos, total));
                    }
                }
            } else if item.is_ascii_digit() {
                let pos_1 = Position {
                    row: row_0,
                    column_start: col_0,
                    column_end: col_0,
                };
                self.part_number = Some((pos_1, ascii_to_u64(item)));
            }
        }
        let temp = self.part_number;
        self.part_number = None;
        temp
    }
}

#[derive(Debug, PartialEq, Clone)]
struct InputData(Array2<u8>);

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let data: Vec<Vec<u8>> = input
        .trim()
        .split('\n')
        .map(|line| line.bytes().collect())
        .collect();
    let mut arr = Array2::<u8>::default((data.len(), data[0].len()));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = data[i][j];
        }
    }
    IResult::Ok(("", InputData(arr)))
}

fn next_to_symbol(arr: ArrayView2<u8>, pos: Position) -> bool {
    let start_row = pos.row.saturating_sub(1);
    let end_row = pos.row.saturating_add(1);

    let start_col = pos.column_start.saturating_sub(1);
    let end_col = pos.column_end.saturating_add(1);

    (start_row..=end_row)
        .cartesian_product(start_col..=end_col)
        .filter(|&(r, c)| !(r == pos.row && (pos.column_start..=pos.column_end).contains(&c)))
        .filter_map(|(r, c)| arr.get((r, c)))
        .any(|&c| c != b'.')
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<u64> {
    Ok(PartNumberIterator::new(&input.0)
        .filter(|&(pos, _)| next_to_symbol(input.0.view(), pos))
        .map(|(_, num)| num)
        .sum())
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<u64> {
    let part_numbers: Vec<(Position, u64)> = PartNumberIterator::new(&input.0).collect();
    Ok(input
        .0
        .indexed_iter()
        .filter(|&(_, &item)| item == b'*')
        .map(|(pos, _)| pos)
        .filter_map(|(r, c)| {
            let start_row = r.saturating_sub(1);
            let end_row = r.saturating_add(1);

            let start_col = c.saturating_sub(1);
            let end_col = c.saturating_add(1);

            let v: Vec<(Position, u64)> = (start_row..=end_row)
                .cartesian_product(start_col..=end_col)
                .filter_map(|(r, c)| {
                    part_numbers
                        .iter()
                        .find(
                            |(
                                Position {
                                    row,
                                    column_start,
                                    column_end,
                                },
                                _,
                            )| {
                                r == *row && (column_start..=column_end).contains(&&c)
                            },
                        )
                        .copied()
                })
                .unique()
                .collect();
            if v.len() == 2 {
                Some(v[0].1 * v[1].1)
            } else {
                None
            }
        })
        .sum())
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_parser() {
        assert_parser!(
            parse,
            INPUT,
            InputData(array![
                [52, 54, 55, 46, 46, 49, 49, 52, 46, 46],
                [46, 46, 46, 42, 46, 46, 46, 46, 46, 46],
                [46, 46, 51, 53, 46, 46, 54, 51, 51, 46],
                [46, 46, 46, 46, 46, 46, 35, 46, 46, 46],
                [54, 49, 55, 42, 46, 46, 46, 46, 46, 46],
                [46, 46, 46, 46, 46, 43, 46, 53, 56, 46],
                [46, 46, 53, 57, 50, 46, 46, 46, 46, 46],
                [46, 46, 46, 46, 46, 46, 55, 53, 53, 46],
                [46, 46, 46, 36, 46, 42, 46, 46, 46, 46],
                [46, 54, 54, 52, 46, 53, 57, 56, 46, 46]
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT, 4361);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT, 467835);
    }
}
