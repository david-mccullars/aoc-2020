extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<isize> {
    let mut boot_code = BootCode::parse(input);
    boot_code.run();
    Some(boot_code.accumulator)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut boot_code = BootCode::parse(input);
    boot_code.fixed_run();
    Some(boot_code.accumulator)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

struct BootCode {
    instructions: Vec<Instruction>,
    accumulator: isize,
    line: usize,
}

impl BootCode {
    fn parse(input: &str) -> Self {
        let instruction = parser!({
            "nop " value:isize => Instruction::Nop(value),
            "acc " value:isize => Instruction::Acc(value),
            "jmp " value:isize => Instruction::Jmp(value),
        });

        let instructions = parser!(lines(instruction))
            .parse(input)
            .expect("Failed to parse boot code");

        Self {
            instructions,
            accumulator: 0,
            line: 0,
        }
    }

    fn reset(&mut self) {
        self.line = 0;
        self.accumulator = 0;
    }

    fn run(&mut self) -> bool {
        self.reset();
        let mut lines_run = HashSet::new();
        loop {
            lines_run.insert(self.line);
            self.run_next();
            if lines_run.contains(&self.line) {
                return false;
            }
            if self.line == self.instructions.len() {
                return true;
            }
        }
    }

    fn run_next(&mut self) {
        match self.instructions[self.line] {
            Instruction::Nop(_) => {
                self.line += 1;
            }
            Instruction::Acc(value) => {
                self.accumulator += value;
                self.line += 1;
            }
            Instruction::Jmp(value) => {
                self.line = (self.line as isize + value) as usize;
            }
        }
    }

    fn fixed_run(&mut self) -> bool {
        for i in 0..self.instructions.len() {
            if self.modified_run(i) {
                return true;
            }
        }
        false
    }

    fn modified_run(&mut self, index: usize) -> bool {
        let original = self.instructions[index];

        let modified = match original {
            Instruction::Acc(_) => return false,
            Instruction::Nop(value) => Instruction::Jmp(value),
            Instruction::Jmp(value) => Instruction::Nop(value),
        };

        self.instructions[index] = modified;
        let result = self.run();
        self.instructions[index] = original;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
