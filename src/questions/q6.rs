use crate::{FromProblemInputLine, ProblemInput, Skip, Solution};
use std::collections::HashSet;

pub struct Q6;

impl FromProblemInputLine for HashSet<char> {
    fn from_line(line: &str) -> Self {
        line.chars().collect()
    }
}

fn apply<P: FnMut(Vec<HashSet<char>>) -> usize>(lines: &ProblemInput, f: P) -> String {
    lines
        .parse::<Skip<Vec<HashSet<char>>>>()
        .unwrap()
        .into_iter()
        .map(f)
        .sum::<usize>()
        .to_string()
}

impl Solution for Q6 {
    fn part1(&self, lines: &ProblemInput) -> String {
        apply(lines, |hc| {
            hc.into_iter().flatten().collect::<HashSet<_>>().len()
        })
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        apply(lines, |hc| {
            hc.into_iter().fold_first(|x, y| &x & &y).unwrap().len()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q6 = Q6;
        assert_eq!(q6.part1(&load_problem_input(6)), 6703.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q6 = Q6;
        assert_eq!(q6.part2(&load_problem_input(6)), 3430.to_string());
    }
}
