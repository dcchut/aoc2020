
use crate::{ProblemInput, Solution};

pub struct Q10;

impl Solution for Q10 {
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
        let q10 = Q10;
        assert_eq!(q10.part1(&load_problem_input(1)), String::new());
    }

    #[test]
    fn test_part2_solution() {
        let q10 = Q10;
        assert_eq!(q10.part2(&load_problem_input(1)), String::new());
    }
}
