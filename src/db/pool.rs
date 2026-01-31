//! Database connection pool

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::DatabaseSettings;
use crate::error::Result;

/// Create database connection pool
pub async fn create_pool(settings: &DatabaseSettings) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .connect(&settings.url)
        .await
        .map_err(|e| crate::error::LevelError::Database(e))?;

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| crate::error::LevelError::Internal(format!("Migration error: {}", e)))?;

    Ok(())
}
