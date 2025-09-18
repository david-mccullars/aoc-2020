extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashSet;

advent_of_code::solution!(3);

const SLOPES: [(isize, isize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn part_one(input: &str) -> Option<isize> {
    Some(TobogganRun::new(input).run(3, 1))
}

pub fn part_two(input: &str) -> Option<isize> {
    let toboggan_run = TobogganRun::new(input);
    let product = SLOPES
        .iter()
        .map(|(x_delta, y_delta)| toboggan_run.run(*x_delta, *y_delta))
        .product();
    Some(product)
}

struct TobogganRun {
    trees: HashSet<Pos>,
    bounds: Pos,
}

impl TobogganRun {
    fn new(input: &str) -> Self {
        let mut grid = parser!(grid_of(".#"))
            .parse(input)
            .expect("Failed to parse");
        let trees = grid.take_all('#');
        let bounds = grid.bounds;
        Self { trees, bounds }
    }

    fn tree(&self, x: isize, y: isize) -> bool {
        let width = self.bounds.0 + 1;
        let pos = (x % width, y);
        self.trees.contains(&pos)
    }

    fn run(&self, x_delta: isize, y_delta: isize) -> isize {
        let mut x = 0;
        let mut y = 0;
        let mut trees_encountered = 0;
        let height = self.bounds.1 + 1;

        while y < height {
            if self.tree(x, y) {
                trees_encountered += 1;
            }
            x += x_delta;
            y += y_delta;
        }

        trees_encountered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
