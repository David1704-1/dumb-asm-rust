use serde::{Deserialize, Serialize};

use crate::{
    instruction::{Instruction, Operation},
    memory::Memory,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CPU {
    pub program_counter: usize,
    pub instruction_register: Option<Operation>,
    pub accumulator: i32,
    pub reminder: i32,
    pub bool_flag: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: 0,
            instruction_register: None,
            accumulator: 0,
            reminder: 0,
            bool_flag: false,
        }
    }
    pub fn fetch(&mut self, memory: &Memory) {
        self.instruction_register = Some(memory.operations[self.program_counter]);
    }
    pub fn execute(&mut self, memory: &mut Memory) {
        if let Some(operation) = self.instruction_register {
            let instruction = &operation.operation.0;
            let instruction_value = &operation.operation.1;
            match instruction {
                Instruction::LOAD => {
                    if let Some(value) = memory.values.pop() {
                        self.accumulator = value;
                    }
                }
                Instruction::ADD => {
                    self.accumulator += instruction_value;
                }
                Instruction::STORE => {
                    memory.values.push(self.accumulator);
                    self.accumulator = 0;
                }
                Instruction::PUT => {
                    memory.values.push(instruction_value.clone());
                }
                Instruction::END => {
                    self.program_counter = 0;
                    self.instruction_register = None;
                }
                Instruction::SUB => {
                    self.accumulator -= instruction_value;
                }
                Instruction::MUL => {
                    self.accumulator *= instruction_value;
                }
                Instruction::DIV => {
                    if operation.operation.1 != 0 {
                        let tmp = self.accumulator.clone();
                        self.accumulator /= instruction_value;
                        self.reminder = tmp % instruction_value;
                    }
                }
                Instruction::JMP => {
                    self.program_counter = *instruction_value as usize;
                    return;
                }
                Instruction::JMPF => {
                    self.program_counter += *instruction_value as usize;
                    return;
                }
                Instruction::JMPB => {
                    self.program_counter -= *instruction_value as usize;
                    return;
                }
                Instruction::EQ => {
                    self.bool_flag = *instruction_value == self.accumulator;
                }
                Instruction::NEQ => {
                    self.bool_flag = *instruction_value != self.accumulator;
                }
                Instruction::JEQ => {
                    if self.bool_flag {
                        self.program_counter = *instruction_value as usize;
                        return;
                    }
                }
                Instruction::JNEQ => {
                    if !self.bool_flag {
                        self.program_counter = *instruction_value as usize;
                        return;
                    }
                }
                Instruction::INV => {}
            }
            self.program_counter += 1;
        }
    }
}
