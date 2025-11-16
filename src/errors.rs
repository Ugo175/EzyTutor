use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_message) = match self {
            AppError::Database(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            AppError::Validation(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Authentication(msg) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                msg.clone(),
            ),
            AppError::Authorization(msg) => (
                actix_web::http::StatusCode::FORBIDDEN,
                msg.clone(),
            ),
            AppError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            AppError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Internal(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
        };

        HttpResponse::build(status_code).json(json!({
            "error": error_message,
            "status": status_code.as_u16()
        }))
    }
}
