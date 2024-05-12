use crate::instruction::Operation;

#[derive(Debug)]
pub struct Memory {
    pub operations: Vec<Operation>,
    pub values: Vec<i32>,
}
