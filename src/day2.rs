use crate::intcode_computer::run_program_no_io;
use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day2)]
pub fn get_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|l| l.parse::<i32>()).collect()
}

#[aoc(day2, part1)]
pub fn part1(program_input: &[i32]) -> i32 {
    let mut program = Vec::new();
    program.resize(program_input.len(), 0);
    program.copy_from_slice(program_input);

    program[1] = 12;
    program[2] = 2;

    run_program_no_io(&mut program)[0]
}

#[aoc(day2, part2)]
pub fn part2(program_input: &[i32]) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = Vec::new();
            program.resize(program_input.len(), 0);
            program.copy_from_slice(program_input);

            program[1] = noun;
            program[2] = verb;

            let result = run_program_no_io(&mut program)[0];
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        return "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,19,5,23,2,13,23,27,1,10,27,31,2,6,31,35,1,9,35,39,2,10,39,43,1,43,9,47,1,47,9,51,2,10,51,55,1,55,9,59,1,59,5,63,1,63,6,67,2,6,67,71,2,10,71,75,1,75,5,79,1,9,79,83,2,83,10,87,1,87,6,91,1,13,91,95,2,10,95,99,1,99,6,103,2,13,103,107,1,107,2,111,1,111,9,0,99,2,14,0,0";
    }

    #[test]
    fn integration_part1() {
        assert_eq!(part1(&get_program(&get_input()).unwrap()), 2692315);
    }

    #[test]
    fn integration_part2() {
        assert_eq!(part2(&get_program(&get_input()).unwrap()), 9507);
    }
}
