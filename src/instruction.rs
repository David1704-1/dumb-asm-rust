use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Instruction {
    LOAD,
    ADD,
    STORE,
    PUT,
    END,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    JEQ,
    JNEQ,
    INV,
    AND,
    OR,
    XOR,
    LSHIFT,
    RSHIFT,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Operation {
    pub operation: (Instruction, i32),
}

impl From<Operation> for String {
    fn from(value: Operation) -> Self {
        format!("{:?}: {}", value.operation.0, value.operation.1)
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "LOAD" => Instruction::LOAD,
            "ADD" => Instruction::ADD,
            "STORE" => Instruction::STORE,
            "PUT" => Instruction::PUT,
            "END" => Instruction::END,
            "SUB" => Instruction::SUB,
            "MUL" => Instruction::MUL,
            "DIV" => Instruction::DIV,
            "JMP" => Instruction::JMP,
            "JMPF" => Instruction::JMPF,
            "JMPB" => Instruction::JMPB,
            "EQ" => Instruction::EQ,
            "NEQ" => Instruction::NEQ,
            "JEQ" => Instruction::JEQ,
            "JNEQ" => Instruction::JNEQ,
            "AND" => Instruction::AND,
            "OR" => Instruction::OR,
            "XOR" => Instruction::XOR,
            "LSHIFT" => Instruction::LSHIFT,
            "RSHIFT" => Instruction::RSHIFT,
            _ => Instruction::INV,
        }
    }
}
