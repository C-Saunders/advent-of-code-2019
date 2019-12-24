use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn get_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|i| i.parse::<i64>()).collect()
}

#[aoc(day5, part1)]
pub fn part1(program: &[i64]) -> i64 {
    let outputs = run_program(&program, &vec![1]);
    *outputs.last().unwrap()
}
