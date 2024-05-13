#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub operation: (Instruction, i32),
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
            _ => Instruction::INV,
        }
    }
}
