use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day11)]
pub fn get_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|l| l.parse::<i64>()).collect()
}

#[aoc(day11, part1)]
pub fn part1(program_input: &[i64]) {}
