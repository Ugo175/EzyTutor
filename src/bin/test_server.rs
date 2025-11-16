// Test server that runs without database for basic endpoint testing
use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse, Result};
use actix_cors::Cors;
use serde_json::json;
use chrono::Utc;

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "EzyTutor API (Test Mode)",
        "version": "0.1.0",
        "timestamp": Utc::now().to_rfc3339(),
        "database": "disabled"
    })))
}

async fn test_error() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(json!({
        "error": "This is a test error endpoint",
        "status": 404
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = "127.0.0.1";
    let port = 8080;

    println!("🚀 Starting EzyTutor Test Server at {}:{}", host, port);
    println!("📋 Available endpoints:");
    println!("   GET  /api/v1/health");
    println!("   GET  /api/v1/test-error");
    println!("💡 This is a test server without database functionality");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .route("/health", web::get().to(health_check))
                    .route("/test-error", web::get().to(test_error))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
