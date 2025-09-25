extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Assessment::parse(input).occurrences_of_safe())
}

pub fn part_two(input: &str) -> Option<String> {
    Some(Assessment::parse(input).unsafe_ingredients().join(","))
}

struct Assessment {
    raw: Vec<(HashSet<String>, HashSet<String>)>,
}

impl Assessment {
    fn parse(input: &str) -> Self {
        let raw = parser!(lines(
            hash_set(repeat_sep(string(alpha+), " "))
            " (contains " hash_set(repeat_sep(string(alpha+), ", ")) ")"
        ))
        .parse(input)
        .expect("Failed to parse");
        Self { raw }
    }

    fn possible_sources(&self) -> HashMap<&str, HashSet<&str>> {
        let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
        for (ingredients, allergens) in &self.raw {
            let ingredients = str_set(ingredients);
            for a in allergens {
                map.entry(a)
                    .and_modify(|h| {
                        *h = h.intersection(&ingredients).copied().collect();
                    })
                    .or_insert_with(|| ingredients.clone());
            }
        }
        map
    }

    fn ingredient_allergens(&self) -> HashMap<&str, &str> {
        let mut map = HashMap::new();
        let mut possible = self.possible_sources();

        while !possible.is_empty() {
            let allergen = possible
                .iter()
                .find(|(_, i)| i.len() == 1)
                .map(|(a, _)| *a)
                .unwrap();
            let ingredient = possible
                .remove(allergen)
                .unwrap()
                .into_iter()
                .next()
                .unwrap();

            map.insert(ingredient, allergen);
            for i in possible.values_mut() {
                i.remove(ingredient);
            }
        }
        map
    }

    fn occurrences_of_safe(&self) -> usize {
        let not_safe: HashSet<_> = self.ingredient_allergens().keys().copied().collect();
        self.raw
            .iter()
            .map(|(i, _)| str_set(i).difference(&not_safe).count())
            .sum()
    }

    fn unsafe_ingredients(&self) -> Vec<&str> {
        let not_safe = self.ingredient_allergens();
        let mut ingredients: Vec<_> = not_safe.keys().copied().collect();
        ingredients.sort_by(|a, b| not_safe.get(a).cmp(&not_safe.get(b)));
        ingredients
    }
}

fn str_set(set: &HashSet<String>) -> HashSet<&str> {
    set.iter().map(|s| s.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("mxmxvkd,sqjhc,fvjkl")));
    }
}
