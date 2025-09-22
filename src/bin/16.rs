extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Notes::parse(input).invalid_number_sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Notes::parse(input).departure_product())
}

#[derive(Debug)]
struct Notes {
    rules: HashMap<String, Vec<RangeInclusive<usize>>>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Notes {
    fn parse(input: &str) -> Self {
        let rule = parser!(string(any_char+) ": " repeat_sep(a:usize "-" b:usize => a..=b, " or "));
        let nums = parser!(repeat_sep(usize, ","));
        parser!(
            a:section(hash_map(lines(rule)))
            b:section(line("your ticket:") line(nums))
            c:section(line("nearby tickets:") lines(nums))
            => Self { rules: a, your_ticket: b.1, nearby_tickets: c.1 }
        )
        .parse(input)
        .expect("Failed to parse")
    }

    fn invalid_number_sum(&self) -> usize {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|field_number| self.matching_rules(field_number).next().is_none())
            })
            .copied()
            .sum()
    }

    fn departure_product(&self) -> usize {
        let mut product = 1;
        let mut assigned = HashSet::new();
        let possibles = self.possible_field_mappings();
        for (_, field) in possibles
            .iter()
            .enumerate()
            .map(|(i, s)| (s.len(), i))
            .sorted()
        {
            let name = possibles[field]
                .difference(&assigned)
                .copied()
                .next()
                .unwrap();
            assigned.insert(name);
            if name.starts_with("departure ") {
                product *= self.your_ticket[field];
            }
        }
        product
    }

    fn matching_rules(&self, field_number: &usize) -> impl Iterator<Item = &str> {
        self.rules.iter().filter_map(move |(rule, rngs)| {
            rngs.iter()
                .any(|rng| rng.contains(&&field_number))
                .then_some(rule.as_str())
        })
    }

    fn possible_field_mappings(&self) -> Vec<HashSet<&str>> {
        self.nearby_tickets
            .iter()
            .filter_map(|ticket| {
                let possibles: Vec<HashSet<&str>> = ticket
                    .iter()
                    .map(|field_number| self.matching_rules(field_number).collect())
                    .collect();
                possibles.iter().all(|s| !s.is_empty()).then_some(possibles)
            })
            .reduce(|p1, p2| p1.iter().zip(p2.iter()).map(|(s1, s2)| s1 & s2).collect())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(71));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(143));
    }
}
