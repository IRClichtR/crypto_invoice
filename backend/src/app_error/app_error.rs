// use thiserror::Error;
use std::fmt;

use axum::response::{IntoResponse, Response};
use hyper::http::StatusCode;
// use std::io;


#[derive(Debug)]
pub enum AppError {
    ConfigError(String),
    DatabaseError(String),
    ServerError(String),
    SignalError(String),
    OtherError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::ServerError(msg) => write!(f, "Server Error: {}", msg),
            AppError::SignalError(msg) => write!(f, "Signal Error: {}", msg),
            AppError::OtherError(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ConfigError(_) => None,
            AppError::DatabaseError(_) => None,
            AppError::ServerError(_) => None,
            AppError::SignalError(_) => None,
            AppError::OtherError(_) => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ConfigError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            AppError::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            AppError::SignalError(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg).into_response(),
            AppError::OtherError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}