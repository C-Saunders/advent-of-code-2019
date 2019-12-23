use crate::intcode_computer::run_program;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::num::ParseIntError;

#[aoc_generator(day7)]
pub fn get_program(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split(',').map(|l| l.parse::<i32>()).collect()
}

#[derive(Debug, PartialEq)]
pub struct MaxResult {
    thruster_output: i32,
    a_phase: Option<i32>,
    b_phase: Option<i32>,
    c_phase: Option<i32>,
    d_phase: Option<i32>,
    e_phase: Option<i32>,
}

impl MaxResult {
    fn new() -> Self {
        MaxResult {
            thruster_output: std::i32::MIN,
            a_phase: None,
            b_phase: None,
            c_phase: None,
            d_phase: None,
            e_phase: None,
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(program_input: &[i32]) -> i32 {
    get_max_result(&program_input).thruster_output
}

fn get_max_result(program_input: &[i32]) -> MaxResult {
    let program = Vec::from(program_input);

    let mut res = MaxResult::new();

    for phases in (0..=4).permutations(5) {
        let a_phase = phases[0];
        let b_phase = phases[1];
        let c_phase = phases[2];
        let d_phase = phases[3];
        let e_phase = phases[4];

        let a_outputs = run_program(&program, &vec![a_phase, 0]);
        let b_outputs = run_program(&program, &vec![b_phase, a_outputs[0]]);
        let c_outputs = run_program(&program, &vec![c_phase, b_outputs[0]]);
        let d_outputs = run_program(&program, &vec![d_phase, c_outputs[0]]);
        let e_outputs = run_program(&program, &vec![e_phase, d_outputs[0]]);

        if e_outputs[0] > res.thruster_output {
            res.thruster_output = e_outputs[0];
            res.a_phase = Some(a_phase);
            res.b_phase = Some(b_phase);
            res.c_phase = Some(c_phase);
            res.d_phase = Some(d_phase);
            res.e_phase = Some(e_phase);
        }
    }
    res
}

#[aoc(day7, part2)]
pub fn part2(program: &[i32]) -> i32 {
    // let mut res = MaxResult::new();

    // for phases in (5..=9).permutations(5) {
    //     let a_phase = phases[0];
    //     let b_phase = phases[1];
    //     let c_phase = phases[2];
    //     let d_phase = phases[3];
    //     let e_phase = phases[4];
    // }

    // res.thruster_output
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example_1() {
        assert_eq!(
            get_max_result(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
            MaxResult {
                thruster_output: 43210,
                a_phase: Some(4),
                b_phase: Some(3),
                c_phase: Some(2),
                d_phase: Some(1),
                e_phase: Some(0),
            }
        );
    }

    #[test]
    fn given_example_2() {
        assert_eq!(
            get_max_result(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            MaxResult {
                thruster_output: 54321,
                a_phase: Some(0),
                b_phase: Some(1),
                c_phase: Some(2),
                d_phase: Some(3),
                e_phase: Some(4),
            }
        );
    }

    #[test]
    fn given_example_3() {
        assert_eq!(
            get_max_result(&[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            MaxResult {
                thruster_output: 65210,
                a_phase: Some(1),
                b_phase: Some(0),
                c_phase: Some(4),
                d_phase: Some(3),
                e_phase: Some(2),
            }
        );
    }
}
