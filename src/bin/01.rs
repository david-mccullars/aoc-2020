extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 3)
}

fn solve(input: &str, size: usize) -> Option<usize> {
    let numbers = parser!(lines(usize)).parse(input).expect("Failed to parse");
    numbers
        .iter()
        .combinations(size)
        .find(|combo| combo.iter().copied().sum::<usize>() == 2020)
        .map(|combo| combo.into_iter().product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
