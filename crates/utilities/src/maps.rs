use std::num::NonZeroU16;

use anyhow::anyhow;

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

pub const ALL_EIGHT_DIRECTIONS: &[EightPointCompass] = &[
    EightPointCompass::North,
    EightPointCompass::East,
    EightPointCompass::South,
    EightPointCompass::West,
    EightPointCompass::NorthEast,
    EightPointCompass::SouthEast,
    EightPointCompass::SouthWest,
    EightPointCompass::NorthWest,
];
