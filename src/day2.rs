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

    let mut index = 0;

    while index < program.len() {
        let opcode = program[index];
        let first_operand_index = program[index + 1];
        let second_operand_index = program[index + 2];
        let output_index = program[index + 3];

        match opcode {
            1 => {
                program[output_index] =
                    program[first_operand_index] + program[second_operand_index];
            }
            2 => {
                program[output_index] =
                    program[first_operand_index] * program[second_operand_index];
            }
            99 => {
                break;
            }
            _ => panic!("Bad opcode found"),
        };
        index += 4;
    }

    program[0]
}
