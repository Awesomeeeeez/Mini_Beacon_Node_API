mod tasks;
mod state;
mod models;
mod routes;
mod tools;

use std::println;
use state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Сервер запущен!");

    axum::serve(listener, app).await.unwrap();
}