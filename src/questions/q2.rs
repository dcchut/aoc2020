use crate::ic::interpreter::ICInterpreter;
use crate::{Extract, ProblemInput, Solution};

pub struct Q2;

impl Solution for Q2 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();

        run_interpreter(&mut interpreter, 12, 2)
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();

        for noun in 0..100 {
            for verb in 0..100 {
                if run_interpreter(&mut interpreter, noun, verb) == 19_690_720 {
                    return verb + (noun * 100);
                }
            }
        }

        panic!("failed to find solution");
    }
}

fn run_interpreter(interpreter: &mut ICInterpreter, noun: i64, verb: i64) -> i64 {
    interpreter.reset();
    interpreter.state.memory[1] = noun;
    interpreter.state.memory[2] = verb;
    interpreter.run();

    interpreter.terminal_state().read(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn test_interpreter() {
        let mut interpreter = ICInterpreter::new(vec![1, 0, 0, 0, 99]);
        interpreter.run();
        assert_eq!(interpreter.terminal_state().memory, vec![2, 0, 0, 0, 99]);

        let mut interpreter = ICInterpreter::new(vec![2, 3, 0, 3, 99]);
        interpreter.run();
        assert_eq!(interpreter.terminal_state().memory, vec![2, 3, 0, 6, 99]);

        let mut interpreter = ICInterpreter::new(vec![2, 4, 4, 5, 99, 0]);
        interpreter.run();
        assert_eq!(
            interpreter.terminal_state().memory,
            vec![2, 4, 4, 5, 99, 9801]
        );

        let mut interpreter = ICInterpreter::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        interpreter.run();
        assert_eq!(
            interpreter.terminal_state().memory,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_part1_solution() {
        let q2 = Q2;
        assert_eq!(q2.part1(&load_problem_input(2)), 3_058_646);
    }

    #[test]
    fn test_part2_solution() {
        let q2 = Q2;
        assert_eq!(q2.part2(&load_problem_input(2)), 8_976);
    }
}
