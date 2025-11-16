use sqlx::{PgPool, Pool, Postgres};
use crate::errors::AppResult;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> AppResult<DbPool> {
    let pool = PgPool::connect(database_url)
        .await
        .map_err(|e| crate::errors::AppError::Database(e))?;
    
    log::info!("Database connection pool created successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> AppResult<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| crate::errors::AppError::Internal(format!("Migration failed: {}", e)))?;
    
    log::info!("Database migrations completed successfully");
    Ok(())
}
