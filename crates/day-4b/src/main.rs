use anyhow::anyhow;
use std::str::FromStr;

use utilities::maps::{EightPointCompass, Grid, Point};

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-4.txt"
    ));

    let wordsearch: Wordsearch = input.parse().unwrap();
    let answer = wordsearch.total_christmases();
    println!("{answer}");
}

const MAX_COORDINATE: u16 = 140;
type WordsearchPoint = Point<MAX_COORDINATE>;

#[derive(Debug)]
struct Wordsearch(Grid<MAX_COORDINATE, Letter>);

impl Wordsearch {
    fn total_christmases(&self) -> usize {
        self.0
            .iter()
            .filter(|(point, letter)| self.contains_x_mas(**point, **letter))
            .count()
    }

    fn letter_at(&self, point: WordsearchPoint) -> Letter {
        self.0[&point]
    }

    fn contains_x_mas(&self, point: WordsearchPoint, letter: Letter) -> bool {
        if !letter.is_a() {
            return false;
        }

        let Some(north_west) = point.shift(EightPointCompass::NorthWest) else {
            return false;
        };
        let Some(south_east) = point.shift(EightPointCompass::SouthEast) else {
            return false;
        };
        let Some(north_east) = point.shift(EightPointCompass::NorthEast) else {
            return false;
        };
        let Some(south_west) = point.shift(EightPointCompass::SouthWest) else {
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
        let mut grid = Grid::default();
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
