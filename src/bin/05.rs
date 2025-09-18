extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    Some(BoardingPasses::parse(input).max_id())
}

pub fn part_two(input: &str) -> Option<usize> {
    BoardingPasses::parse(input).missing()
}

struct BoardingPasses {
    ids: Vec<usize>,
}

impl BoardingPasses {
    fn parse(input: &str) -> Self {
        let pass = parser!(line:string(char_of("FBLR")+) => {
            let binary = line
                .chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    'B' | 'R' => '1',
                    _ => unreachable!(),
                })
                .collect::<String>();
            usize::from_str_radix(&binary, 2).unwrap()
        });

        let ids = parser!(lines(pass))
            .parse(input)
            .expect("Failed to parse boarding passes");

        Self { ids }
    }

    fn max_id(&self) -> usize {
        *self.ids.iter().max().unwrap()
    }

    fn missing(&self) -> Option<usize> {
        let mut sorted = self.ids.clone();
        sorted.sort();

        sorted.windows(2).find_map(|pair| {
            if pair[1] - pair[0] > 1 {
                Some(pair[0] + 1)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(120));
    }
}
