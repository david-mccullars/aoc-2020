extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        Cups::parse(input, 9)
            .play_n(100)
            .cups_after_one(8)
            .fold(0, |total, i| (total * 10) + i),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        Cups::parse(input, 1_000_000)
            .play_n(10_000_000)
            .cups_after_one(2)
            .product(),
    )
}

struct Cups {
    // Wrapping linked-list implemented via fixed-size array where:
    //   cups[0] is the current cup
    //   i -> cups[i]
    cups: Vec<usize>,
}

impl Cups {
    fn parse(input: &str, max: usize) -> Self {
        let parsed = parser!(line(digit+)).parse(input).expect("Failed to parse");

        let mut cups = vec![0; max + 1];
        cups[0] = parsed[0]; // Current cup
        let extension = (parsed.len() + 1)..;
        let wrap_to_start = std::iter::once(parsed[0]);
        for (a, b) in parsed
            .into_iter()
            .chain(extension)
            .take(max)
            .chain(wrap_to_start)
            .tuple_windows()
        {
            cups[a] = b; // i -> cups[i]
        }
        Self { cups }
    }

    fn play_n(mut self, rounds: usize) -> Self {
        (0..rounds).fold(self, |c, _| c.play())
    }

    fn play(mut self) -> Self {
        let c0 = self.cups[0];
        let c1 = self.cups[c0];
        let c2 = self.cups[c1];
        let c3 = self.cups[c2];

        let mut dest = c0;
        while [c0, c1, c2, c3].contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = self.cups.len() - 1;
            }
        }

        self.cups[0] = self.cups[c3]; // [cup after 3rd removed cup] is now current
        self.cups[c0] = self.cups[c3]; // prev current    -> cup after 3rd removed cup
        self.cups[c3] = self.cups[dest]; // 4rd removed cup -> cup after destination
        self.cups[dest] = c1; // destination cup -> 1st removed cup
        self
    }

    fn cups_after_one(&self, count: usize) -> impl Iterator<Item = usize> {
        let mut current = self.cups[1]; // Start after "1"
        std::iter::from_fn(move || {
            let value = current;
            current = self.cups[current];
            Some(value)
        })
        .take(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(67384529));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(149245887792));
    }
}
