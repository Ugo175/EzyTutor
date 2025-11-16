use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::database::DbPool;
use crate::errors::AppError;
use crate::models::{CreateCourseRequest, UpdateCourseRequest};
use crate::services::CourseService;
use crate::middleware::Claims;

pub async fn create_course(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    request: web::Json<CreateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let course = CourseService::create_course(&pool, claims.sub, request.into_inner()).await?;
    Ok(HttpResponse::Created().json(course))
}

pub async fn get_all_courses(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let courses = CourseService::get_all_courses(&pool).await?;
    Ok(HttpResponse::Ok().json(courses))
}

pub async fn get_course(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();
    let course = CourseService::get_course_by_id(&pool, course_id).await?;
    Ok(HttpResponse::Ok().json(course))
}

pub async fn update_course(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateCourseRequest>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();
    let course = CourseService::update_course(&pool, course_id, claims.sub, request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let course_id = path.into_inner();
    CourseService::delete_course(&pool, course_id, claims.sub).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn get_tutor_courses(
    pool: web::Data<DbPool>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, AppError> {
    let courses = CourseService::get_courses_by_tutor(&pool, claims.sub).await?;
    Ok(HttpResponse::Ok().json(courses))
}
