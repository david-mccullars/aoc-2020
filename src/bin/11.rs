extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    Some(Seats::new(input, true).fill_until_stable(4).filled.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Seats::new(input, false).fill_until_stable(5).filled.len())
}

#[derive(PartialEq, Eq)]
struct Seats {
    empty: HashSet<Pos>,
    filled: HashSet<Pos>,
    bounds: Pos,
    immediate_only: bool,
}

impl Seats {
    fn new(input: &str, immediate_only: bool) -> Self {
        let mut grid = parser!(grid_of(".L#"))
            .parse(input)
            .expect("Failed to parse");
        let empty = grid.take_all('L');
        let filled = grid.take_all('#');
        let bounds = grid.bounds;
        Self {
            empty,
            filled,
            bounds,
            immediate_only,
        }
    }

    fn fill_until_stable(self, tolerance: usize) -> Self {
        let mut seats = self;
        loop {
            let seats2 = seats.fill(tolerance);
            if seats == seats2 {
                return seats2;
            } else {
                seats = seats2;
            }
        }
    }

    fn fill(&self, tolerance: usize) -> Self {
        let mut empty = HashSet::new();
        let mut filled = HashSet::new();

        for p in &self.empty {
            if self.adjacent_filled(p) == 0 {
                filled.insert(*p);
            } else {
                empty.insert(*p);
            }
        }
        for p in &self.filled {
            if self.adjacent_filled(p) >= tolerance {
                empty.insert(*p);
            } else {
                filled.insert(*p);
            }
        }

        Self {
            empty,
            filled,
            bounds: self.bounds.clone(),
            immediate_only: self.immediate_only,
        }
    }

    fn adjacent_filled(&self, p1: &Pos) -> usize {
        if self.immediate_only {
            self.immediately_adjacent_filled(p1)
        } else {
            self.adjacent_seen_filled(p1)
        }
    }

    fn immediately_adjacent_filled(&self, p1: &Pos) -> usize {
        adjacent()
            .filter(|p2| self.filled.contains(&(p1.0 + p2.0, p1.1 + p2.1)))
            .count()
    }

    fn adjacent_seen_filled(&self, p1: &Pos) -> usize {
        adjacent()
            .filter(|p2| {
                let mut p = p1.clone();
                loop {
                    p.0 += p2.0;
                    p.1 += p2.1;
                    if !self.is_in_bounds(&p) || self.empty.contains(&p) {
                        return false;
                    } else if self.filled.contains(&p) {
                        return true;
                    }
                }
            })
            .count()
    }

    fn is_in_bounds(&self, pos: &Pos) -> bool {
        (0..=self.bounds.0).contains(&pos.0) && (0..=self.bounds.0).contains(&pos.1)
    }
}

fn adjacent() -> impl Iterator<Item = Pos> {
    (-1..=1).flat_map(|y| (-1..=1).filter_map(move |x| (x != 0 || y != 0).then_some((x, y))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }
}
