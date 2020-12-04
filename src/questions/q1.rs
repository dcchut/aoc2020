use crate::{ProblemInput, Solution};
use std::collections::HashSet;

pub struct Q1;

fn two_sum(n: &[i64], target: i64, skip_index: Option<usize>) -> Option<i64> {
    let mut seen = HashSet::new();

    for (i, x) in n.iter().copied().enumerate() {
        if Some(i) == skip_index {
            continue;
        }

        if seen.contains(&(target - x)) {
            return Some(x);
        }
        seen.insert(x);
    }

    None
}

impl Solution for Q1 {
    /// Find the two entries that sum to 2020 and multiply them together.
    fn part1(&self, lines: &ProblemInput) -> String {
        let x = two_sum(lines.parse::<Vec<i64>>().as_slice(), 2020, None).unwrap();
        (x * (2020 - x)).to_string()
    }

    /// Find the three entries that sum to 2020 and multiply them together.
    fn part2(&self, lines: &ProblemInput) -> String {
        let nums: Vec<i64> = lines.parse();

        for (i, x) in nums.iter().copied().enumerate() {
            if let Some(y) = two_sum(nums.as_slice(), 2020 - x, Some(i)) {
                return (x * y * (2020 - x - y)).to_string();
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q1 = Q1;
        assert_eq!(q1.part1(&load_problem_input(1)), 987339.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q1 = Q1;
        assert_eq!(q1.part2(&load_problem_input(1)), 259521570.to_string());
    }
}
