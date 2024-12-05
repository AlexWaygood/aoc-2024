use anyhow::anyhow;
use rustc_hash::FxHashMap;
use std::num::NonZeroU16;
use std::str::FromStr;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-4.txt"
    ));

    let wordsearch: Wordsearch = input.parse().unwrap();
    let answer = wordsearch.total_christmases();
    println!("{answer}");
}

const MAX_COORDINATE: NonZeroU16 = NonZeroU16::new(140).unwrap();

#[derive(Debug)]
struct Wordsearch(FxHashMap<Point, Letter>);

impl Wordsearch {
    fn total_christmases(&self) -> usize {
        self.0
            .iter()
            .map(|(point, letter)| self.christmases_at(*point, *letter))
            .sum()
    }

    fn christmases_at(&self, point: Point, letter: Letter) -> usize {
        if !letter.is_x() {
            return 0;
        }
        ALL_DIRECTIONS
            .iter()
            .filter(|direction| self.contains_directional_christmas(point, **direction))
            .count()
    }

    fn letter_at(&self, point: Point) -> Letter {
        self.0[&point]
    }

    fn contains_directional_christmas(&self, point: Point, direction: Direction) -> bool {
        debug_assert!(self.letter_at(point).is_x());
        point
            .shift_n(direction, 3)
            .is_some_and(|point| self.letter_at(point).is_s())
            && point
                .shift_n(direction, 2)
                .is_some_and(|point| self.letter_at(point).is_a())
            && point
                .shift(direction)
                .is_some_and(|point| self.letter_at(point).is_m())
    }
}

impl FromStr for Wordsearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = FxHashMap::default();
        for (zero_indexed_y, line) in s.lines().enumerate() {
            for (zero_indexed_x, c) in line.char_indices() {
                grid.insert(
                    Point::try_from((zero_indexed_x + 1, zero_indexed_y + 1))?,
                    Letter::try_from(c)?,
                );
            }
        }
        Ok(Self(grid))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(NonZeroU16);

impl Coordinate {
    fn new(val: u16) -> Option<Self> {
        let candidate = NonZeroU16::new(val)?;
        (candidate <= MAX_COORDINATE).then_some(Self(candidate))
    }

    fn checked_add(self, other: u16) -> Option<Self> {
        self.0.get().checked_add(other).and_then(Self::new)
    }

    fn checked_sub(self, other: u16) -> Option<Self> {
        self.0.get().checked_sub(other).and_then(Self::new)
    }
}

impl TryFrom<usize> for Coordinate {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let candidate = u16::try_from(value)?;
        Self::new(candidate).map_or_else(
            || Err(anyhow!("Expected a value >=1 and <= {MAX_COORDINATE}")),
            Ok,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: Coordinate,
    y: Coordinate,
}

impl Point {
    fn shift(self, direction: Direction) -> Option<Self> {
        let Point { x, y } = self;

        match direction {
            Direction::North => y.checked_sub(1).map(|y| Point { x, y }),
            Direction::East => x.checked_add(1).map(|x| Point { x, y }),
            Direction::South => y.checked_add(1).map(|y| Point { x, y }),
            Direction::West => x.checked_sub(1).map(|x| Point { x, y }),
            Direction::NorthEast => self
                .shift(Direction::North)
                .and_then(|point| point.shift(Direction::East)),
            Direction::SouthEast => self
                .shift(Direction::South)
                .and_then(|point| point.shift(Direction::East)),
            Direction::SouthWest => self
                .shift(Direction::South)
                .and_then(|point| point.shift(Direction::West)),
            Direction::NorthWest => self
                .shift(Direction::North)
                .and_then(|point| point.shift(Direction::West)),
        }
    }

    fn shift_n(self, direction: Direction, n: u16) -> Option<Self> {
        let mut point = self;
        for _ in 0..n {
            point = point.shift(direction)?;
        }
        Some(point)
    }
}

impl TryFrom<(usize, usize)> for Point {
    type Error = anyhow::Error;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let (x, y) = value;
        Ok(Self {
            x: Coordinate::try_from(x)?,
            y: Coordinate::try_from(y)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::NorthWest,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn is_x(self) -> bool {
        matches!(self, Letter::X)
    }

    fn is_m(self) -> bool {
        matches!(self, Letter::M)
    }

    fn is_a(self) -> bool {
        matches!(self, Letter::A)
    }

    fn is_s(self) -> bool {
        matches!(self, Letter::S)
    }
}

impl TryFrom<char> for Letter {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => Err(anyhow!("Unexpected wordsearch character {value}")),
        }
    }
}
