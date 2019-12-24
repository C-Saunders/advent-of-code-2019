use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    ChangeRelativeBase,
    End,
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(opcode: &str) -> Result<Self, Self::Err> {
        match opcode {
            "01" => Ok(Opcode::Add),
            "02" => Ok(Opcode::Multiply),
            "03" => Ok(Opcode::Input),
            "04" => Ok(Opcode::Output),
            "05" => Ok(Opcode::JumpIfTrue),
            "06" => Ok(Opcode::JumpIfFalse),
            "07" => Ok(Opcode::LessThan),
            "08" => Ok(Opcode::Equals),
            "09" => Ok(Opcode::ChangeRelativeBase),
            "99" => Ok(Opcode::End),
            _ => Err(()),
        }
    }
}

pub enum ProgramOutput {
    Yielded(i64),
    Complete(Vec<i64>),
}

#[derive(Debug)]
pub struct IntcodeComputer {
    program: Vec<i64>,
    inputs: Vec<i64>,
    next_input_index: usize,
    outputs: Vec<i64>,
    instruction_pointer: usize,
    yield_on_output: bool,
    relative_base: usize,
}

impl IntcodeComputer {
    pub fn new(program_input: &[i64], initial_inputs: &[i64]) -> Self {
        IntcodeComputer {
            program: Vec::from(program_input),
            inputs: Vec::from(initial_inputs),
            next_input_index: 0,
            outputs: Vec::new(),
            instruction_pointer: 0,
            yield_on_output: false,
            relative_base: 0,
        }
    }

    pub fn yielding_computer(program_input: &[i64]) -> Self {
        IntcodeComputer {
            program: Vec::from(program_input),
            inputs: Vec::new(),
            next_input_index: 0,
            outputs: Vec::new(),
            instruction_pointer: 0,
            yield_on_output: true,
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, input: i64) -> () {
        self.inputs.push(input);
    }

    pub fn run_program(&mut self) -> ProgramOutput {
        let program = &mut self.program;
        let program_len = &mut program.len();
        let next_input_index = &mut self.next_input_index;
        let inputs = &self.inputs;
        let outputs = &mut self.outputs;
        let instruction_pointer = &mut self.instruction_pointer;

        let mut inputs_iter = inputs.iter().skip(*next_input_index);

        while instruction_pointer < program_len {
            let padded_instruction = format!("{:0>2}", &program[*instruction_pointer]);
            let opcode =
                Opcode::from_str(&padded_instruction[(padded_instruction.len() - 2)..]).unwrap();

            if opcode == Opcode::End {
                break;
            }

            match opcode {
                // No inputs
                Opcode::Input | Opcode::Output => {
                    let output_index = program[*instruction_pointer + 1] as usize;
                    *instruction_pointer += 2;
                    match opcode {
                        Opcode::Input => {
                            program[output_index] = *inputs_iter.next().unwrap();
                            *next_input_index += 1;
                        }
                        Opcode::Output => {
                            // TODO: Band-aid hack job
                            let output_value = if padded_instruction == "104" {
                                output_index as i64
                            } else {
                                program[output_index]
                            };

                            outputs.push(output_value);

                            if self.yield_on_output {
                                return ProgramOutput::Yielded(output_value);
                            }
                        }
                        _ => {}
                    }
                }
                // Two inputs
                _ => {
                    let parameter_modes = format!(
                        "{:0>2}",
                        &padded_instruction[..(padded_instruction.len() - 2)]
                    );

                    let parameter_values: Vec<i64> = parameter_modes
                        .chars()
                        .rev()
                        .enumerate()
                        .map(|(index, op_mode)| match op_mode {
                            '0' => program[program[*instruction_pointer + index + 1] as usize],
                            '1' => program[*instruction_pointer + index + 1],
                            _ => panic!("Bad"),
                        })
                        .collect();

                    match opcode {
                        Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => {
                            let output_index = program[*instruction_pointer + 3] as usize;
                            program[output_index] = match opcode {
                                Opcode::Add => parameter_values[0] + parameter_values[1],
                                Opcode::Multiply => parameter_values[0] * parameter_values[1],
                                Opcode::LessThan => {
                                    if parameter_values[0] < parameter_values[1] {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                Opcode::Equals => {
                                    if parameter_values[0] == parameter_values[1] {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                _ => panic!("More bad"),
                            };
                            *instruction_pointer += 4;
                        }
                        Opcode::JumpIfTrue => {
                            if parameter_values[0] != 0 {
                                &instruction_pointer.clone_from(&(parameter_values[1] as usize));
                            } else {
                                *instruction_pointer += 3;
                            }
                        }
                        Opcode::JumpIfFalse => {
                            if parameter_values[0] == 0 {
                                instruction_pointer.clone_from(&(parameter_values[1] as usize));
                            } else {
                                *instruction_pointer += 3;
                            }
                        }
                        _ => {}
                    }
                }
            };
        }

        ProgramOutput::Complete(outputs.to_vec())
    }
}

pub fn run_program_no_io(program: &[i64]) -> Vec<i64> {
    let mut computer = IntcodeComputer::new(program, &vec![]);
    computer.run_program();
    computer.program
}

pub fn run_program(program: &[i64], inputs: &[i64]) -> Vec<i64> {
    let mut computer = IntcodeComputer::new(program, inputs);
    computer.run_program();
    computer.outputs
}

#[cfg(test)]
mod no_io {
    use super::run_program_no_io;

    #[test]
    fn simple_examples() {
        assert_eq!(
            run_program_no_io(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[0],
            3500
        );
        assert_eq!(run_program_no_io(&[1, 0, 0, 0, 99])[0], 2);
        assert_eq!(run_program_no_io(&[1, 1, 1, 4, 99, 5, 6, 0, 99])[0], 30);
    }

    #[test]
    fn negative_immediate() {
        assert_eq!(run_program_no_io(&[1101, 100, -1, 4, 0])[4], 99)
    }
}

#[cfg(test)]
mod equality {
    use super::run_program;

    #[test]
    fn equality_position_mode_true() {
        let outputs = run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_position_mode_false() {
        let outputs = run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &vec![-8]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn equality_immediate_mode_true() {
        let outputs = run_program(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &vec![8]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_immediate_mode_false() {
        let outputs = run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &vec![-8]);
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod less_than {
    use super::run_program;

    #[test]
    fn less_than_position_mode_true() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8 - 1]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_position_mode_false() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8 + 1]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_position_mode_false_equal() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_true() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8 - 1]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_immediate_mode_false() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8 + 1]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_false_equal() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8]);
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod jumps {
    use super::run_program;

    #[test]
    fn position_mode() {
        let outputs = run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &vec![0],
        );
        assert_eq!(outputs[0], 0);

        let outputs_2 = run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &vec![10],
        );
        assert_eq!(outputs_2[0], 1);
    }

    #[test]
    fn immediate_mode() {
        let outputs = run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &vec![0],
        );
        assert_eq!(outputs[0], 0);

        let outputs_2 = run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &vec![10],
        );
        assert_eq!(outputs_2[0], 1);
    }
}

#[cfg(test)]
mod complex {
    use super::run_program;

    #[test]
    fn test_1() {
        let outputs = run_program(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![0],
        );
        assert_eq!(outputs[0], 999);
    }

    #[test]
    fn test_2() {
        let outputs = run_program(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![8],
        );
        assert_eq!(outputs[0], 1000);
    }

    #[test]
    fn test_3() {
        let outputs = run_program(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![80],
        );
        assert_eq!(outputs[0], 1001);
    }
}

#[cfg(test)]
mod large_numbers {
    use super::*;

    #[test]
    fn output_only() {
        assert_eq!(run_program(&[104, 1125899906842624, 99], &[]), [1125899906842624]);
    }

    #[test]
    fn math_and_output() {
        assert_eq!(run_program(&[1102,34915192,34915192,7,4,7,99,0], &[]), [34915192 * 34915192]);
    }
}
