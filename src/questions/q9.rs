use crate::{ProblemInput, Solution};
use itertools::Itertools;

pub struct Q9;

impl Solution for Q9 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let nums = lines.parse::<Vec<i64>>();

        for w in nums.windows(26) {
            if !w[..25].iter().combinations(2).any(|x| x[0] + x[1] == w[25]) {
                return w[25].to_string();
            }
        }

        unreachable!()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let nums = lines.parse::<Vec<i64>>();

        // Compute partial sums
        let mut partial_sums = vec![0];
        let mut total = 0;

        for &n in nums.iter() {
            total += n;
            partial_sums.push(total);
        }

        // Find a contiguous set of at least two numbers summing to 3199139634
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if partial_sums[j + 1] - partial_sums[i] == 3199139634 {
                    let mut range = nums[i..=j].to_vec();
                    range.sort_unstable();
                    return (range[0] + range[range.len() - 1]).to_string();
                }
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
        let q9 = Q9;
        assert_eq!(q9.part1(&load_problem_input(9)), 3199139634_i64.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q9 = Q9;
        assert_eq!(q9.part2(&load_problem_input(9)), 438559930_i64.to_string());
    }
}
