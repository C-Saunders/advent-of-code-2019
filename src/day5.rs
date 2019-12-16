use crate::intcode_computer::run_program;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day5)]
pub fn get_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|i| i.parse::<i32>()).collect()
}

#[aoc(day5, part1)]
pub fn part1(initial_program: &[i32]) -> i32 {
    let mut program = Vec::new();
    program.resize(initial_program.len(), 0);
    program.copy_from_slice(initial_program);

    let mut outputs = Vec::with_capacity(1);
    run_program(&mut program, vec![1], &mut outputs);
    outputs[outputs.len() - 1]
}

#[aoc(day5, part2)]
pub fn part2(initial_program: &[i32]) -> i32 {
    let mut program = Vec::new();
    program.resize(initial_program.len(), 0);
    program.copy_from_slice(initial_program);

    let mut outputs = Vec::with_capacity(1);
    run_program(&mut program, vec![5], &mut outputs);
    outputs[0]
}
