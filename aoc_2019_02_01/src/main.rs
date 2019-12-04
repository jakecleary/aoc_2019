use std::fs;

#[derive(PartialEq)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Terminate = 99,
}

impl OpCode {
    fn from(value: i32) -> OpCode {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Terminate,
            _ => panic!("Unknown op-code: {}", value),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<i32> = input
        .trim()
        .split(",")
        .map(|v| v.parse().expect("Value should be a number!"))
        .collect();
    let result = run_program(input);

    println!("{:?}", result);
}

fn run_program(mut input: Vec<i32>) -> Vec<i32> {
    let mut position = 0;
    let mut instruction = OpCode::from(input[position]);

    while instruction != OpCode::Terminate {
        let input_1 = input[input[position + 1] as usize];
        let input_2 = input[input[position + 2] as usize];
        let output_position = input[position + 3] as usize;
        match instruction {
            OpCode::Add => input[output_position] = input_1 + input_2,
            OpCode::Multiply => input[output_position] = input_1 * input_2,
            _ => panic!("Unsupported op-code encountered."),
        }
        position += 4;
        instruction = OpCode::from(input[position]);
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_99_terminates() {
        let program = vec![99];
        let answer = run_program(program);
        assert_eq!(answer, vec![99]);
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let answer = run_program(program);
        assert_eq!(answer, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let answer = run_program(program);
        assert_eq!(answer, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply_and_store_after_termination() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let answer = run_program(program);
        assert_eq!(answer, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let answer = run_program(program);
        assert_eq!(answer, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    #[should_panic(expected = "Unknown op-code: 42")]
    fn unknown_opcode_panics() {
        let program = vec![42];
        run_program(program);
    }
}
