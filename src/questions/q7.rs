use crate::ic::interpreter::ICInterpreter;
use crate::ic::io::{ICInput, Queue};
use crate::ic::orchestrator::ICInterpreterOrchestrator;
use crate::ic::ICPostAction;
use crate::{Extract, ProblemInput, Solution};
use std::cmp::max;
use std::collections::HashSet;

pub struct Q7;

impl Solution for Q7 {
    fn part1(&self, lines: &ProblemInput) -> i64 {
        let mut interpreter: ICInterpreter = lines.extract().unwrap();
        let mut best_output = -1;

        for i in 0..=4 {
            for j in 0..=4 {
                for k in 0..=4 {
                    for l in 0..=4 {
                        for m in 0..=4 {
                            let index_set: HashSet<_> = vec![i, j, k, l, m].into_iter().collect();
                            if index_set.len() != 5 {
                                continue;
                            }

                            // Run the interpreter
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![i, 0]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![j, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![k, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![l, last]);
                            let last = interpreter.outputs.pop().unwrap();
                            interpreter.reset();
                            interpreter.run_with_inputs(vec![m, last]);
                            let last = interpreter.outputs.pop().unwrap();

                            best_output = max(best_output, last);
                        }
                    }
                }
            }
        }

        best_output
    }

    fn part2(&self, lines: &ProblemInput) -> i64 {
        // Orchestrate the interpreters
        let mut orchestrators = ICInterpreterOrchestrator::new(vec![lines.extract().unwrap(); 5]);

        // register this postprocess with each interpreter
        for index in 0..5 {
            orchestrators.interpreters[index].postprocess(4, |_, fz: &mut ICPostAction| {
                // convert output finalization continue states to terminate
                if let ICPostAction::Continue = fz {
                    *fz = ICPostAction::Terminate;
                };
            });
        }

        let mut best_output = -1;

        for i in 5..=9 {
            for j in 5..=9 {
                for k in 5..=9 {
                    for l in 5..=9 {
                        for m in 5..=9 {
                            let index_set: HashSet<_> = vec![i, j, k, l, m].into_iter().collect();
                            if index_set.len() != 5 {
                                continue;
                            }

                            orchestrators.reset();
                            orchestrators.prime(vec![
                                ICInput::from(vec![i, 0]),
                                ICInput::single(j),
                                ICInput::single(k),
                                ICInput::single(l),
                                ICInput::single(m),
                            ]);

                            let last_value;

                            loop {
                                let state = orchestrators.run();
                                if state.opcode == 99 {
                                    last_value = orchestrators.interpreters[0].inputs.buffer[0];
                                    break;
                                }
                                orchestrators.run();
                                orchestrators.run();
                                orchestrators.run();
                                orchestrators.run();
                            }

                            best_output = max(best_output, last_value);
                        }
                    }
                }
            }
        }

        best_output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_problem_input;

    #[test]
    fn max_thruster_signal() {
        let q7 = Q7 {};

        assert_eq!(
            q7.part1(&ProblemInput::from(vec![
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            ])),
            43210
        );
        assert_eq!(
            q7.part1(&ProblemInput::from(vec![
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ])),
            54321
        );
        assert_eq!(q7.part1(&ProblemInput::from(vec!["3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"])), 65210);
    }

    #[test]
    fn test_part1_solution() {
        let q7 = Q7;
        assert_eq!(q7.part1(&load_problem_input(7)), 17_406);
    }

    #[test]
    fn test_part2_solution() {
        let q7 = Q7;
        assert_eq!(q7.part2(&load_problem_input(7)), 1_047_153);
    }
}
