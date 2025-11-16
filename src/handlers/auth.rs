use actix_web::{web, HttpResponse, Result};
use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{CreateUserRequest, LoginRequest};
use crate::services::AuthService;

pub async fn register(
    pool: web::Data<DbPool>,
    request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
    let user = AuthService::register_user(&pool, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let login_response = AuthService::login_user(&pool, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(login_response))
}
