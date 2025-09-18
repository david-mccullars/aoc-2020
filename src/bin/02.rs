extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let policies: Vec<PasswordPolicy> = input.lines().map(PasswordPolicy::new).collect();
    Some(policies.iter().filter(|p| p.minmax_valid()).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let policies: Vec<PasswordPolicy> = input.lines().map(PasswordPolicy::new).collect();
    Some(policies.iter().filter(|p| p.position_valid()).count())
}

struct PasswordPolicy {
    v1: usize,
    v2: usize,
    char: char,
    password: String,
}

impl PasswordPolicy {
    fn new(line: &str) -> Self {
        let (v1, v2, char, password) = parser!(usize "-" usize " " any_char ": " string(alpha+))
            .parse(line)
            .expect("Failed to parse");
        Self {
            v1,
            v2,
            char,
            password: String::from(password),
        }
    }

    fn minmax_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.char).count();
        count >= self.v1 && count <= self.v2
    }

    fn position_valid(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let pos1_match = self.v1 > 0 && self.v1 <= chars.len() && chars[self.v1 - 1] == self.char;
        let pos2_match = self.v2 > 0 && self.v2 <= chars.len() && chars[self.v2 - 1] == self.char;
        pos1_match ^ pos2_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
