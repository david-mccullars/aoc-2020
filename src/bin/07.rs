extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    Some(containers(&parse(input), "shiny gold"))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(contents(&parse(input), "shiny gold") - 1)
}

type BagRules = HashMap<String, HashMap<String, usize>>;

fn parse(input: &str) -> BagRules {
    let name = parser!(string(lower+ " " lower+));

    parser!(hash_map(lines(
        s:name " bags contain "
        hash_map({
            repeat_sep(qty:usize " " s:name " bag" "s"? => (s, qty), ", "),
            "no other bags" => vec![],
        })
        "."
    )))
    .parse(input)
    .expect("Failed to parse")
}

fn containers(rules: &BagRules, color: &str) -> usize {
    let mut used = HashSet::new();
    find_containers(rules, color, &mut used);
    used.len()
}

fn find_containers(rules: &BagRules, color: &str, used: &mut HashSet<String>) {
    for (outside, inside) in rules {
        if inside.contains_key(color) && !used.contains(outside) {
            used.insert(outside.clone());
            find_containers(rules, outside, used);
        }
    }
}

fn contents(rules: &BagRules, color: &str) -> usize {
    rules
        .get(color)
        .map(|inside| {
            inside
                .iter()
                .map(|(c, qty)| qty * contents(rules, c))
                .sum::<usize>()
        })
        .unwrap_or(0)
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_example1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two_example2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(126));
    }
}
