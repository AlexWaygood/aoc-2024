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

    let answer: u16 = updates
        .into_iter()
        .filter(|update| update.satisfies_rules(rule_table.rules_for_update(update)))
        .map(|update| u16::from(update.middle_page()))
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
        rules.into_iter().all(|rule| self.satisfies_rule(rule))
    }

    fn satisfies_rule(&self, rule: Rule) -> bool {
        let Rule { earlier, later } = rule;
        if !self.page_numbers.contains(&earlier) {
            return true;
        }
        if !self.page_numbers.contains(&later) {
            return true;
        }
        let mut found_earlier = false;
        for number in &self.page_numbers {
            if number == &earlier {
                found_earlier = true;
            } else if number == &later {
                return found_earlier;
            }
        }
        unreachable!()
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test() {
        let (rule_table, updates) = parse_input(INPUT).unwrap();
        assert_eq!(updates.len(), 6);

        assert!(updates[0].satisfies_rules(rule_table.rules_for_update(&updates[0])));
        assert_eq!(updates[0].middle_page(), 61);

        assert!(updates[1].satisfies_rules(rule_table.rules_for_update(&updates[1])));
        assert_eq!(updates[1].middle_page(), 53);

        assert!(updates[2].satisfies_rules(rule_table.rules_for_update(&updates[2])));
        assert_eq!(updates[2].middle_page(), 29);

        assert!(!updates[3].satisfies_rules(rule_table.rules_for_update(&updates[3])));
        assert_eq!(updates[3].middle_page(), 47);

        assert!(!updates[4].satisfies_rules(rule_table.rules_for_update(&updates[4])));
        assert_eq!(updates[4].middle_page(), 13);

        assert!(!updates[5].satisfies_rules(rule_table.rules_for_update(&updates[5])));
        assert_eq!(updates[5].middle_page(), 75);
    }
}
