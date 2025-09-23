extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use pathfinding::directed::bfs::bfs;
use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Validator::new(input).matching().count())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Validator::new(input).with_loops().matching().count())
}

#[derive(Debug)]
enum Rule {
    Rules(Vec<u8>),
    Atom(u8),
}

struct Validator {
    rules: HashMap<u8, Vec<Rule>>,
    messages: Vec<Vec<u8>>,
}

impl Validator {
    fn new(input: &str) -> Self {
        let rule = parser!({
            "\"a\"" => Rule::Atom(0),
            "\"b\"" => Rule::Atom(1),
            v:repeat_sep(u8, " ") => Rule::Rules(v),
        });
        let (rules, messages) = parser!(
            section(hash_map(lines(u8 ": " repeat_sep(rule, " | "))))
            section(lines(v:char_of("ab")+ => v.into_iter().map(|i| i as u8).collect()))
        )
        .parse(input)
        .expect("Failedt to parse");
        Self { rules, messages }
    }

    fn matching(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.messages.iter().filter(|msg| self.matches(msg))
    }

    fn matches(&self, msg: &[u8]) -> bool {
        bfs(
            &SearchState::new(),
            |ss| ss.successors(&self, msg),
            |ss| ss.success(msg),
        )
        .is_some()
    }

    fn with_loops(mut self) -> Self {
        self.rules
            .insert(8, vec![Rule::Rules(vec![42, 8]), Rule::Rules(vec![42])]);
        self.rules.insert(
            11,
            vec![Rule::Rules(vec![42, 11, 31]), Rule::Rules(vec![42, 31])],
        );
        self
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct SearchState {
    rule_ids: Vec<u8>,
    index: usize,
}

impl SearchState {
    fn new() -> Self {
        Self {
            rule_ids: vec![0],
            index: 0,
        }
    }

    fn successors(&self, validator: &Validator, msg: &[u8]) -> Vec<Self> {
        validator
            .rules
            .get(&self.rule_ids[0])
            .unwrap()
            .iter()
            .filter_map(|rule| self.successor(rule, msg))
            .collect()
    }

    fn successor(&self, rule: &Rule, msg: &[u8]) -> Option<Self> {
        match rule {
            Rule::Rules(sub_rules) => {
                let mut rule_ids = sub_rules.clone();
                rule_ids.extend_from_slice(&self.rule_ids[1..]);
                (rule_ids.len() + self.index <= msg.len()).then(|| Self {
                    rule_ids,
                    index: self.index,
                })
            }
            Rule::Atom(a) => (msg[self.index] == *a
                && (self.rule_ids.len() > 1 || self.index + 1 == msg.len()))
            .then(|| Self {
                rule_ids: self.rule_ids[1..].to_vec(),
                index: self.index + 1,
            }),
        }
    }

    fn success(&self, msg: &[u8]) -> bool {
        self.rule_ids.is_empty() && self.index == msg.len()
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(12));
    }
}
