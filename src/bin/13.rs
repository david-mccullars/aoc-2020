extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<isize> {
    let (earliest, routes) = parse(input);
    routes
        .into_iter()
        .flatten()
        .map(|i| ((-earliest).rem_euclid(i), i))
        .sorted()
        .next()
        .map(|(wait, id)| wait * id)
}

pub fn part_two(input: &str) -> Option<i128> {
    let (_, routes) = parse(input);
    let combos = routes.into_iter().enumerate().filter_map(|(pos, id)| {
        if let Some(id) = id {
            Some((-(pos as i128), id as i128))
        } else {
            None
        }
    });
    Some(chinese_remainder_theorem(combos).0)
}

fn parse(input: &str) -> (isize, Vec<Option<isize>>) {
    parser!(
        line(isize)
        line(repeat_sep({ v:isize => Some(v), 'x' => None }, ','))
    )
    .parse(input)
    .expect("Failed to parse")
}

fn chinese_remainder_theorem(equations: impl Iterator<Item = (i128, i128)>) -> (i128, i128) {
    equations.fold((0, 1), |(a1, n1), (a2, n2)| {
        let (gcd, m1, m2) = bezout(n1, n2);
        assert_eq!(gcd, 1);
        let n3 = n1 * n2;
        let a3 = (a1 * m2 * n2 + a2 * m1 * n1).rem_euclid(n3);
        (a3, n3)
    })
}

fn bezout(a: i128, b: i128) -> (i128, i128, i128) {
    // Extended Euclidean algorithm
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = bezout(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1068781));
    }
}
