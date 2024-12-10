use std::str::FromStr;

use anyhow::Context;
use utilities::maps::{self, Grid, ALL_FOUR_COMPASS_DIRECTIONS};

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-10.txt"
    ));

    let map = LavaMap::from_str(input).unwrap();
    println!("{}", map.trailhead_score_sum());
}

const MAX_COORDINATE: u16 = 52;
type Point = maps::Point<MAX_COORDINATE>;
type Height = u8;

#[derive(Debug)]
struct LavaMap(Grid<MAX_COORDINATE, Height>);

impl LavaMap {
    fn height_at(&self, point: Point) -> Height {
        self.0[&point]
    }

    fn routes_to_peak_from(&self, point: Point) -> u16 {
        let current_height = self.height_at(point);
        if current_height == 9 {
            return 1;
        }
        ALL_FOUR_COMPASS_DIRECTIONS
            .iter()
            .copied()
            .filter_map(|direction| point.shift(direction))
            .filter(|new_point| self.height_at(*new_point) == current_height + 1)
            .map(|new_point| self.routes_to_peak_from(new_point))
            .sum()
    }

    fn trailhead_score(&self, point: Point, height: Height) -> u16 {
        if height != 0 {
            return 0;
        }
        self.routes_to_peak_from(point)
    }

    fn trailhead_score_sum(&self) -> u16 {
        self.0
            .iter()
            .map(|(point, height)| self.trailhead_score(*point, *height))
            .sum()
    }
}

impl FromStr for LavaMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        for (zero_based_y, line) in s.lines().enumerate() {
            for (zero_based_x, c) in line.char_indices() {
                let point = Point::try_from((zero_based_x + 1, zero_based_y + 1))?;
                let height = Height::try_from(
                    c.to_digit(10)
                        .context("Expected all points on the map to be digit characters")?,
                )?;
                grid.insert(point, height);
            }
        }
        Ok(Self(grid))
    }
}