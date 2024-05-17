use crate::{instruction::Operation, lexer::parse_line, vm::spawn_vm};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CPUState {
    operation: String,
    accumulator: i32,
    reminder: i32,
    bool_flag: bool,
}

#[derive(Serialize, Deserialize)]
struct ExecuteResponse {
    cpu: Vec<CPUState>,
    memory: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteRequest {
    input: String,
}

pub async fn execute(Json(payload): Json<ExecuteRequest>) -> impl IntoResponse {
    let mut operations: Vec<Operation> = vec![];
    for line in payload.input.split('\n') {
        let value = parse_line(line);
        operations.push(value);
    }
    let (mut cpu, mut memory) = spawn_vm(operations);
    let mut cpu_state: Vec<CPUState> = vec![];
    while cpu.program_counter < memory.operations.len() || !cpu.instruction_register.is_none() {
        cpu.fetch(&memory);
        cpu.execute(&mut memory);
        cpu_state.push(CPUState {
            accumulator: cpu.accumulator,
            operation: String::from(cpu.instruction_register.unwrap()),
            reminder: cpu.reminder,
            bool_flag: cpu.bool_flag,
        });
    }
    (
        StatusCode::OK,
        Json(ExecuteResponse {
            cpu: cpu_state,
            memory: memory.values,
        }),
    )
}
