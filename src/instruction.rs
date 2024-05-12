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
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub operation: (Instruction, i32),
}
