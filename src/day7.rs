use crate::intcode_computer::run_program;
use aoc_runner_derive::{aoc, aoc_generator};
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
    let mut program = Vec::new();
    program.resize(program_input.len(), 0);

    let mut res = MaxResult::new();

    for a_phase in 0..=4 {
        program.copy_from_slice(program_input);
        let mut a_outputs = Vec::with_capacity(1);
        run_program(&mut program, vec![a_phase, 0], &mut a_outputs);

        for b_phase in (0..=4).filter(|b| *b != a_phase) {
            program.copy_from_slice(program_input);
            let mut b_outputs = Vec::with_capacity(1);
            run_program(&mut program, vec![b_phase, a_outputs[0]], &mut b_outputs);

            for c_phase in (0..=4).filter(|c| *c != a_phase && *c != b_phase) {
                program.copy_from_slice(program_input);
                let mut c_outputs = Vec::with_capacity(1);
                run_program(&mut program, vec![c_phase, b_outputs[0]], &mut c_outputs);

                for d_phase in (0..=4).filter(|d| *d != a_phase && *d != b_phase && *d != c_phase) {
                    program.copy_from_slice(program_input);
                    let mut d_outputs = Vec::with_capacity(1);
                    run_program(&mut program, vec![d_phase, c_outputs[0]], &mut d_outputs);

                    for e_phase in (0..=4).filter(|e| {
                        *e != a_phase && *e != b_phase && *e != c_phase && *e != d_phase
                    }) {
                        program.copy_from_slice(program_input);
                        let mut e_outputs = Vec::with_capacity(1);
                        run_program(&mut program, vec![e_phase, d_outputs[0]], &mut e_outputs);

                        if e_outputs[0] > res.thruster_output {
                            res.thruster_output = e_outputs[0];
                            res.a_phase = Some(a_phase);
                            res.b_phase = Some(b_phase);
                            res.c_phase = Some(c_phase);
                            res.d_phase = Some(d_phase);
                            res.e_phase = Some(e_phase);
                        }
                    }
                }
            }
        }
    }
    res
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
