use actix_web::{HttpResponse, Result, web};
use serde_json::json;
use chrono::Utc;
use crate::errors::AppError;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "EzyTutor API",
        "version": "0.1.0",
        "timestamp": Utc::now().to_rfc3339()
    })))
}

// Example endpoint that demonstrates error usage
pub async fn test_error() -> Result<HttpResponse, AppError> {
    // This will use our NotFound error variant
    Err(AppError::NotFound("This is a test error endpoint".to_string()))
}
