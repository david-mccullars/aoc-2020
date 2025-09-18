extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashMap;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let chain = parse(input);
    let counts = jolt_differences(&chain).fold(HashMap::new(), |mut acc, d| {
        *acc.entry(d).or_insert(0) += 1;
        acc
    });
    Some(counts.get(&1).unwrap_or(&0) * counts.get(&3).unwrap_or(&0))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(arrangements(&parse(input)))
}

type AdapterChain = Vec<usize>;

fn parse(input: &str) -> AdapterChain {
    let mut adapters = parser!(lines(usize))
        .parse(input)
        .expect("Failed to parse adapters");

    let max = *adapters.iter().max().unwrap();
    adapters.push(0);
    adapters.push(max + 3);
    adapters.sort_unstable();

    adapters
}

fn jolt_differences(chain: &AdapterChain) -> impl Iterator<Item = usize> {
    chain.windows(2).map(|w| w[1] - w[0])
}

fn arrangements(chain: &AdapterChain) -> usize {
    let n = chain.len();
    let mut ways = vec![0usize; n];
    ways[0] = 1;

    for i in 1..n {
        for j in (0..i).rev() {
            if chain[i] - chain[j] <= 3 {
                ways[i] += ways[j];
            } else {
                break;
            }
        }
    }

    ways[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7 * 5));
    }

    #[test]
    fn test_part_one_example_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(22 * 10));
    }

    #[test]
    fn test_part_two_example_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(19208));
    }
}
