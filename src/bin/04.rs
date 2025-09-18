extern crate advent_of_code;

#[allow(unused_imports)]
use advent_of_code::*;
use fancy_regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(4);

const REQUIRED_FIELDS: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

pub fn part_one(input: &str) -> Option<usize> {
    let re = Regex::new(r"([a-z]{3}):").unwrap();
    let with_required_fields = input.split("\n\n").filter(|s| {
        let fields = re
            .captures_iter(s)
            .map(|c| c.unwrap().get(1).unwrap().as_str())
            .filter(|c| *c != "cid")
            .sorted();
        REQUIRED_FIELDS
            .iter()
            .zip(fields)
            .filter(|(a, b)| *a == b)
            .count()
            == REQUIRED_FIELDS.len()
    });
    Some(with_required_fields.count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let valid = input
        .split("\n\n")
        .filter(|s| Passport::parse(s).is_valid());
    Some(valid.count())
}

#[derive(Default)]
struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<EyeColor>,
    passport_id: Option<String>,
    country_id: Option<usize>,
}

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

enum Height {
    Cm(usize),
    In(usize),
}

enum Field {
    BirthYear(usize),
    IssueYear(usize),
    ExpirationYear(usize),
    Height(Height),
    HairColor(String),
    EyeColor(EyeColor),
    PassportId(String),
    CountryId(usize),
}

impl Passport {
    fn parse(chunk: &str) -> Self {
        let eye_color = parser!({
            "amb" => EyeColor::Amb,
            "blu" => EyeColor::Blu,
            "brn" => EyeColor::Brn,
            "gry" => EyeColor::Gry,
            "grn" => EyeColor::Grn,
            "hzl" => EyeColor::Hzl,
            "oth" => EyeColor::Oth,
        });
        let height = parser!({
            u:usize "cm" => Height::Cm(u),
            u:usize "in" => Height::In(u),
        });
        let field = parser!({
            "byr:" y:usize => Field::BirthYear(y),
            "iyr:" y:usize => Field::IssueYear(y),
            "eyr:" y:usize => Field::ExpirationYear(y),
            "hgt:" h:height => Field::Height(h),
            "hcl:#" c:string(digit_hex+) => Field::HairColor(c),
            "ecl:" c:eye_color => Field::EyeColor(c),
            "pid:" i:string(digit+) => Field::PassportId(i),
            "cid:" i:usize => Field::CountryId(i),
        });

        let mut passport = Self::default();
        for s in chunk.replace("\n", " ").split(" ") {
            match field.parse(s) {
                Ok(Field::BirthYear(v)) => passport.birth_year = Some(v),
                Ok(Field::IssueYear(v)) => passport.issue_year = Some(v),
                Ok(Field::ExpirationYear(v)) => passport.expiration_year = Some(v),
                Ok(Field::Height(v)) => passport.height = Some(v),
                Ok(Field::HairColor(v)) => passport.hair_color = Some(v),
                Ok(Field::EyeColor(v)) => passport.eye_color = Some(v),
                Ok(Field::PassportId(v)) => passport.passport_id = Some(v),
                Ok(Field::CountryId(v)) => passport.country_id = Some(v),
                _ => {}
            }
        }
        passport
    }

    fn is_valid(&self) -> bool {
        matches!(self.birth_year, Some(1920..=2002))
            && matches!(self.issue_year, Some(2010..=2020))
            && matches!(self.expiration_year, Some(2020..=2030))
            && matches!(
                self.height,
                Some(Height::Cm(150..=193) | Height::In(59..=76))
            )
            && self.hair_color.as_ref().is_some_and(|c| c.len() == 6)
            && self.eye_color.is_some()
            && self.passport_id.as_ref().is_some_and(|id| id.len() == 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }
}
