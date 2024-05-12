use crate::{
    instruction::{Instruction, Operation},
    memory::Memory,
};

#[derive(Debug)]
pub struct CPU {
    pub program_counter: usize,
    pub instruction_register: Option<Operation>,
    pub accumulator: i32,
    pub reminder: i32,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: 0,
            instruction_register: None,
            accumulator: 0,
            reminder: 0,
        }
    }
    pub fn fetch(&mut self, memory: &Memory) {
        self.instruction_register = Some(memory.operations[self.program_counter]);
    }
    pub fn execute(&mut self, memory: &mut Memory) {
        if let Some(operation) = self.instruction_register {
            match operation.operation.0 {
                Instruction::LOAD => {
                    if let Some(value) = memory.values.pop() {
                        self.accumulator = value;
                    }
                }
                Instruction::ADD => {
                    self.accumulator += operation.operation.1;
                }
                Instruction::STORE => {
                    memory.values.push(self.accumulator);
                    self.accumulator = 0;
                }
                Instruction::PUT => {
                    memory.values.push(operation.operation.1);
                }
                Instruction::END => {
                    self.program_counter = 0;
                    self.instruction_register = None;
                }
                Instruction::SUB => {
                    self.accumulator -= operation.operation.1;
                }
                Instruction::MUL => {
                    self.accumulator *= operation.operation.1;
                }
                Instruction::DIV => {
                    if operation.operation.1 != 0 {
                        self.accumulator /= operation.operation.1;
                        self.reminder = self.accumulator % operation.operation.1;
                    }
                }
                Instruction::JMP => {
                    self.program_counter = operation.operation.1 as usize;
                    return;
                }
                Instruction::JMPF => {
                    self.program_counter += operation.operation.1 as usize;
                    return;
                }
                Instruction::JMPB => {
                    self.program_counter -= operation.operation.1 as usize;
                    return;
                }
            }
            self.program_counter += 1;
        }
    }
}
