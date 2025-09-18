extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Groups::parse(input).anyone().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Groups::parse(input).everyone().sum())
}

struct Groups {
    groups: Vec<Vec<String>>,
}

impl Groups {
    fn parse(input: &str) -> Self {
        let group_parser = parser!(lines(string(lower+)));
        let groups = parser!(sections(group_parser))
            .parse(input)
            .expect("Failed to parse groups");

        Self { groups }
    }

    fn anyone(&self) -> impl Iterator<Item = usize> {
        self.groups
            .iter()
            .map(|group| group.iter().flat_map(|line| line.chars()).unique().count())
    }

    fn everyone(&self) -> impl Iterator<Item = usize> {
        self.groups.iter().map(|group| {
            group
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|acc, set| acc.intersection(&set).copied().collect())
                .map_or(0, |set| set.len())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
