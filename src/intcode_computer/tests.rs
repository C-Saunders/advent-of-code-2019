use super::{run_program, run_program_no_io, IntcodeComputer, ProgramOutput};

#[cfg(test)]
mod no_io {
    use super::run_program_no_io;

    #[test]
    fn simple_examples() {
        assert_eq!(
            run_program_no_io(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[0],
            3500
        );
        assert_eq!(run_program_no_io(&[1, 0, 0, 0, 99])[0], 2);
        assert_eq!(run_program_no_io(&[1, 1, 1, 4, 99, 5, 6, 0, 99])[0], 30);
    }

    #[test]
    fn negative_immediate() {
        assert_eq!(run_program_no_io(&[1101, 100, -1, 4, 0])[4], 99)
    }
}

#[cfg(test)]
mod equality {
    use super::run_program;

    #[test]
    fn equality_position_mode_true() {
        let outputs = run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_position_mode_false() {
        let outputs = run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &vec![-8]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn equality_immediate_mode_true() {
        let outputs = run_program(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &vec![8]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn equality_immediate_mode_false() {
        let outputs = run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &vec![-8]);
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod less_than {
    use super::run_program;

    #[test]
    fn less_than_position_mode_true() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8 - 1]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_position_mode_false() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8 + 1]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_position_mode_false_equal() {
        let outputs = run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_true() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8 - 1]);
        assert_eq!(outputs[0], 1);
    }

    #[test]
    fn less_than_immediate_mode_false() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8 + 1]);
        assert_eq!(outputs[0], 0);
    }

    #[test]
    fn less_than_immediate_mode_false_equal() {
        let outputs = run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8]);
        assert_eq!(outputs[0], 0);
    }
}

#[cfg(test)]
mod jumps {
    use super::run_program;

    #[test]
    fn position_mode() {
        let outputs = run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &vec![0],
        );
        assert_eq!(outputs[0], 0);

        let outputs_2 = run_program(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &vec![10],
        );
        assert_eq!(outputs_2[0], 1);
    }

    #[test]
    fn immediate_mode() {
        let outputs = run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &vec![0],
        );
        assert_eq!(outputs[0], 0);

        let outputs_2 = run_program(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &vec![10],
        );
        assert_eq!(outputs_2[0], 1);
    }
}

#[cfg(test)]
mod complex {
    use super::run_program;

    #[test]
    fn test_1() {
        let outputs = run_program(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![0],
        );
        assert_eq!(outputs[0], 999);
    }

    #[test]
    fn test_2() {
        let outputs = run_program(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![8],
        );
        assert_eq!(outputs[0], 1000);
    }

    #[test]
    fn test_3() {
        let outputs = run_program(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &vec![80],
        );
        assert_eq!(outputs[0], 1001);
    }
}

#[cfg(test)]
mod large_numbers {
    use super::*;

    #[test]
    fn output_only() {
        assert_eq!(
            run_program(&[104, 1125899906842624, 99], &[]),
            [1125899906842624]
        );
    }

    #[test]
    fn math_and_output() {
        assert_eq!(
            run_program(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0], &[]),
            [34915192 * 34915192]
        );
    }
}

#[cfg(test)]
mod relative_mode_and_extra_memory {
    use super::*;

    #[test]
    fn relative_mode() {
        let input = [109, 1, 204, -1, 99];
        let mut comp = IntcodeComputer::new(&input, &[]);
        match comp.run_program() {
            ProgramOutput::Yielded(_) => panic!("Should not yield"),
            ProgramOutput::Complete(output) => assert_eq!(output, [109]),
        }
    }

    #[test]
    #[ignore]
    fn quine() {
        let input = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut comp = IntcodeComputer::new(&input, &[]);
        match comp.run_program() {
            ProgramOutput::Yielded(_) => panic!("Should not yield"),
            ProgramOutput::Complete(output) => assert_eq!(output, input),
        }
    }
}
