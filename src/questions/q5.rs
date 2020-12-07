use crate::{FromProblemInputLine, ProblemInput, Solution};

pub struct Q5;

#[derive(Copy, Clone, Debug)]
struct BoardingPass {
    row: i32,
    col: i32,
}

impl BoardingPass {
    fn seat_id(self) -> i32 {
        self.row * 8 + self.col
    }
}

fn binary_parse<IT: Iterator<Item = char>>(it: IT, target: char) -> i32 {
    it.fold(0, |acc, x| 2 * acc + ((x == target) as i32))
}

impl FromProblemInputLine for BoardingPass {
    fn from_line(line: &str) -> Self {
        Self {
            row: binary_parse(line.chars().take(7), 'B'),
            col: binary_parse(line.chars().skip(7), 'R'),
        }
    }
}

impl Solution for Q5 {
    fn part1(&self, lines: &ProblemInput) -> String {
        lines
            .parse::<Vec<BoardingPass>>()
            .into_iter()
            .map(BoardingPass::seat_id)
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut seat_ids: Vec<_> = lines
            .parse::<Vec<BoardingPass>>()
            .into_iter()
            .map(BoardingPass::seat_id)
            .collect();
        seat_ids.sort_unstable();

        seat_ids
            .iter()
            .zip(seat_ids.iter().skip(1))
            .filter(|(x, y)| *x + 2 == **y)
            .map(|(x, _)| x + 1)
            .next()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_part1_solution() {
        let q5 = Q5;
        assert_eq!(q5.part1(&load_problem_input(5)), 878.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q5 = Q5;
        assert_eq!(q5.part2(&load_problem_input(5)), 504.to_string());
    }
}
