use crate::intcode_computer::run_program;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day5)]
pub fn get_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|i| i.parse::<i64>()).collect()
}

#[aoc(day5, part1)]
pub fn part1(program: &[i64]) -> i64 {
    let outputs = run_program(&program, &vec![1]);
    *outputs.last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(program: &[i64]) -> i64 {
    *run_program(&program, &vec![5]).first().unwrap()
}
