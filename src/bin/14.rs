extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u64> {
    let mut mem = HashMap::new();
    for (index, value, bitmask) in parse(input) {
        mem.insert(index, bitmask.apply_v1(&value));
    }
    Some(mem.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mem = HashMap::new();
    for (index, value, bitmask) in parse(input) {
        for altindex in bitmask.apply_v2(&index) {
            mem.insert(altindex, value);
        }
    }
    Some(mem.values().sum())
}

enum Inst {
    Mask(Vec<usize>),
    Mem(u64, u64),
}

fn parse(input: &str) -> impl Iterator<Item = (u64, u64, StrangeBitmask)> {
    let mut bitmask = StrangeBitmask::default();
    parser!(lines({
        "mask = " m:char_of("01X")+ => Inst::Mask(m),
        "mem[" i:u64 "] = " v:u64 => Inst::Mem(i, v),
    }))
    .parse(input)
    .expect("Failed to parse")
    .into_iter()
    .filter_map(move |i| match i {
        Inst::Mask(m) => {
            bitmask = StrangeBitmask::new(&m);
            None
        }
        Inst::Mem(index, value) => Some((index, value, bitmask)),
    })
}

#[derive(Default, Copy, Clone)]
struct StrangeBitmask {
    x_mask: u64,
    bitmask: u64,
}

impl StrangeBitmask {
    fn new(bits: &[usize]) -> Self {
        let x_mask = to_mask(bits, 2);
        let bitmask = to_mask(bits, 1);
        Self { x_mask, bitmask }
    }

    fn apply_v1(&self, v: &u64) -> u64 {
        (v & self.x_mask) + self.bitmask
    }

    fn apply_v2(&self, v: &u64) -> impl Iterator<Item = u64> {
        let v2 = (v | self.bitmask) & !self.x_mask;
        (0..36)
            .filter_map(|i| (((self.x_mask >> i) & 0b1) > 0).then_some(1 << i))
            .powerset()
            .map(move |s| v2 | s.iter().copied().sum::<u64>())
    }
}

fn to_mask(bits: &[usize], value: usize) -> u64 {
    bits.iter()
        .fold(0, |sum, b| (sum << 1) + if *b == value { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(165));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(208));
    }
}
