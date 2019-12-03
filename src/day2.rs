use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day2)]
pub fn get_program(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.split(',').map(|l| l.parse::<usize>()).collect()
}

#[aoc(day2, part1)]
pub fn part1(program_input: &[usize]) -> usize {
    let mut program = Vec::new();
    program.resize(program_input.len(), 0);
    program.copy_from_slice(program_input);

    program[1] = 12;
    program[2] = 2;

    run_program(&mut program)
}

#[aoc(day2, part2)]
pub fn part2(program_input: &[usize]) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = Vec::new();
            program.resize(program_input.len(), 0);
            program.copy_from_slice(program_input);

            program[1] = noun;
            program[2] = verb;

            let result = run_program(&mut program);
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn run_program(program: &mut [usize]) -> usize {
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let first_parameter_index = program[instruction_pointer + 1];
        let second_operand_index = program[instruction_pointer + 2];
        let output_index = program[instruction_pointer + 3];

        match opcode {
            1 => {
                program[output_index] =
                    program[first_parameter_index] + program[second_operand_index];
            }
            2 => {
                program[output_index] =
                    program[first_parameter_index] * program[second_operand_index];
            }
            99 => {
                break;
            }
            _ => panic!("Bad opcode found"),
        };
        instruction_pointer += 4;
    }

    program[0]
}
