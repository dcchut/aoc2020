use crate::{FromProblemInput, ProblemInput, Skip, Solution};
use serde::de::value::{Error, MapDeserializer};
use serde::Deserialize;

pub struct Q4;

#[derive(Debug, Clone, Deserialize)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,

    #[serde(default)]
    cid: String,
}

impl Passport {
    fn is_valid(&self) -> bool {
        // byr: four digits, 1920 <= 2002
        let byr = match self.byr.parse::<i32>() {
            Ok(x) => x,
            Err(_) => return false,
        };

        if self.byr.len() != 4 || byr < 1920 || byr > 2002 {
            return false;
        }

        // iyr: four digits, at least 2010 and at most 2020.
        let iyr = match self.iyr.parse::<i32>() {
            Ok(iyr) => iyr,
            Err(_) => return false,
        };

        if self.iyr.len() != 4 || iyr < 2010 || iyr > 2020 {
            return false;
        }

        // eyr: four digits; at least 2020 and at most 2030.
        let eyr = match self.eyr.parse::<i32>() {
            Ok(x) => x,
            Err(_) => return false,
        };

        if self.eyr.len() != 4 || eyr < 2020 || eyr > 2030 {
            return false;
        }

        // hgt: a number followed by either cm or in:
        let bounds = if self.hgt.ends_with("cm") {
            (150, 193)
        } else if self.hgt.ends_with("in") {
            (59, 76)
        } else {
            return false;
        };

        // bounds check
        match (&self.hgt[..self.hgt.len() - 2]).parse::<i32>() {
            Ok(x) if x >= bounds.0 && x <= bounds.1 => {}
            _ => return false,
        };

        // hcl: a # followed by exactly six characters 0-9 or a-f.
        if !self.hcl.starts_with('#') || self.hcl.len() != 7 {
            return false;
        }
        // Check that self.hcl[1..] are all 0-9 and a-f
        for c in self.hcl.chars().skip(1) {
            if !('0'..='9').contains(&c) && !('a'..='f').contains(&c) {
                return false;
            }
        }

        // ecl: exactly one of: amb blu brn gry grn hzl oth.
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str()) {
            return false;
        }

        // pid: a nine-digit number, including leading zeroes.
        if self.pid.len() != 9 || self.pid.parse::<i32>().is_err() {
            return false;
        }

        true
    }
}

impl FromProblemInput for Option<Passport> {
    fn from(lines: &ProblemInput) -> Self {
        Passport::deserialize(MapDeserializer::<_, Error>::new(
            lines
                .lines
                .join("\n")
                .split_ascii_whitespace()
                .map(|s| (&s[..3], &s[4..])),
        ))
        .ok()
    }
}

impl FromProblemInput for Vec<Passport> {
    fn from(lines: &ProblemInput) -> Self {
        lines
            .parse::<Skip<Option<Passport>>>()
            .unwrap()
            .into_iter()
            .flatten()
            .collect()
    }
}

impl Solution for Q4 {
    fn part1(&self, lines: &ProblemInput) -> String {
        lines.parse::<Vec<Passport>>().len().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        lines
            .parse::<Vec<Passport>>()
            .into_iter()
            .filter(Passport::is_valid)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q4 = Q4;
        assert_eq!(q4.part1(&load_problem_input(4)), 264.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q4 = Q4;
        assert_eq!(q4.part2(&load_problem_input(4)), 224.to_string());
    }
}
