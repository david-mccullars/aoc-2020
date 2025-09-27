extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let (card_pk, door_pk) = parser!(line(usize) line(usize))
        .parse(input)
        .expect("Failed to parse");
    let card_loop_size = find_loop_size(card_pk);
    let door_loop_size = find_loop_size(door_pk);
    let encryption_key = transform(door_pk, card_loop_size);
    assert_eq!(encryption_key, transform(card_pk, door_loop_size));
    Some(encryption_key)
}

pub fn part_two(_input: &str) -> Option<&str> {
    Some("CLAIM THE FINAL GOLD STAR!!!")
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut v = 1;
    for _ in 0..loop_size {
        v *= subject_number;
        v = v.rem_euclid(20201227);
    }
    v
}

fn find_loop_size(key: usize) -> usize {
    let mut v: usize = 1;
    for loop_size in 1.. {
        v *= 7;
        v = v.rem_euclid(20201227);
        if v == key {
            return loop_size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14897079));
    }
}
