use crate::instruction::{Instruction, Operation};

pub fn parse_line(line: &str) -> Operation {
    let split: Vec<&str> = line.split(" ").collect();
    let value = if split.len() == 1 {
        0
    } else {
        String::from(split[1]).parse::<i32>().unwrap_or(0)
    };
    return Operation {
        operation: (Instruction::from(split[0]), value),
    };
}
