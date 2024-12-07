use std::str::FromStr;

use anyhow::{bail, Context};
use itertools::Itertools;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-7.txt"
    ));

    let answer: u64 = input
        .lines()
        .map(|line| Equation::from_str(line).unwrap())
        .filter(Equation::could_be_true)
        .map(|equation| equation.test_value)
        .sum();

    println!("{answer}");
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    remaining_numbers: Box<[u64]>,
}

impl Equation {
    fn could_be_true(&self) -> bool {
        itertools::repeat_n(OPERATORS, self.remaining_numbers.len() - 1)
            .multi_cartesian_product()
            .any(|possibility| self.try_solution(possibility.into_iter().copied()))
    }

    fn try_solution(&self, operators: impl IntoIterator<Item = Operator>) -> bool {
        let mut result = self.remaining_numbers[0];
        for (number, operator) in self.remaining_numbers.iter().skip(1).zip(operators) {
            let Some(candidate) = operator.apply(result, *number) else {
                return false;
            };
            if candidate > self.test_value {
                return false;
            }
            result = candidate;
        }
        result == self.test_value
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .context("Expected substring ': ' to appear on each line")?;

        if right.contains(':') {
            bail!("Expected only one ':' character per line");
        }

        let test_value = left.parse()?;
        let remaining_numbers = right.split(' ').map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            test_value,
            remaining_numbers,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(self, left: u64, right: u64) -> Option<u64> {
        match self {
            Self::Add => left.checked_add(right),
            Self::Multiply => left.checked_mul(right),
        }
    }
}

const OPERATORS: &[Operator] = &[Operator::Add, Operator::Multiply];
