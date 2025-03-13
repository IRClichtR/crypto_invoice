use axum::{
    http::StatusCode, 
    response::{IntoResponse, Response}, 
    Json
};
use serde_json::json;
use std::fmt;
use thiserror::Error;


// Error type for the application
#[derive(Debug)]
pub enum AppError {
    InvalidCredentials,
    InvalidToken,
    TokenExpired,
    InternalServerError,
}

// Implement the Display trait for the AppError enum
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::InvalidToken => write!(f, "Invalid token"),
            AppError::TokenExpired => write!(f, "Token expired"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

// Implement the Error trait for the AppError enum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({ "error": message })); // create a json response
        (status, body).into_response() // return the response
    }
}