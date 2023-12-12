use ndarray::prelude::*;
use nom::IResult;
#[allow(clippy::wildcard_imports)]
use utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Pipe {
    #[default]
    None,
    Start,
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
}

impl Pipe {
    const fn valid_from_direction(self, direction: Direction) -> bool {
        matches!(
            (self, direction),
            (Self::Vertical, Direction::North | Direction::South)
                | (Self::Horizontal, Direction::East | Direction::West)
                | (Self::NorthToEast, Direction::South | Direction::West)
                | (Self::NorthToWest, Direction::South | Direction::East)
                | (Self::SouthToWest, Direction::North | Direction::East)
                | (Self::SouthToEast, Direction::North | Direction::West)
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum Flag {
    #[default]
    Empty,
    Null,
    Up,
    Down,
}

#[derive(Debug)]
struct PipeIterator<'a> {
    array: ArrayView2<'a, Pipe>,
    direction: Direction,
    position: (usize, usize),
    finished: bool,
}

impl<'a> PipeIterator<'a> {
    fn try_new(
        array: ArrayView2<'a, Pipe>,
        direction: Direction,
        position: (usize, usize),
    ) -> Option<Self> {
        if let Some(&pipe) = array.get(position) {
            if pipe != Pipe::Start {
                return None;
            }
        }
        let (row, col) = position;
        let position = match direction {
            Direction::North => (row.saturating_sub(1), col),
            Direction::East => (row, col.saturating_add(1)),
            Direction::South => (row.saturating_add(1), col),
            Direction::West => (row, col.saturating_sub(1)),
        };
        if let Some(&pipe) = array.get(position) {
            if pipe.valid_from_direction(direction) {
                Some(PipeIterator {
                    array,
                    direction,
                    position,
                    finished: false,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for PipeIterator<'a> {
    type Item = ((usize, usize), Pipe, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let &current_pipe = self.array.get(self.position).expect("Path has to be valid");
        let current_direction = self.direction;
        let (row, col) = self.position;

        #[allow(clippy::match_same_arms)]
        match (current_pipe, current_direction) {
            (Pipe::None, _) => unreachable!("Cannot move off Pipe::None"),
            (Pipe::Start, _) => {
                if self.finished {
                    return None;
                }
                self.finished = true;
            }

            (Pipe::Vertical, Direction::North) => self.position = (row.saturating_sub(1), col),
            (Pipe::Vertical, Direction::South) => self.position = (row.saturating_add(1), col),

            (Pipe::Horizontal, Direction::East) => self.position = (row, col.saturating_add(1)),
            (Pipe::Horizontal, Direction::West) => self.position = (row, col.saturating_sub(1)),

            (Pipe::NorthToEast, Direction::South) => {
                self.position = (row, col.saturating_add(1));
                self.direction = Direction::East;
            }
            (Pipe::NorthToEast, Direction::West) => {
                self.position = (row.saturating_sub(1), col);
                self.direction = Direction::North;
            }

            (Pipe::NorthToWest, Direction::South) => {
                self.position = (row, col.saturating_sub(1));
                self.direction = Direction::West;
            }
            (Pipe::NorthToWest, Direction::East) => {
                self.position = (row.saturating_sub(1), col);
                self.direction = Direction::North;
            }

            (Pipe::SouthToWest, Direction::North) => {
                self.position = (row, col.saturating_sub(1));
                self.direction = Direction::West;
            }
            (Pipe::SouthToWest, Direction::East) => {
                self.position = (row.saturating_add(1), col);
                self.direction = Direction::South;
            }

            (Pipe::SouthToEast, Direction::North) => {
                self.position = (row, col.saturating_add(1));
                self.direction = Direction::East;
            }
            (Pipe::SouthToEast, Direction::West) => {
                self.position = (row.saturating_add(1), col);
                self.direction = Direction::South;
            }

            (p, d) => unreachable!("Invalid Move: ({p:?}, {d:?}"),
        };

        Some(((row, col), current_pipe, current_direction))
    }
}

#[derive(Debug, PartialEq)]
struct InputData(Array2<Pipe>);

impl InputData {
    fn start(&self) -> (usize, usize) {
        self.0
            .indexed_iter()
            .find(|(_, &pipe)| pipe == Pipe::Start)
            .map(|(pos, _)| pos)
            .expect("Puzzle starts somewhere")
    }

    fn start_pipe(&self) -> Pipe {
        let (row, col) = self.start();
        let mut north = false;
        let mut east = false;
        let mut south = false;
        let mut west = false;
        // North
        if let Some(p) = self.0.get((row.saturating_sub(1), col)) {
            north = p.valid_from_direction(Direction::North);
        }
        if let Some(p) = self.0.get((row, col.saturating_add(1))) {
            east = p.valid_from_direction(Direction::East);
        }
        if let Some(p) = self.0.get((row.saturating_add(1), col)) {
            south = p.valid_from_direction(Direction::South);
        }
        if let Some(p) = self.0.get((row, col.saturating_add(1))) {
            west = p.valid_from_direction(Direction::West);
        }

        match (north, east, south, west) {
            (true, true, false, false) => Pipe::NorthToEast,
            (true, false, true, false) => Pipe::Vertical,
            (true, false, false, true) => Pipe::NorthToWest,
            (false, true, true, false) => Pipe::SouthToEast,
            (false, true, false, true) => Pipe::Horizontal,
            (false, false, true, true) => Pipe::SouthToWest,
            _ => unreachable!("should not be possible"),
        }
    }

    fn north(&self) -> Option<PipeIterator<'_>> {
        PipeIterator::try_new(self.0.view(), Direction::North, self.start())
    }

    fn east(&self) -> Option<PipeIterator<'_>> {
        PipeIterator::try_new(self.0.view(), Direction::East, self.start())
    }

    fn south(&self) -> Option<PipeIterator<'_>> {
        PipeIterator::try_new(self.0.view(), Direction::South, self.start())
    }

    fn west(&self) -> Option<PipeIterator<'_>> {
        PipeIterator::try_new(self.0.view(), Direction::West, self.start())
    }
}

#[allow(clippy::unnecessary_wraps)]
fn parse(input: &str) -> ParseResult<InputData> {
    let data: Vec<Vec<Pipe>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => Pipe::Vertical,
                    '-' => Pipe::Horizontal,
                    'L' => Pipe::NorthToEast,
                    'J' => Pipe::NorthToWest,
                    '7' => Pipe::SouthToWest,
                    'F' => Pipe::SouthToEast,
                    '.' => Pipe::None,
                    'S' => Pipe::Start,
                    _ => unreachable!("Invalid pipe"),
                })
                .collect()
        })
        .collect();
    let mut arr = Array2::<Pipe>::default((data.len(), data[0].len()));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = data[i][j];
        }
    }
    IResult::Ok(("", InputData(arr)))
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &InputData) -> AocResult<usize> {
    let mut distance: Array2<usize> = Array::zeros(input.0.raw_dim());
    distance.fill(usize::MAX);
    for pipe_iter in [input.north(), input.east(), input.south(), input.west()]
        .into_iter()
        .flatten()
    {
        for (i, (pos, _, _)) in pipe_iter.enumerate() {
            distance[pos] = distance[pos].min(i);
        }
    }
    distance[input.start()] = 0;

    Ok(distance
        .into_iter()
        .filter(|&i| i != usize::MAX)
        .max()
        .unwrap()
        + 1)
}

const fn pipe_dir_to_flag(pipe: Pipe, direction: Direction) -> Option<Flag> {
    #[allow(clippy::unnested_or_patterns)]
    match (pipe, direction) {
        (Pipe::Vertical, Direction::North)
        | (Pipe::SouthToEast, Direction::North)
        | (Pipe::NorthToWest, Direction::East)
        | (Pipe::SouthToWest, Direction::North)
        | (Pipe::NorthToEast, Direction::West) => Some(Flag::Up),
        (Pipe::Vertical, Direction::South)
        | (Pipe::NorthToWest, Direction::South)
        | (Pipe::SouthToEast, Direction::West)
        | (Pipe::SouthToWest, Direction::East)
        | (Pipe::NorthToEast, Direction::South) => Some(Flag::Down),
        _ => {
            if matches!(pipe, Pipe::None) {
                None
            } else {
                Some(Flag::Null)
            }
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &InputData) -> AocResult<usize> {
    let mut marker: Array2<Flag> = Array::default(input.0.raw_dim());

    for (pos, pipe, direction) in input
        .north()
        .or_else(|| input.east())
        .or_else(|| input.south())
        .unwrap()
    {
        if let Some(flag) = pipe_dir_to_flag(pipe, direction) {
            marker[pos] = flag;
        }
    }
    let mut total: usize = 0;
    for row in (marker).axis_iter(Axis(0)) {
        let mut count = false;
        for &flag in row {
            match (flag, count) {
                (Flag::Empty, true) => total += 1,
                (Flag::Up, _) => count = true,
                (Flag::Down, _) => count = false,
                _ => {}
            }
        }
    }
    Ok(total)
}

aoc_main!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".....\n.S-7.\n.|.|.\n.L-J.\n.....\n";
    const INPUT_2: &str = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
    const INPUT_3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const INPUT_4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_parser() {
        use Pipe::*;
        assert_parser!(
            parse,
            INPUT_1,
            InputData(array![
                [None, None, None, None, None],
                [None, Start, Horizontal, SouthToWest, None],
                [None, Vertical, None, Vertical, None],
                [None, NorthToEast, Horizontal, NorthToWest, None],
                [None, None, None, None, None]
            ])
        );
    }

    #[test]
    fn test_part1() {
        assert_part!(parse, part1, INPUT_1, 4);
        assert_part!(parse, part1, INPUT_2, 8);
    }

    #[test]
    fn test_part2() {
        assert_part!(parse, part2, INPUT_3, 4);
        assert_part!(parse, part2, INPUT_4, 10);
    }
}
