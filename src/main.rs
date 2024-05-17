use axum::routing::post;
use axum::Router;
use route_handlers::execute;

mod cpu;
mod instruction;
mod lexer;
mod memory;
mod route_handlers;
mod vm;

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
