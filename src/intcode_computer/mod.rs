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
    Halt,
}

impl From<i64> for Opcode {
    fn from(opcode: i64) -> Self {
        match opcode {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::ChangeRelativeBase,
            99 => Opcode::Halt,
            _ => panic!("Bad op code"),
        }
    }
}

impl Opcode {
    fn num_params(&self) -> usize {
        match self {
            Opcode::Add
            | Opcode::Multiply
            | Opcode::JumpIfTrue
            | Opcode::JumpIfFalse
            | Opcode::LessThan
            | Opcode::Equals => 2,
            Opcode::Input | Opcode::Output | Opcode::ChangeRelativeBase => 1,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Pointer,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(opcode: i64) -> Self {
        match opcode {
            0 => ParameterMode::Pointer,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Bad param mode"),
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
    relative_base: i64,
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
        let relative_base = &mut self.relative_base;

        let mut inputs_iter = inputs.iter().skip(*next_input_index);

        while instruction_pointer < program_len {
            let instruction = program[*instruction_pointer as usize];

            let opcode = Opcode::from(instruction % 100);

            if opcode == Opcode::Halt {
                break;
            }

            let parameter_modes = [
                ParameterMode::from((instruction / 100) % 10),
                ParameterMode::from((instruction / 1000) % 10),
                ParameterMode::from((instruction / 10000) % 10),
            ];

            let parameter_values: Vec<i64> = (0..opcode.num_params())
                .map(|index| {
                    let value = program[*instruction_pointer + index + 1];
                    match parameter_modes[index] {
                        ParameterMode::Pointer => program[value as usize],
                        ParameterMode::Immediate => value,
                        ParameterMode::Relative => program[(value + *relative_base) as usize],
                    }
                })
                .collect();

            match opcode {
                Opcode::Halt => break,
                Opcode::Input => {
                    let value = program[*instruction_pointer + 1];

                    // TODO: this needs to only add relative base in relative mode
                    program[(value + *relative_base) as usize] = *inputs_iter.next().unwrap();
                    *next_input_index += 1;
                    *instruction_pointer += 2;
                }
                Opcode::Output => {
                    let output_value = parameter_values[0];
                    outputs.push(output_value);

                    *instruction_pointer += 2;

                    if self.yield_on_output {
                        return ProgramOutput::Yielded(output_value);
                    }
                }
                Opcode::ChangeRelativeBase => {
                    *relative_base += parameter_values[0];
                    *instruction_pointer += 2;
                }
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
mod tests;
