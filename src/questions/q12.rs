
use crate::{ProblemInput, Solution};

pub struct Q12;

impl Solution for Q12 {
    fn part1(&self, _lines: &ProblemInput) -> String {
        String::new()
    }

    fn part2(&self, _lines: &ProblemInput) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q12 = Q12;
        assert_eq!(q12.part1(&load_problem_input(1)), String::new());
    }

    #[test]
    fn test_part2_solution() {
        let q12 = Q12;
        assert_eq!(q12.part2(&load_problem_input(1)), String::new());
    }
}
