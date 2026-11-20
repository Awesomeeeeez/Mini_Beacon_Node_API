use std::format;
use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub slot: u64,
    pub parent_root: String,   // hex-строка "0x..."
    pub proposer_index: u64,
    pub body: String,          // любая полезная нагрузка
}

impl Block {
    pub fn new() -> Self {
        Self {
            slot: 0,
            parent_root: format!("0x{}", "0".repeat(64)),
            proposer_index: 0,
            body: "hello".to_string(),
        }
    }
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
        match self {
            ApiError::InvalidFormat => (
                StatusCode::NOT_ACCEPTABLE,
                "Ошибка формата",
            )
                .into_response(),

            ApiError::ParentUnknown => (
                StatusCode::NOT_ACCEPTABLE,
                "Хэш последнего блока не верный",
            )
                .into_response(),

            ApiError::OrderDisrupted => (
                StatusCode::NOT_ACCEPTABLE,
                "нарушен порядок",
            )
                .into_response(),

            ApiError::BadData => (
                StatusCode::NOT_ACCEPTABLE,
                "Плохие данные",
            )
                .into_response(),
            
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "Запрашиваемый блок не найден",
            )
                .into_response(),
        }
    }
}