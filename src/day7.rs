use crate::intcode_computer::{run_program, IntcodeComputer, ProgramOutput};
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
pub fn part2(program_input: &[i32]) -> i32 {
    get_max_result_with_feedback_loop(&program_input).thruster_output
}

fn get_max_result_with_feedback_loop(program_input: &[i32]) -> MaxResult {
    let program = Vec::from(program_input);
    let mut res = MaxResult::new();

    for phases in (5..=9).permutations(5) {
        let a_phase = phases[0];
        let b_phase = phases[1];
        let c_phase = phases[2];
        let d_phase = phases[3];
        let e_phase = phases[4];

        let mut a_computer = IntcodeComputer::yielding_computer(&program);
        a_computer.add_input(a_phase);
        let mut b_computer = IntcodeComputer::yielding_computer(&program);
        b_computer.add_input(b_phase);
        let mut c_computer = IntcodeComputer::yielding_computer(&program);
        c_computer.add_input(c_phase);
        let mut d_computer = IntcodeComputer::yielding_computer(&program);
        d_computer.add_input(d_phase);
        let mut e_computer = IntcodeComputer::yielding_computer(&program);
        e_computer.add_input(e_phase);

        a_computer.add_input(0);
        let mut a_result = extract_output(&(a_computer.run_program()));

        loop {
            b_computer.add_input(a_result);
            let b_result = extract_output(&(b_computer.run_program()));
            c_computer.add_input(b_result);
            let c_result = extract_output(&(c_computer.run_program()));
            d_computer.add_input(c_result);
            let d_result = extract_output(&(d_computer.run_program()));

            e_computer.add_input(d_result);
            let e_result = e_computer.run_program();
            match e_result {
                ProgramOutput::Yielded(val) => {
                    a_computer.add_input(val);
                    a_result = extract_output(&(a_computer.run_program()));
                }
                ProgramOutput::Complete(values) => {
                    let final_output = values.last().unwrap();
                    if final_output > &res.thruster_output {
                        res.thruster_output = *final_output;
                        res.a_phase = Some(a_phase);
                        res.b_phase = Some(b_phase);
                        res.c_phase = Some(c_phase);
                        res.d_phase = Some(d_phase);
                        res.e_phase = Some(e_phase);
                    }
                    break;
                }
            };
        }
    }

    res
}

fn extract_output(output: &ProgramOutput) -> i32 {
    match output {
        ProgramOutput::Yielded(val) => *val,
        ProgramOutput::Complete(values) => *values.last().unwrap(),
    }
}

#[cfg(test)]
mod part1_tests {
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

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn given_example_1() {
        assert_eq!(
            get_max_result_with_feedback_loop(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            MaxResult {
                thruster_output: 139629729,
                a_phase: Some(9),
                b_phase: Some(8),
                c_phase: Some(7),
                d_phase: Some(6),
                e_phase: Some(5),
            }
        );
    }

    #[test]
    fn given_example_2() {
        assert_eq!(
            get_max_result_with_feedback_loop(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            MaxResult {
                thruster_output: 18216,
                a_phase: Some(9),
                b_phase: Some(7),
                c_phase: Some(8),
                d_phase: Some(5),
                e_phase: Some(6),
            }
        );
    }
}
