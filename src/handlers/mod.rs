pub mod health;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health::health_check))
            .route("/test-error", web::get().to(health::test_error))
    );
}
