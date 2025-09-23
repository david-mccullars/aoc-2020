extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

advent_of_code::solution!(17);

pub type Pos3 = (isize, isize, isize);
pub type Pos4 = (isize, isize, isize, isize);

pub fn part_one(input: &str) -> Option<usize> {
    let active = parse(input, |x, y| (x, y, 0));
    Some(conway_n(active, 6).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let active = parse(input, |x, y| (x, y, 0, 0));
    Some(conway_n(active, 6).len())
}

fn parse<T: Hash + Eq>(input: &str, to_pos: impl Fn(isize, isize) -> T) -> HashSet<T> {
    let mut grid = parser!(grid_of(".#"))
        .parse(input)
        .expect("Failed to parse");
    grid.take_all('#')
        .into_iter()
        .map(|(x, y)| to_pos(x, y))
        .collect()
}

fn conway_n<T: HasAdjacent + Copy + Hash + Eq>(set: HashSet<T>, n: usize) -> HashSet<T> {
    (0..n).fold(set, |s, _| conway(&s))
}

fn conway<T: HasAdjacent + Copy + Hash + Eq>(set: &HashSet<T>) -> HashSet<T> {
    let mut new_set = HashSet::new();
    let mut all_adjacent = HashSet::new();
    for active in set {
        let mut adj_active = 0;
        for adj in active.adjacent() {
            if set.contains(&adj) {
                adj_active += 1;
            }
            all_adjacent.insert(adj);
        }
        if matches!(adj_active, 2 | 3) {
            new_set.insert(*active);
        } else {
        }
    }
    for adj in all_adjacent.difference(set) {
        let adj_adj_active = adj
            .adjacent()
            .filter(|adj_adj| set.contains(adj_adj))
            .count();
        if adj_adj_active == 3 {
            new_set.insert(*adj);
        }
    }
    new_set
}

trait HasAdjacent {
    fn adjacent(self: &Self) -> impl Iterator<Item = Self>;
}

impl HasAdjacent for Pos3 {
    fn adjacent(self: &Self) -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(move |z| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).filter_map(move |x| {
                    (x != 0 || y != 0 || z != 0).then_some((
                        self.0 + x, //
                        self.1 + y, //
                        self.2 + z, //
                    ))
                })
            })
        })
    }
}

impl HasAdjacent for Pos4 {
    fn adjacent(self: &Self) -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(move |w| {
            (-1..=1).flat_map(move |z| {
                (-1..=1).flat_map(move |y| {
                    (-1..=1).filter_map(move |x| {
                        (x != 0 || y != 0 || z != 0 || w != 0).then_some((
                            self.0 + x, //
                            self.1 + y, //
                            self.2 + z, //
                            self.3 + w, //
                        ))
                    })
                })
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(848));
    }
}
