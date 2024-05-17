use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use cpu::CPU;
use instruction::Operation;
use lexer::parse_line;
use memory::Memory;
use serde::{Deserialize, Serialize};

mod cpu;
mod instruction;
mod lexer;
mod memory;

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
struct ExecuteRequest {
    input: String,
}

async fn execute(Json(payload): Json<ExecuteRequest>) -> impl IntoResponse {
    let mut operations: Vec<Operation> = vec![];
    for line in payload.input.split('\n') {
        let value = parse_line(String::from(line));
        operations.push(value);
    }
    let mut cpu = CPU::new();
    let mut cpu_state: Vec<CPUState> = vec![];
    let mut memory = Memory {
        values: vec![],
        operations,
    };
    while cpu.program_counter < memory.operations.len() {
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/execute", post(execute))
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 3000))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
