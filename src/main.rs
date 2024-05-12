use cpu::CPU;
use instruction::Operation;
use lexer::parse_line;
use memory::Memory;
use std::env;
use std::fs::read_to_string;

mod cpu;
mod instruction;
mod lexer;
mod memory;

fn main() {
    let mut operations: Vec<Operation> = vec![];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    for line in read_to_string(filename).unwrap().lines() {
        if let Some(value) = parse_line(String::from(line)) {
            operations.push(value);
        }
    }
    let mut memory = Memory {
        values: vec![],
        operations,
    };
    let mut cpu = CPU::new();
    loop {
        cpu.fetch(&memory);
        cpu.execute(&mut memory);
        if cpu.instruction_register.is_none() {
            break;
        }
        println!("{:?}\n", cpu);
        println!("memory: {:?}\n", memory.values);
    }
}
