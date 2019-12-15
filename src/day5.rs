use crate::intcode_computer::run_program;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day5)]
pub fn get_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|i| i.parse::<i32>()).collect()
}

#[aoc(day5, part1)]
pub fn part1(program_input: &[i32]) -> i32 {
    let mut program = Vec::new();
    program.resize(program_input.len(), 0);
    program.copy_from_slice(program_input);
    let result = run_program(&mut program);
    result[0]
}
