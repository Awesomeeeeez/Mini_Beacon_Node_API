use axum::{Router, routing::{get, post}};
use crate::{state::AppState, tasks};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(tasks::health))
        .route("/blocks/{root}", get(tasks::get_one))
        .route("/head", get(tasks::head))
        .route("/blocks", post(tasks::new_block))
        .route("/blocks", get(tasks::get_blocks))
        .with_state(state)
}