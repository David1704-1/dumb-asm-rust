use crate::instruction::{Instruction, Operation};

pub fn parse_line(line: String) -> Option<Operation> {
    let split: Vec<&str> = line.split(" ").collect();
    let value = if split.len() == 1 {
        0
    } else {
        String::from(split[1]).parse::<i32>().unwrap_or(0)
    };
    match split[0] {
        "LOAD" => Some(Operation {
            operation: (Instruction::LOAD, 0),
        }),
        "ADD" => Some(Operation {
            operation: (Instruction::ADD, value),
        }),
        "STORE" => Some(Operation {
            operation: (Instruction::STORE, 0),
        }),
        "PUT" => Some(Operation {
            operation: (Instruction::PUT, value),
        }),
        "END" => Some(Operation {
            operation: (Instruction::END, 0),
        }),
        "SUB" => Some(Operation {
            operation: (Instruction::SUB, value),
        }),
        "MUL" => Some(Operation {
            operation: (Instruction::MUL, value),
        }),
        "DIV" => Some(Operation {
            operation: (Instruction::DIV, value),
        }),
        "JMP" => Some(Operation {
            operation: (Instruction::JMP, value),
        }),
        "JMPF" => Some(Operation {
            operation: (Instruction::JMPF, value),
        }),
        "JMPB" => Some(Operation {
            operation: (Instruction::JMPB, value),
        }),
        _ => None,
    }
}
