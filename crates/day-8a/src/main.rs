use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use smallvec::SmallVec;
use utilities::maps::{self, Delta};

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-8.txt"
    ));

    let locations = AntennaLocations::from_str(input).unwrap();

    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    for (_frequency, frequency_locations) in &locations {
        for pair in frequency_locations.iter().copied().combinations(2) {
            antinodes.extend(find_antinode_locations(pair[0], pair[1]));
        }
    }

    println!("{}", antinodes.len());
}

const MAX_COORDINATE: u16 = 50;
type Point = maps::Point<MAX_COORDINATE>;

#[derive(Debug)]
struct AntennaLocations(FxHashMap<Antenna, FxHashSet<Point>>);

impl<'a> IntoIterator for &'a AntennaLocations {
    type IntoIter = std::collections::hash_map::Iter<'a, Antenna, FxHashSet<Point>>;
    type Item = (&'a Antenna, &'a FxHashSet<Point>);

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromStr for AntennaLocations {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: FxHashMap<Antenna, FxHashSet<Point>> = FxHashMap::default();

        for (zero_based_y, line) in s.lines().enumerate() {
            for (zero_based_x, c) in line.char_indices() {
                if c == '.' {
                    continue;
                }
                let point = Point::try_from((zero_based_x + 1, zero_based_y + 1))?;
                map.entry(Antenna { frequency: c })
                    .or_default()
                    .insert(point);
            }
        }

        Ok(Self(map))
    }
}

/// Given two locations on the grid that are known to contain antennae with the same frequency,
/// return a list of antinode locations formed by these two antennae.
///
/// There will be exactly 0, 1 or 2 antinode locations formed by any two antennae.
fn find_antinode_locations(point_a: Point, point_b: Point) -> SmallVec<[Point; 2]> {
    let delta = Delta::try_from((point_a, point_b)).unwrap();
    let mut locations = SmallVec::default();
    if let Some(point) = point_a.apply_delta(delta) {
        locations.push(point);
    }
    if let Some(point) = point_b.apply_delta(delta.reversed()) {
        locations.push(point);
    }
    locations
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    frequency: char,
}
