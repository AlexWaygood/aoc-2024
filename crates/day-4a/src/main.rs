use anyhow::anyhow;
use std::str::FromStr;

use utilities::maps::{EightPointCompass, Grid, Point, ALL_EIGHT_COMPASS_DIRECTIONS};

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
            .map(|(point, letter)| self.christmases_at(*point, *letter))
            .sum()
    }

    fn christmases_at(&self, point: WordsearchPoint, letter: Letter) -> usize {
        if !letter.is_x() {
            return 0;
        }
        ALL_EIGHT_COMPASS_DIRECTIONS
            .iter()
            .filter(|direction| self.contains_directional_christmas(point, **direction))
            .count()
    }

    fn letter_at(&self, point: WordsearchPoint) -> Letter {
        self.0[&point]
    }

    fn contains_directional_christmas(
        &self,
        point: WordsearchPoint,
        direction: EightPointCompass,
    ) -> bool {
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
        Grid::from_str(s).map(Self)
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
