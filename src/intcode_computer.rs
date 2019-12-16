use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    StdIn,
    StdOut,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(opcode: &str) -> Result<Self, Self::Err> {
        match opcode {
            "01" => Ok(Opcode::Add),
            "02" => Ok(Opcode::Multiply),
            "03" => Ok(Opcode::StdIn),
            "04" => Ok(Opcode::StdOut),
            "05" => Ok(Opcode::JumpIfTrue),
            "06" => Ok(Opcode::JumpIfFalse),
            "07" => Ok(Opcode::LessThan),
            "08" => Ok(Opcode::Equals),
            "99" => Ok(Opcode::End),
            _ => Err(()),
        }
    }
}

pub fn run_program_no_stdio(mut program: &mut [i32]) -> Vec<i32> {
    run_program(&mut program, vec![], &mut vec![])
}

pub fn run_program(program: &mut [i32], inputs: Vec<i32>, outputs: &mut Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer: usize = 0;
    let mut inputs_iter = inputs.iter();

    while instruction_pointer < program.len() {
        let padded_instruction = format!("{:0>2}", program[instruction_pointer]);
        let opcode =
            Opcode::from_str(&padded_instruction[(padded_instruction.len() - 2)..]).unwrap();

        if opcode == Opcode::End {
            break;
        }

        match opcode {
            // No inputs
            Opcode::StdIn | Opcode::StdOut => {
                let output_index = program[instruction_pointer + 1] as usize;
                match opcode {
                    Opcode::StdIn => {
                        program[output_index] = *inputs_iter.next().unwrap();
                    }
                    Opcode::StdOut => {
                        // TODO: Band-aid hack job
                        if padded_instruction == "104" {
                            outputs.push(output_index as i32);
                        } else {
                            outputs.push(program[output_index]);
                        }
                    }
                    _ => {}
                }

                instruction_pointer += 2;
            }
            // Two inputs
            _ => {
                let parameter_modes = format!(
                    "{:0>2}",
                    &padded_instruction[..(padded_instruction.len() - 2)]
                );

                let parameter_values: Vec<i32> = parameter_modes
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(index, op_mode)| match op_mode {
                        '0' => program[program[instruction_pointer + index + 1] as usize],
                        '1' => program[instruction_pointer + index + 1],
                        _ => panic!("Bad"),
                    })
                    .collect();

                match opcode {
                    Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => {
                        let output_index = program[instruction_pointer + 3] as usize;
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
                        instruction_pointer += 4;
                    }
                    Opcode::JumpIfTrue => {
                        if parameter_values[0] != 0 {
                            instruction_pointer = parameter_values[1] as usize;
                        } else {
                            instruction_pointer += 3;
                        }
                    }
                    Opcode::JumpIfFalse => {
                        if parameter_values[0] == 0 {
                            instruction_pointer = parameter_values[1] as usize;
                        } else {
                            instruction_pointer += 3;
                        }
                    }
                    _ => {}
                }
            }
        };
    }

    program.to_vec()
}

#[cfg(test)]
mod no_stdio {
    use super::run_program_no_stdio;

    #[test]
    fn simple_examples() {
        assert_eq!(
            run_program_no_stdio(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[0],
            3500
        );
        assert_eq!(run_program_no_stdio(&mut [1, 0, 0, 0, 99])[0], 2);
        assert_eq!(
            run_program_no_stdio(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99])[0],
            30
        );
    }

    #[test]
    fn negative_immediate() {
        assert_eq!(run_program_no_stdio(&mut [1101, 100, -1, 4, 0])[4], 99)
    }
}

#[cfg(test)]
mod equality {
    use super::run_program;

    #[test]
    fn equality_position_mode_true() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![8],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_position_mode_false() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![-8],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn equality_immediate_mode_true() {
        let mut outputs = Vec::with_capacity(1);
        run_program(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8], &mut outputs);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_immediate_mode_false() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![-8],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod less_than {
    use super::run_program;

    #[test]
    fn less_than_position_mode_true() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![8 - 1],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_position_mode_false() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![8 + 1],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_position_mode_false_equal() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![8],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_true() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 3, 1107, -1, 8, 3, 4, 3, 99],
            vec![8 - 1],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_immediate_mode_false() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 3, 1107, -1, 8, 3, 4, 3, 99],
            vec![8 + 1],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_false_equal() {
        let mut outputs = Vec::with_capacity(1);
        run_program(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![8], &mut outputs);
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod jumps {
    use super::run_program;

    #[test]
    fn position_mode() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);

        outputs.clear();
        run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![10],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn immediate_mode() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
            &mut outputs,
        );
        assert_eq!(outputs[0], 0);

        outputs.clear();
        run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![10],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1);
    }
}

#[cfg(test)]
mod complex {
    use super::run_program;

    #[test]
    fn test_1() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![0],
            &mut outputs,
        );
        assert_eq!(outputs[0], 999);
    }

    #[test]
    fn test_2() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![8],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1000);
    }

    #[test]
    fn test_3() {
        let mut outputs = Vec::with_capacity(1);
        run_program(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![80],
            &mut outputs,
        );
        assert_eq!(outputs[0], 1001);
    }
}
