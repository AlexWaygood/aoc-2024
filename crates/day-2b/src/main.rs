use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};

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
        fn determine_safety<'a>(levels_iter: impl Iterator<Item = &'a u8>) -> Result<(), usize> {
            let mut difference_kind = None;
            for (i, (this, next)) in levels_iter.tuple_windows().enumerate() {
                let cmp = this.cmp(next);
                if cmp.is_eq() {
                    return Err(i);
                }
                match difference_kind {
                    Some(kind) => {
                        if cmp != kind {
                            return Err(i);
                        }
                    }
                    None => difference_kind = Some(cmp),
                }
                if this.abs_diff(*next) > 3 {
                    return Err(i);
                }
            }
            Ok(())
        }

        match determine_safety(self.levels.iter()) {
            Ok(()) => Safety::Safe,
            Err(i) => {
                for x in [i, i.saturating_sub(1), i + 1] {
                    let iterator = self
                        .levels
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, element)| (idx != x).then_some(element));
                    if determine_safety(iterator).is_ok() {
                        return Safety::Safe;
                    }
                }
                Safety::Unsafe
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("7 6 4 2 1", Safety::Safe)]
    #[test_case("1 2 7 8 9", Safety::Unsafe)]
    #[test_case("9 7 6 2 1", Safety::Unsafe)]
    #[test_case("1 3 2 4 5", Safety::Safe)]
    #[test_case("8 6 4 4 1", Safety::Safe)]
    #[test_case("1 3 6 7 9", Safety::Safe)]
    #[test_case("48 46 47 49 51 54 56", Safety::Safe)]
    #[test_case("1 1 2 3 4 5", Safety::Safe)]
    #[test_case("1 2 3 4 5 5", Safety::Safe)]
    #[test_case("5 1 2 3 4 5", Safety::Safe)]
    #[test_case("1 4 3 2 1", Safety::Safe)]
    #[test_case("1 6 7 8 9", Safety::Safe)]
    #[test_case("1 2 3 4 3", Safety::Safe)]
    #[test_case("9 8 7 6 7", Safety::Safe)]
    #[test_case("7 10 8 10 11", Safety::Safe)]
    #[test_case("29 28 27 25 26 25 22 20", Safety::Safe)]
    fn test(input: &str, expected_safety: Safety) {
        assert_eq!(Report::from_str(input).unwrap().safety(), expected_safety);
    }
}
