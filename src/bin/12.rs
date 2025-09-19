extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(12);

const START: Pos = (0, 0);

pub fn part_one(input: &str) -> Option<isize> {
    let mut ferry = SimpleFerry {
        pos: START,
        dir: Direction::East,
    };
    ferry.apply_all(input);
    Some(ferry.dist(&START))
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut ferry = AdvancedFerry {
        pos: START,
        wp: Direction::North.forward_from(&Direction::East.forward_n_from(&START, 10)),
    };
    ferry.apply_all(input);
    Some(ferry.dist(&START))
}

trait Ferry {
    fn dir(&mut self, dir: Direction, qty: isize);
    fn turn(&mut self, qty: isize);
    fn fwd(&mut self, qty: isize);
    fn dist(&self, from: &Pos) -> isize;

    fn apply_all(&mut self, input: &str) {
        for i in parser!(lines(upper isize)).parse(input).unwrap() {
            self.apply(i);
        }
    }

    fn apply(&mut self, (i, qty): (char, isize)) {
        match i {
            'N' | 'S' | 'E' | 'W' => self.dir(Direction::from_char(i), qty),
            'L' => self.turn(-qty),
            'R' => self.turn(qty),
            'F' => self.fwd(qty),
            _ => panic!("Invalid instruction"),
        }
    }
}

struct SimpleFerry {
    pos: Pos,
    dir: Direction,
}

impl Ferry for SimpleFerry {
    fn dir(&mut self, dir: Direction, qty: isize) {
        self.pos = dir.forward_n_from(&self.pos, qty);
    }

    fn turn(&mut self, qty: isize) {
        for _ in (0..(qty.abs() / 90)) {
            if qty < 0 {
                self.dir = self.dir.turn_left();
            } else {
                self.dir = self.dir.turn_right();
            }
        }
    }

    fn fwd(&mut self, qty: isize) {
        self.pos = self.dir.forward_n_from(&self.pos, qty);
    }

    fn dist(&self, from: &Pos) -> isize {
        manhattan_distance(from, &self.pos)
    }
}

struct AdvancedFerry {
    pos: Pos,
    wp: Pos,
}

impl Ferry for AdvancedFerry {
    fn dir(&mut self, dir: Direction, qty: isize) {
        self.wp = dir.forward_n_from(&self.wp, qty);
    }

    fn turn(&mut self, qty: isize) {
        let d = if qty < 0 { (1, -1) } else { (-1, 1) };
        for _ in 0..(qty.abs() / 90) {
            self.wp = (self.wp.1 * d.0, self.wp.0 * d.1);
        }
    }

    fn fwd(&mut self, qty: isize) {
        self.pos = (self.pos.0 + qty * self.wp.0, self.pos.1 + qty * self.wp.1);
    }

    fn dist(&self, from: &Pos) -> isize {
        manhattan_distance(from, &self.pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
