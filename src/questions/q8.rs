use crate::{FromProblemInputLine, ProblemInput, Solution};

pub struct Q8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromProblemInputLine for Instruction {
    fn from_line(line: &str) -> Self {
        // Parse the numeric bit
        let arg = (&line[4..]).parse::<i32>().expect("failed to parse");

        match &line[..3] {
            "nop" => Instruction::Nop(arg),
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            instruction => panic!("invalid instruction {}", instruction),
        }
    }
}

struct Interpreter<'a> {
    instructions: &'a [Instruction],

    // Forward defensive play
    accumulators: Vec<i32>,
    current_accumulator: usize,

    // Index of the current instruction.
    current_instruction: usize,
}

impl<'a> Interpreter<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            accumulators: vec![0],
            current_accumulator: 0,
            current_instruction: 0,
        }
    }

    fn current_accumulator(&self) -> i32 {
        self.accumulators[self.current_accumulator]
    }

    fn current_accumulator_mut(&mut self) -> &mut i32 {
        &mut self.accumulators[self.current_accumulator]
    }

    fn current_instruction(&self) -> usize {
        self.current_instruction
    }

    fn has_terminated(&self) -> bool {
        self.current_instruction() >= self.instructions.len()
    }

    fn step(&mut self) {
        match self.instructions[self.current_instruction] {
            Instruction::Nop(_) => {
                self.current_instruction += 1;
            }
            Instruction::Acc(x) => {
                *self.current_accumulator_mut() += x;
                self.current_instruction += 1;
            }
            Instruction::Jmp(j) => {
                self.current_instruction = ((self.current_instruction as i32) + j) as usize;
            }
        }
    }

    /// Will repeatedly step the interpreter until `pred` evaluates to false.  If `pred`
    /// evaluates to false initially then the interpreter will not be ran at all.
    fn step_while<P: FnMut(&Self) -> bool>(&mut self, mut pred: P) {
        while pred(self) {
            self.step();
        }
    }
}

/// Runs an interpreter on the given collection of instructions until
/// either the instruction count exceeds the number of instructions
/// or we visit an instruction twice.
///
/// Returns the interpreter after completion.
fn run_interpreter(instructions: &[Instruction]) -> Interpreter {
    let mut seen = vec![false; instructions.len()];
    let mut interpreter = Interpreter::new(instructions);

    interpreter.step_while(|ip| {
        let inst = ip.current_instruction();
        if ip.has_terminated() || seen[inst] {
            false
        } else {
            seen[inst] = true;
            true
        }
    });

    interpreter
}

impl Solution for Q8 {
    fn part1(&self, lines: &ProblemInput) -> String {
        let instructions = lines.parse::<Vec<Instruction>>();
        let interpreter = run_interpreter(&instructions);
        interpreter.current_accumulator().to_string()
    }

    fn part2(&self, lines: &ProblemInput) -> String {
        let mut instructions = lines.parse::<Vec<Instruction>>();

        // Just try modifying each nop/jmp instruction.
        for i in 0..instructions.len() {
            let curr = instructions[i];
            instructions[i] = match curr {
                Instruction::Nop(x) => Instruction::Jmp(x),
                Instruction::Acc(_) => {
                    continue;
                }
                Instruction::Jmp(x) => Instruction::Nop(x),
            };

            let interpreter = run_interpreter(&instructions);
            if interpreter.has_terminated() {
                return interpreter.current_accumulator().to_string();
            }
            instructions[i] = curr;
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
        let q8 = Q8;
        assert_eq!(q8.part1(&load_problem_input(8)), 1337.to_string());
    }

    #[test]
    fn test_part2_solution() {
        let q8 = Q8;
        assert_eq!(q8.part2(&load_problem_input(8)), 1358.to_string());
    }
}
