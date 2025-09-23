extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    Some(input.lines().map(|line| Arg::parse(line).eval()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| Arg::parse(line).wrap_add_in_phrase().eval())
            .sum(),
    )
}

#[derive(Debug, Clone)]
enum Arg {
    Val(usize),
    Add,
    Mul,
    Phrase(Vec<Arg>),
}

impl Arg {
    fn parse(line: &str) -> Arg {
        let mut current = vec![];
        let mut stashed = vec![];
        for mut c in line.split(" ") {
            while c.starts_with("(") {
                stashed.push(current);
                current = vec![];
                c = &c[1..];
            }
            let mut to_pop = 0;
            while c.ends_with(")") {
                to_pop += 1;
                c = &c[..(c.len() - 1)];
            }

            let a = match c {
                "+" => Arg::Add,
                "*" => Arg::Mul,
                _ => Arg::Val(c.parse::<usize>().unwrap()),
            };
            current.push(a);

            for _ in 0..to_pop {
                let a = Arg::Phrase(current);
                current = stashed.pop().unwrap();
                current.push(a);
            }
        }
        Arg::Phrase(current)
    }

    fn eval(&self) -> usize {
        match self {
            Arg::Val(v) => *v,
            Arg::Phrase(args) => {
                if let Some(a) = args.iter().next() {
                    args[1..]
                        .iter()
                        .tuples()
                        .fold(a.eval(), |total, pair| match pair {
                            (Arg::Add, a) => total + a.eval(),
                            (Arg::Mul, a) => total * a.eval(),
                            _ => panic!("Invalid pair of arguments to apply: {:?}", &pair),
                        })
                } else {
                    0
                }
            }
            Arg::Add | Arg::Mul => panic!("Can not evaluate operator alone"),
        }
    }

    fn wrap_add_in_phrase(&self) -> Arg {
        match self {
            Arg::Phrase(args) => {
                let mut args = &args[..];
                let mut muls = vec![];
                let mut adds = vec![];
                while args.len() > 1 {
                    match args[1] {
                        Arg::Mul => {
                            if adds.is_empty() {
                                muls.push(args[0].wrap_add_in_phrase());
                            } else {
                                adds.push(args[0].wrap_add_in_phrase());
                                muls.push(Arg::Phrase(adds));
                                adds = vec![];
                            }
                            muls.push(Arg::Mul);
                        }
                        Arg::Add => {
                            adds.push(args[0].wrap_add_in_phrase());
                            adds.push(Arg::Add);
                        }
                        _ => panic!("Invalid operator: {:?}", args[1]),
                    }
                    args = &args[2..];
                }
                if adds.is_empty() {
                    muls.push(args[0].wrap_add_in_phrase());
                } else {
                    adds.push(args[0].wrap_add_in_phrase());
                    muls.push(Arg::Phrase(adds));
                }
                Arg::Phrase(muls)
            }
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71 + 51 + 26 + 437 + 12240 + 13632));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(231 + 51 + 46 + 1445 + 669060 + 23340));
    }
}
