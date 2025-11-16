mod config;
mod database;
mod handlers;
mod models;
mod services;
mod middleware;
mod errors;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = config::Config::from_env().expect("Failed to load configuration");
    
    // Try to create database pool, but don't fail if it's not available
    let pool_result = database::create_pool(&config.database_url).await;
    
    match pool_result {
        Ok(pool) => {
            log::info!("Database connected successfully");
            if let Err(e) = database::run_migrations(&pool).await {
                log::warn!("Migration failed: {}", e);
            }
        }
        Err(e) => {
            log::warn!("Database connection failed: {}. Running without database.", e);
        }
    }

    log::info!("Starting EzyTutor server at {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(handlers::configure_routes)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
