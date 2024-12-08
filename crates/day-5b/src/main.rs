use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use anyhow::{bail, Context};
use indexmap::IndexSet;
use rustc_hash::FxHashMap;

fn main() {
    let input = include_str!(concat!(
        std::env!("CARGO_MANIFEST_DIR"),
        "/../../inputs/day-5.txt"
    ));

    let (rule_table, updates) = parse_input(input).unwrap();

    let mut bad_updates: Vec<(Update, HashSet<Rule>)> = updates
        .into_iter()
        .filter_map(|update| {
            let rules = rule_table.rules_for_update(&update);
            if update.satisfies_rules(rules.iter().copied()) {
                None
            } else {
                Some((update, rules))
            }
        })
        .collect();

    for (update, rules) in &mut bad_updates {
        'outer: loop {
            'inner: for rule in rules.iter().copied() {
                match update.satisfies_rule(rule) {
                    Ok(()) => continue 'inner,
                    Err((first, second)) => {
                        update.swap_indices(first, second);
                        continue 'outer;
                    }
                }
            }
            break 'outer;
        }
    }

    let answer: u16 = bad_updates
        .into_iter()
        .map(|(update, _)| u16::from(update.middle_page()))
        .sum();

    println!("{answer}");
}

fn parse_input(input: &str) -> anyhow::Result<(RuleTable, Vec<Update>)> {
    let (rules_input, updates_input) = input.split_once("\n\n").context(
        "Expected a double line break in between the rules and the updates list in the input",
    )?;

    if updates_input.contains("\n\n") {
        bail!("Expected `\n\n` to appear only once in the input!");
    }

    let rule_table = rules_input.parse()?;

    let updates = updates_input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    Ok((rule_table, updates))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    earlier: u8,
    later: u8,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (earlier, later) = s
            .split_once('|')
            .context("Expected a pipe character in a rule definition")?;

        if later.contains('|') {
            bail!("Expected only one `|` character in the input");
        }

        Ok(Self {
            earlier: earlier.parse()?,
            later: later.parse()?,
        })
    }
}

#[derive(Debug)]
struct RuleTable(FxHashMap<u8, Vec<Rule>>);

impl RuleTable {
    fn rules_for_update(&self, update: &Update) -> HashSet<Rule> {
        update
            .page_numbers
            .iter()
            .copied()
            .flat_map(|page_number| self.rules_for_page_number(page_number))
            .copied()
            .collect()
    }

    fn rules_for_page_number(&self, page_number: u8) -> &[Rule] {
        self.0
            .get(&page_number)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}

impl FromStr for RuleTable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: FxHashMap<u8, Vec<Rule>> = FxHashMap::default();

        for line in s.lines() {
            let rule: Rule = line.parse()?;
            map.entry(rule.earlier).or_default().push(rule);
            map.entry(rule.later).or_default().push(rule);
        }

        Ok(Self(map))
    }
}

#[derive(Debug)]
struct Update {
    page_numbers: IndexSet<u8>,
}

impl Update {
    fn satisfies_rules(&self, rules: impl IntoIterator<Item = Rule>) -> bool {
        rules
            .into_iter()
            .all(|rule| self.satisfies_rule(rule).is_ok())
    }

    fn satisfies_rule(&self, rule: Rule) -> Result<(), (usize, usize)> {
        let Rule { earlier, later } = rule;

        if !self.page_numbers.contains(&earlier) {
            return Ok(());
        }
        if !self.page_numbers.contains(&later) {
            return Ok(());
        }

        let mut later_index = None;

        for (index, number) in self.page_numbers.iter().enumerate() {
            if number == &earlier {
                return match later_index {
                    Some(later_index) => Err((index, later_index)),
                    None => Ok(()),
                };
            } else if number == &later {
                later_index = Some(index);
            }
        }
        unreachable!()
    }

    fn swap_indices(&mut self, first: usize, second: usize) {
        self.page_numbers.swap_indices(first, second);
    }

    fn middle_page(&self) -> u8 {
        assert_eq!(self.page_numbers.len() % 2, 1);
        self.page_numbers[self.page_numbers.len() / 2]
    }
}

impl FromStr for Update {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .map(|page_numbers| Self { page_numbers })
    }
}
