use std::str::FromStr;

use anyhow::{bail, Context};
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::maps::{FourPointCompass, Point};

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-6.txt"
    ));

    let PuzzleInput {
        map,
        start_location,
    } = input.parse().unwrap();

    let mut current_location = start_location;
    let mut points_encountered = FxHashSet::from_iter([start_location]);
    let mut current_direction = FourPointCompass::North;

    loop {
        debug_assert!(!map[&current_location].is_obstructed());
        let Some(candidate) = current_location.shift(current_direction) else {
            break;
        };
        if map[&candidate].is_obstructed() {
            current_direction.shift_90_degrees();
        } else {
            points_encountered.insert(candidate);
            current_location = candidate;
        }
    }

    println!("{}", points_encountered.len());
}

#[derive(Debug)]
struct PuzzleInput {
    map: LabMap,
    start_location: LabPoint,
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = LabMap::default();
        let mut start_location = None;
        for (zero_based_y, line) in s.lines().enumerate() {
            for (zero_based_x, c) in line.char_indices() {
                let point = LabPoint::try_from((zero_based_x + 1, zero_based_y + 1))?;
                let contents = match c {
                    '.' => PointContents::Empty,
                    '#' => PointContents::Obstructed,
                    '^' => {
                        start_location = Some(point);
                        PointContents::Empty
                    }
                    _ => bail!("Unexpected character '{c}'!"),
                };
                map.insert(point, contents);
            }
        }
        let start_location =
            start_location.context("Expected to find '^' somewhere in the map!")?;
        Ok(Self {
            map,
            start_location,
        })
    }
}

type LabMap = FxHashMap<LabPoint, PointContents>;
type LabPoint = Point<130>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointContents {
    Obstructed,
    Empty,
}

impl PointContents {
    const fn is_obstructed(self) -> bool {
        matches!(self, Self::Obstructed)
    }
}
