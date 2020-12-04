use crate::{FromProblemInputLine, ProblemInput, Solution};
use std::ops::RangeInclusive;

pub struct Q2;

struct Rule {
    c: char,
    r: RangeInclusive<i32>,
    s: String,
}

impl FromProblemInputLine for Rule {
    fn from_line(line: &str) -> Self {
        let mut parts: Vec<&str> = line.split_ascii_whitespace().collect();

        // Parse the numeric portion
        let numbers: Vec<_> = parts[0]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        // Parse the middle letter
        let letter = parts[1].chars().next().unwrap();

        Rule {
            c: letter,
            r: numbers[0]..=numbers[1],
            s: parts.pop().unwrap().to_string(),
        }
    }
}

impl Solution for Q2 {
    fn part1(&self, lines: &ProblemInput) -> String {
        lines
            .parse::<Vec<Rule>>()
            .into_iter()
            .filter(|rule| {
                rule.r
                    .contains(&(rule.s.chars().filter(|&z| z == rule.c).count() as i32))
            })
            .count()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        lines
            .parse::<Vec<Rule>>()
            .into_iter()
            .filter(|rule| {
                (rule.s.chars().nth((rule.r.start() - 1) as usize).unwrap() == rule.c)
                    ^ (rule.s.chars().nth((rule.r.end() - 1) as usize).unwrap() == rule.c)
            })
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
        let q2 = Q2;
        assert_eq!(q2.part1(&load_problem_input(2)), 393.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q2 = Q2;
        assert_eq!(q2.part2(&load_problem_input(2)), 690.to_string());
    }
}
