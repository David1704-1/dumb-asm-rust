use crate::{cpu::CPU, instruction::Operation, memory::Memory};

pub fn spawn_vm(operations: Vec<Operation>) -> (CPU, Memory) {
    let cpu = CPU::new();
    let memory = Memory {
        values: vec![],
        operations,
    };
    (cpu, memory)
}
