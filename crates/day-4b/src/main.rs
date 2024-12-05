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
            .filter(|(point, letter)| self.contains_x_mas(**point, **letter))
            .count()
    }

    fn letter_at(&self, point: Point) -> Letter {
        self.0[&point]
    }

    fn contains_x_mas(&self, point: Point, letter: Letter) -> bool {
        if !letter.is_a() {
            return false;
        }

        let Some(north_west) = point.shift(Direction::NorthWest) else {
            return false;
        };
        let Some(south_east) = point.shift(Direction::SouthEast) else {
            return false;
        };
        let Some(north_east) = point.shift(Direction::NorthEast) else {
            return false;
        };
        let Some(south_west) = point.shift(Direction::SouthWest) else {
            return false;
        };

        matches!(
            (self.letter_at(north_west), self.letter_at(south_east)),
            (Letter::M, Letter::S) | (Letter::S, Letter::M)
        ) && matches!(
            (self.letter_at(north_east), self.letter_at(south_west)),
            (Letter::M, Letter::S) | (Letter::S, Letter::M)
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    fn is_a(self) -> bool {
        matches!(self, Letter::A)
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
