use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Conflict(String),
    NotFound(String),
    Internal(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Conflict(message) => (StatusCode::CONFLICT, message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Self::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        (
            status,
            Json(ErrorResponse {
                success: false,
                message,
            }),
        )
            .into_response()
    }
}

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(rejection: axum::extract::rejection::JsonRejection) -> Self {
        AppError::BadRequest(format!("Invalid JSON payload: {}", rejection.body_text()))
    }
}

impl From<axum::extract::rejection::QueryRejection> for AppError {
    fn from(rejection: axum::extract::rejection::QueryRejection) -> Self {
        AppError::BadRequest(format!(
            "Invalid query parameters: {}",
            rejection.body_text()
        ))
    }
}
