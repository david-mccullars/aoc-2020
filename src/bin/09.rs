extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(XmasEncoding::parse(input).first_invalid())
}

pub fn part_two(input: &str) -> Option<i64> {
    let xmas = XmasEncoding::parse(input);
    let weakness = xmas.first_weakness();
    Some(weakness.iter().min().unwrap() + weakness.iter().max().unwrap())
}

struct XmasEncoding {
    numbers: Vec<i64>,
    preamble_size: usize,
}

impl XmasEncoding {
    fn parse(input: &str) -> Self {
        let numbers = parser!(lines(i64))
            .parse(input)
            .expect("Failed to parse numbers");

        let preamble_size = if numbers.len() == 20 { 5 } else { 25 };

        Self {
            numbers,
            preamble_size,
        }
    }

    fn first_invalid(&self) -> i64 {
        self.numbers
            .windows(self.preamble_size + 1)
            .find_map(|window| {
                let n = window.last().unwrap();
                let previous = &window[..self.preamble_size];
                let is_invalid = previous
                    .iter()
                    .cartesian_product(previous.iter())
                    .all(|(x, y)| x == y || x + y != *n);

                if is_invalid { Some(*n) } else { None }
            })
            .expect("No invalid number found")
    }

    fn first_weakness(&self) -> Vec<i64> {
        let invalid = self.first_invalid();
        let invalid_index = self
            .numbers
            .iter()
            .position(|&n| n == invalid)
            .expect("Invalid number not found");

        for count in 2..=invalid_index {
            for start in 0..=invalid_index - count {
                let range = &self.numbers[start..start + count];
                if range.iter().sum::<i64>() == invalid {
                    return range.to_vec();
                }
            }
        }

        panic!("No weakness found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }
}
