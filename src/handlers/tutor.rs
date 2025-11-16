use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{CreateTutorProfileRequest, UpdateTutorProfileRequest, CreateReviewRequest};
use crate::services::TutorService;
use crate::middleware::Claims;

pub async fn create_tutor_profile(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    request: web::Json<CreateTutorProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let tutor = TutorService::create_tutor_profile(&pool, claims.sub, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(tutor))
}

pub async fn get_all_tutors(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let tutors = TutorService::get_all_tutors(&pool).await?;
    Ok(HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let tutor_id = path.into_inner();
    let tutor = TutorService::get_tutor_by_id(&pool, tutor_id).await?;
    Ok(HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_profile(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    request: web::Json<UpdateTutorProfileRequest>,
) -> Result<HttpResponse, AppError> {
    let tutor = TutorService::update_tutor_profile(&pool, claims.sub, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(tutor))
}

pub async fn create_review(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    path: web::Path<Uuid>,
    request: web::Json<CreateReviewRequest>,
) -> Result<HttpResponse, AppError> {
    let tutor_id = path.into_inner();
    let review = TutorService::create_review(&pool, tutor_id, claims.sub, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(review))
}

pub async fn get_tutor_reviews(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let tutor_id = path.into_inner();
    let reviews = TutorService::get_tutor_reviews(&pool, tutor_id).await?;
    Ok(HttpResponse::Ok().json(reviews))
}

pub async fn search_tutors(
    pool: web::Data<DbPool>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let tutors = if let Some(specialization) = &query.specialization {
        TutorService::search_tutors_by_specialization(&pool, specialization).await?
    } else {
        TutorService::get_all_tutors(&pool).await?
    };
    Ok(HttpResponse::Ok().json(tutors))
}

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub specialization: Option<String>,
}
