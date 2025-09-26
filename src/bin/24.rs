extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    Some(HexFloor::parse(input).black.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut floor = HexFloor::parse(input);
    for _ in 0..100 {
        floor = floor.next_day();
    }
    Some(floor.black.len())
}

#[derive(Debug)]
struct HexFloor {
    black: HashSet<Pos>,
}

impl HexFloor {
    fn parse(input: &str) -> Self {
        let mut black = HashSet::new();
        for pos in parse(input) {
            if !black.remove(&pos) {
                black.insert(pos);
            }
        }
        Self { black }
    }

    fn adj_black_count(&self, pos: &Pos) -> usize {
        DIRECTIONS
            .iter()
            .filter(move |dir| self.black.contains(&dir.forward_from(&pos)))
            .count()
    }

    fn adj_white(&self) -> impl Iterator<Item = Pos> {
        self.black
            .iter()
            .flat_map(|pos| DIRECTIONS.iter().map(move |dir| dir.forward_from(&pos)))
            .unique()
    }

    fn next_day(&self) -> Self {
        let black_to_keep = self
            .black
            .iter()
            .filter(|pos| matches!(self.adj_black_count(&pos), 1 | 2))
            .copied();

        let white_to_flip = self
            .adj_white()
            .filter(|pos| self.adj_black_count(&pos) == 2);

        Self {
            black: black_to_keep.chain(white_to_flip).collect(),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Pos> {
    let dir = parser!({
        "se" => HexDir::SouthEast,
        "sw" => HexDir::SouthWest,
        "ne" => HexDir::NorthEast,
        "nw" => HexDir::NorthWest,
        "e" => HexDir::East,
        "w" => HexDir::West,
    });

    parser!(lines(line(dir+)))
        .parse(input)
        .expect("Failed to parse")
        .into_iter()
        .map(|dirs| {
            dirs.into_iter()
                .fold((0, 0), |pos, dir| dir.forward_from(&pos))
        })
}

#[derive(Debug)]
enum HexDir {
    SouthEast,
    SouthWest,
    NorthEast,
    NorthWest,
    East,
    West,
}

pub static DIRECTIONS: [HexDir; 6] = [
    HexDir::SouthEast,
    HexDir::SouthWest,
    HexDir::NorthEast,
    HexDir::NorthWest,
    HexDir::East,
    HexDir::West,
];

impl HexDir {
    fn forward_from(&self, pos: &Pos) -> Pos {
        match self {
            HexDir::SouthEast => (pos.0, pos.1 + 1),
            HexDir::SouthWest => (pos.0 - 1, pos.1 + 1),
            HexDir::NorthEast => (pos.0 + 1, pos.1 - 1),
            HexDir::NorthWest => (pos.0, pos.1 - 1),
            HexDir::East => (pos.0 + 1, pos.1),
            HexDir::West => (pos.0 - 1, pos.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2208));
    }
}
