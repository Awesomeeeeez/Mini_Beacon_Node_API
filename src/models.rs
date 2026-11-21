use std::format;
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub slot: u64,
    pub root: String,
    pub parent_root: String,   // hex-строка "0x..."
    pub proposer_index: u64,
    pub body: String,          // любая полезная нагрузка
}

impl Block {
    pub fn new() -> Self {
        Self {
            slot: 0,
            root: format!("0x{}", "0".repeat(64)),
            parent_root: format!("0x{}", "0".repeat(64)),
            proposer_index: 0,
            body: "hello".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct NewBlock {
    pub slot: u64,
    pub parent_root: String,   // hex-строка "0x..."
    pub proposer_index: u64,
    pub body: String,          // любая полезная нагрузка
}

#[derive(Serialize)]
pub struct Health {
    pub status: String,
    pub blocks_count: usize,
}

#[derive(Serialize)]
pub struct RootResponse {
    pub root: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub from_slot: Option<usize>,
    pub limit: Option<usize>,
}

pub enum ApiError {
    InvalidFormat,
    ParentUnknown,
    OrderDisrupted,
    BadData,
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            ApiError::InvalidFormat => (
                StatusCode::NOT_ACCEPTABLE,
                "invalid_format",
                "Некорректный формат parent_root",
            ),

            ApiError::ParentUnknown => (
                StatusCode::NOT_ACCEPTABLE,
                "parent_unknown",
                "Родительский блок не найден",
            ),

            ApiError::OrderDisrupted => (
                StatusCode::NOT_ACCEPTABLE,
                "order_disrupted",
                "Нарушен порядок блоков",
            ),

            ApiError::BadData => (
                StatusCode::NOT_ACCEPTABLE,
                "bad_data",
                "Некорректные данные блока",
            ),

            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "Запрашиваемый блок не найден",
            ),
        };

        (
            status,
            axum::Json(
                ErrorResponse{
                    error,
                    message,
                }
            ),
        )
            .into_response()

    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: &'static str,
}