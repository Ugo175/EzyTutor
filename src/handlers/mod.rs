pub mod health;
pub mod auth;
pub mod course;
pub mod tutor;

use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::jwt_middleware;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(jwt_middleware);
    
    cfg.service(
        web::scope("/api/v1")
            // Public routes
            .route("/health", web::get().to(health::health_check))
            .route("/test-error", web::get().to(health::test_error))
            .route("/auth/register", web::post().to(auth::register))
            .route("/auth/login", web::post().to(auth::login))
            .route("/courses", web::get().to(course::get_all_courses))
            .route("/courses/{id}", web::get().to(course::get_course))
            .route("/tutors", web::get().to(tutor::get_all_tutors))
            .route("/tutors/{id}", web::get().to(tutor::get_tutor))
            .route("/tutors/{id}/reviews", web::get().to(tutor::get_tutor_reviews))
            .route("/tutors/search", web::get().to(tutor::search_tutors))
            
            // Protected routes
            .service(
                web::scope("")
                    .wrap(auth)
                    // Course management (tutors only)
                    .route("/courses", web::post().to(course::create_course))
                    .route("/courses/{id}", web::put().to(course::update_course))
                    .route("/courses/{id}", web::delete().to(course::delete_course))
                    .route("/my/courses", web::get().to(course::get_tutor_courses))
                    
                    // Tutor profile management
                    .route("/tutors/profile", web::post().to(tutor::create_tutor_profile))
                    .route("/tutors/profile", web::put().to(tutor::update_tutor_profile))
                    
                    // Reviews (students only)
                    .route("/tutors/{id}/reviews", web::post().to(tutor::create_review))
            )
    );
}
