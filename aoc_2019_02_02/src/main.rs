use std::fs;
use std::iter::FromIterator;

fn main() {
    println!("Hello, world!");
}

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

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn initialize(memory: String) -> Program {
        let values: Vec<i32> = memory
            .trim().split(",")
            .map(|v| v.parse().expect("Value should be a number!"))
            .collect();

        let mut position = 0;
        let mut instruction_index = 0;
        let mut op_code = OpCode::from(values[position]);
        let mut instructions: Vec<Instruction> = vec![];
        
        while op_code != OpCode::Terminate {
            match op_code {
                OpCode::Add => instructions.insert(instruction_index, Instruction::create(op_code, &values[position..position + 3])),
                OpCode::Multiply => instructions.insert(instruction_index, Instruction::create(op_code, &values[position..position + 3])),
                _ => panic!("Unsupported op-code encountered."),
            }
            position += instructions[instruction_index].get_paramater_count() + 1;
            op_code = OpCode::from(values[position]);
            instruction_index += 1;
        }

        Program {
            instructions: instructions,
        }
    }
}

struct Instruction {
    code: OpCode,
    parameters: Vec<i32>,
}

impl Instruction {
    fn create(op_code: OpCode, parameters: &[i32]) -> Instruction {
        Instruction {
            code: op_code,
            parameters: Vec::from_iter(parameters.iter().cloned()),
        }
    }

    fn get_paramater_count(&self) -> usize { return self.parameters.len(); }
}
