mod config;
mod database;
mod handlers;
mod models;
mod services;
mod middleware;
mod errors;

use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = config::Config::from_env().expect("Failed to load configuration");
    
    // Create database pool - fail if not available for testing
    let pool = database::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool. Please ensure PostgreSQL is running and DATABASE_URL is set.");

    // Run migrations
    database::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    log::info!("Database connected and migrations completed successfully");
    log::info!("Starting EzyTutor server at {}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(handlers::configure_routes)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
