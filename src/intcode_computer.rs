extern crate rprompt;

pub fn run_program(program: &mut [i32]) -> Vec<i32> {
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < program.len() {
        let padded_instruction = format!("{:0>2}", program[instruction_pointer]);
        let opcode = &padded_instruction[(padded_instruction.len() - 2)..];
        let parameter_modes = &padded_instruction[..(padded_instruction.len() - 2)];

        let parameter_values: Vec<i32> = parameter_modes
            .chars()
            .rev()
            .enumerate()
            .map(|(index, op_mode)| match op_mode {
                '0' => program[program[instruction_pointer + index + 1] as usize],
                '1' => program[instruction_pointer + index + 1],
                _ => panic!("Bad"),
            })
            .collect();

        match opcode {
            "01" | "02" => {
                let first_param = parameter_values
                    .get(0)
                    .unwrap_or_else(|| &program[program[instruction_pointer + 1] as usize]);
                let second_param = parameter_values
                    .get(1)
                    .unwrap_or_else(|| &program[program[instruction_pointer + 2] as usize]);
                let output_index = program[instruction_pointer + 3] as usize;

                program[output_index] = match opcode {
                    "01" => first_param + second_param,
                    "02" => first_param * second_param,
                    _ => panic!("More bad"),
                };
                instruction_pointer += 4;
            }
            "03" => {
                let input = rprompt::prompt_reply_stdout("Input: ").unwrap();
                let output_index = program[instruction_pointer + 1] as usize;

                program[output_index] = input.parse::<i32>().unwrap();

                instruction_pointer += 2;
            }
            "04" => {
                let output_index = program[instruction_pointer + 1] as usize;
                println!("Output: {}", program[output_index]);

                instruction_pointer += 2;
            }
            "99" => {
                break;
            }
            _ => panic!("Bad opcode found"),
        };
    }

    program.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            run_program(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50])[0],
            3500
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(run_program(&mut [1, 0, 0, 0, 99])[0], 2)
    }

    #[test]
    fn example_3() {
        assert_eq!(run_program(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99])[0], 30)
    }

    #[test]
    fn example_4() {
        assert_eq!(run_program(&mut [1101, 100, -1, 4, 0])[4], 99)
    }
}
