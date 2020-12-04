use crate::{FromProblemInput, ProblemInput, Solution};

pub struct Q3;

impl FromProblemInput for Vec<Vec<bool>> {
    fn from(lines: &ProblemInput) -> Vec<Vec<bool>> {
        lines
            .lines
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect()
    }
}

fn slope_counter(grid: &[Vec<bool>], right: usize, down: usize) -> usize {
    grid.iter()
        .step_by(down)
        .enumerate()
        .skip(1)
        .filter(|(c, row)| row[(right * c) % row.len()])
        .count()
}

impl Solution for Q3 {
    fn part1(&self, lines: &ProblemInput) -> String {
        slope_counter(&lines.parse::<Vec<Vec<bool>>>(), 3, 1).to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let grid: Vec<Vec<bool>> = lines.parse::<Vec<Vec<bool>>>();

        (slope_counter(&grid, 1, 1)
            * slope_counter(&grid, 3, 1)
            * slope_counter(&grid, 5, 1)
            * slope_counter(&grid, 7, 1)
            * slope_counter(&grid, 1, 2))
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q3 = Q3;
        assert_eq!(q3.part1(&load_problem_input(3)), 148.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q3 = Q3;
        assert_eq!(q3.part2(&load_problem_input(3)), 727923200.to_string());
    }
}
