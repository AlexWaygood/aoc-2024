use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-2.txt"
    ));

    let answer = input
        .lines()
        .map(|line| Report::from_str(line).unwrap().safety())
        .filter(|safety| safety.is_safe())
        .count();

    println!("{answer}");
}

#[derive(Debug)]
struct Report {
    levels: Box<[u8]>,
}

impl Report {
    fn safety(&self) -> Safety {
        let mut difference_kind = None;
        for (this, next) in self.levels.iter().tuple_windows() {
            let cmp = this.cmp(next);
            if cmp.is_eq() {
                return Safety::Unsafe;
            }
            match difference_kind {
                Some(kind) => {
                    if cmp != kind {
                        return Safety::Unsafe;
                    }
                }
                None => difference_kind = Some(cmp),
            }
            if this.abs_diff(*next) > 3 {
                return Safety::Unsafe;
            }
        }
        Safety::Safe
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = vec![];
        for level in s.split_ascii_whitespace() {
            levels.push(level.parse()?);
        }
        Ok(Self {
            levels: levels.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

impl Safety {
    const fn is_safe(self) -> bool {
        matches!(self, Self::Safe)
    }
}
