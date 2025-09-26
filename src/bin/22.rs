extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use rapidhash::fast::RapidHasher;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    let mut game = Game::parse(input);
    let _winner = game.play_game(false);
    Some(game.winner_score())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut game = Game::parse(input);
    let _winner = game.play_game(true);
    Some(game.winner_score())
}

#[derive(Debug, Hash)]
struct Game {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

enum Winner {
    Player1,
    Player2,
}

impl Game {
    fn parse(input: &str) -> Self {
        let (player1, player2) = parser!(
            section(line("Player 1:") v:vec_deque(lines(usize)) => v)
            section(line("Player 2:") v:vec_deque(lines(usize)) => v)
        )
        .parse(input)
        .expect("Failed to parse");
        Self { player1, player2 }
    }

    fn play_game(&mut self, recursive: bool) -> Winner {
        let mut seen = HashSet::new();
        loop {
            if self.player2.is_empty() {
                return Winner::Player1;
            } else if self.player1.is_empty() {
                return Winner::Player2;
            }

            let hash_code = self.hash_code();
            if seen.contains(&hash_code) {
                return Winner::Player1;
            } else {
                seen.insert(hash_code);
            }

            self.play_round(recursive);
        }
    }

    fn play_round(&mut self, recursive: bool) {
        let c1 = self.player1.pop_front().unwrap();
        let c2 = self.player2.pop_front().unwrap();

        let winner = if recursive && self.player1.len() >= c1 && self.player2.len() >= c2 {
            self.subgame(c1, c2).play_game(recursive)
        } else if c1 > c2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        if matches!(winner, Winner::Player1) {
            self.player1.push_back(c1);
            self.player1.push_back(c2);
        } else {
            self.player2.push_back(c2);
            self.player2.push_back(c1);
        }
    }

    fn winner_score(&self) -> usize {
        let winner = if self.player2.is_empty() {
            &self.player1
        } else {
            &self.player2
        };
        winner
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum()
    }

    fn subgame(&self, c1: usize, c2: usize) -> Self {
        Self {
            player1: self.player1.iter().copied().take(c1).collect(),
            player2: self.player2.iter().copied().take(c2).collect(),
        }
    }

    fn hash_code(&self) -> u64 {
        let mut hasher = RapidHasher::default();
        for n in &self.player1 {
            hasher.write_usize(*n);
        }
        hasher.write_usize(0);
        for n in &self.player2 {
            hasher.write_usize(*n);
        }
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(306));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(291));
    }
}
