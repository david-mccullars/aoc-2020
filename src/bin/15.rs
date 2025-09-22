extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    game(input, 2020)
}

pub fn part_two(input: &str) -> Option<usize> {
    game(input, 30000000)
}

fn game(input: &str, n: usize) -> Option<usize> {
    let mut seeds = parser!(line(vec_deque(repeat_sep(usize, ","))))
        .parse(input)
        .expect("Failed to parse");

    let mut total_numbers: usize = 0;
    let mut last_number = 0;
    let mut prev_spoken = OptimizedHashMap::new();
    std::iter::from_fn(move || {
        let new_number = if let Some(n) = seeds.pop_front() {
            n
        } else if let Some(x) = prev_spoken.get(&last_number) {
            total_numbers - x
        } else {
            0
        };
        prev_spoken.insert(last_number, total_numbers);
        total_numbers += 1;
        last_number = new_number;
        Some(new_number)
    })
    .nth(n - 1)
}

const SM_CUTOFF: usize = 10240;

struct OptimizedHashMap<T> {
    sm: [Option<T>; SM_CUTOFF],
    lg: HashMap<usize, T>,
}

impl<T: Copy> OptimizedHashMap<T> {
    fn new() -> Self {
        Self {
            sm: [None; SM_CUTOFF],
            lg: HashMap::new(),
        }
    }

    fn get(&self, key: &usize) -> Option<&T> {
        if *key < SM_CUTOFF {
            self.sm[*key].as_ref()
        } else {
            self.lg.get(key)
        }
    }

    fn insert(&mut self, key: usize, value: T) {
        if key < SM_CUTOFF {
            self.sm[key] = Some(value);
        } else {
            self.lg.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(175594));
    }
}
