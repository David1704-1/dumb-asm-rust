use crate::instruction::Operation;

#[derive(Debug)]
pub struct Memory {
    pub operations: Vec<Operation>,
    pub values: Vec<i32>,
}

impl Memory {
    pub fn new(operations: Vec<Operation>) -> Memory {
        Memory {
            operations,
            values: vec![],
        }
    }
}
