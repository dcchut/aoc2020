use crate::{ProblemInput, Solution};
use std::collections::HashMap;

pub struct Q10;

impl Solution for Q10 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let mut adapters = lines.parse::<Vec<i64>>();
        adapters.push(0);
        adapters.sort_unstable();

        let ones = adapters.windows(2).filter(|w| w[1] - w[0] == 1).count();

        (ones * (adapters.len() - ones)).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut adapters = lines.parse::<Vec<i64>>();
        adapters.push(0);
        adapters.sort_unstable();

        let mut map = HashMap::new();
        map.insert(0, 1);

        for &n in &adapters {
            match map.get(&n).copied() {
                Some(curr) if curr > 0 => {
                    for target in (n + 1)..=(n + 3) {
                        *map.entry(target).or_default() += curr as i64;
                    }
                }
                _ => {}
            }
        }
        map[&adapters[adapters.len() - 1]].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q10 = Q10;
        assert_eq!(q10.part1(&load_problem_input(10)), 2112.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q10 = Q10;
        assert_eq!(
            q10.part2(&load_problem_input(10)),
            3022415986688_i64.to_string()
        );
    }
}
