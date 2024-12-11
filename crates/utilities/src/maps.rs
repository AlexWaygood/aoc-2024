use std::{
    num::NonZeroU16,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use anyhow::{anyhow, Context};
use rustc_hash::{FxBuildHasher, FxHashMap};

#[derive(Debug)]
pub struct Grid<const MAX_COORDINATE: u16, T>(FxHashMap<Point<MAX_COORDINATE>, T>);

impl<const MAX_COORDINATE: u16, T> Default for Grid<MAX_COORDINATE, T> {
    fn default() -> Self {
        Self(FxHashMap::with_capacity_and_hasher(
            usize::from(MAX_COORDINATE).pow(2),
            FxBuildHasher,
        ))
    }
}

impl<const MAX_COORDINATE: u16, T> Deref for Grid<MAX_COORDINATE, T> {
    type Target = FxHashMap<Point<MAX_COORDINATE>, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const MAX_COORDINATE: u16, T> DerefMut for Grid<MAX_COORDINATE, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const MAX_COORDINATE: u16, T> FromStr for Grid<MAX_COORDINATE, T>
where
    T: TryFrom<char, Error = anyhow::Error>,
{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Self::default();
        for (zero_based_y, line) in s.lines().enumerate() {
            for (zero_based_x, c) in line.char_indices() {
                let point = Point::try_from((zero_based_x + 1, zero_based_y + 1))?;
                let value = T::try_from(c)?;
                grid.insert(point, value);
            }
        }
        Ok(grid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate<const MAX_COORDINATE: u16>(NonZeroU16);

impl<const MAX_COORDINATE: u16> Coordinate<MAX_COORDINATE> {
    fn new(val: u16) -> Option<Self> {
        if val <= MAX_COORDINATE {
            NonZeroU16::new(val).map(Self)
        } else {
            None
        }
    }

    fn get(self) -> u16 {
        self.0.get()
    }

    fn checked_add(self, other: u16) -> Option<Self> {
        self.get().checked_add(other).and_then(Self::new)
    }

    fn checked_sub(self, other: u16) -> Option<Self> {
        self.get().checked_sub(other).and_then(Self::new)
    }

    fn difference_from(self, other: Coordinate<MAX_COORDINATE>) -> anyhow::Result<i32> {
        i32::from(self.get())
            .checked_sub_unsigned(u32::from(other.get()))
            .context("Expected values to fit into an i32")
    }

    fn try_apply_delta(self, delta: i32) -> Option<Self> {
        u32::from(self.get())
            .checked_add_signed(delta)
            .and_then(|x| u16::try_from(x).ok())
            .and_then(Self::new)
    }
}

impl<const MAX_COORDINATE: u16> TryFrom<usize> for Coordinate<MAX_COORDINATE> {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let candidate = u16::try_from(value)?;
        Self::new(candidate).map_or_else(
            || Err(anyhow!("Expected a value >=1 and <= {}", MAX_COORDINATE)),
            Ok,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<const MAX_COORDINATE: u16> {
    x: Coordinate<MAX_COORDINATE>,
    y: Coordinate<MAX_COORDINATE>,
}

impl<const MAX_COORDINATE: u16> Point<MAX_COORDINATE> {
    pub fn shift(self, direction: impl Into<EightPointCompass>) -> Option<Self> {
        let Point { x, y } = self;

        match direction.into() {
            EightPointCompass::North => y.checked_sub(1).map(|y| Point { x, y }),
            EightPointCompass::East => x.checked_add(1).map(|x| Point { x, y }),
            EightPointCompass::South => y.checked_add(1).map(|y| Point { x, y }),
            EightPointCompass::West => x.checked_sub(1).map(|x| Point { x, y }),
            EightPointCompass::NorthEast => self
                .shift(EightPointCompass::North)
                .and_then(|point| point.shift(EightPointCompass::East)),
            EightPointCompass::SouthEast => self
                .shift(EightPointCompass::South)
                .and_then(|point| point.shift(EightPointCompass::East)),
            EightPointCompass::SouthWest => self
                .shift(EightPointCompass::South)
                .and_then(|point| point.shift(EightPointCompass::West)),
            EightPointCompass::NorthWest => self
                .shift(EightPointCompass::North)
                .and_then(|point| point.shift(EightPointCompass::West)),
        }
    }

    pub fn shift_n(self, direction: EightPointCompass, n: u16) -> Option<Self> {
        let mut point = self;
        for _ in 0..n {
            point = point.shift(direction)?;
        }
        Some(point)
    }

    pub fn apply_delta(self, delta: Delta) -> Option<Self> {
        let Delta { x_delta, y_delta } = delta;
        let Point { x, y } = self;

        Some(Point {
            x: x.try_apply_delta(x_delta)?,
            y: y.try_apply_delta(y_delta)?,
        })
    }
}

impl<const MAX_COORDINATE: u16> TryFrom<(usize, usize)> for Point<MAX_COORDINATE> {
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
pub struct Delta {
    x_delta: i32,
    y_delta: i32,
}

impl Delta {
    #[must_use]
    pub fn reversed(self) -> Delta {
        let Delta { x_delta, y_delta } = self;
        Delta {
            x_delta: -x_delta,
            y_delta: -y_delta,
        }
    }
}

impl<const MAX_COORDINATE: u16> TryFrom<(Point<MAX_COORDINATE>, Point<MAX_COORDINATE>)> for Delta {
    type Error = anyhow::Error;

    fn try_from(
        value: (Point<MAX_COORDINATE>, Point<MAX_COORDINATE>),
    ) -> Result<Self, Self::Error> {
        let (point_a, point_b) = value;

        Ok(Delta {
            x_delta: point_a.x.difference_from(point_b.x)?,
            y_delta: point_a.y.difference_from(point_b.y)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EightPointCompass {
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
pub enum FourPointCompass {
    North,
    South,
    East,
    West,
}

impl FourPointCompass {
    pub fn shift_90_degrees(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        };
    }
}

impl From<FourPointCompass> for EightPointCompass {
    fn from(value: FourPointCompass) -> Self {
        match value {
            FourPointCompass::East => Self::East,
            FourPointCompass::North => Self::North,
            FourPointCompass::South => Self::South,
            FourPointCompass::West => Self::West,
        }
    }
}

pub const ALL_EIGHT_COMPASS_DIRECTIONS: &[EightPointCompass] = &[
    EightPointCompass::North,
    EightPointCompass::East,
    EightPointCompass::South,
    EightPointCompass::West,
    EightPointCompass::NorthEast,
    EightPointCompass::SouthEast,
    EightPointCompass::SouthWest,
    EightPointCompass::NorthWest,
];

pub const ALL_FOUR_COMPASS_DIRECTIONS: &[FourPointCompass] = &[
    FourPointCompass::North,
    FourPointCompass::East,
    FourPointCompass::South,
    FourPointCompass::West,
];
