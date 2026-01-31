use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod db;
mod error;
mod models;
mod services;

use crate::config::Settings;

/// Application version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build timestamp (set at compile time)
const BUILD_TIME: &str = compile_time::date_str!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration first
    let settings = Settings::load()?;

    // Initialize tracing with settings
    if settings.logging.json_output {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&settings.log_filter()))
            .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(&settings.log_filter()))
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    tracing::info!(
        version = VERSION,
        deployment_mode = %settings.deployment_mode,
        port = settings.server.port,
        "Starting Level API server"
    );

    // Create database connection pool with timeout
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .acquire_timeout(Duration::from_secs(settings.database.connect_timeout_secs))
        .connect(&settings.database.url)
        .await?;

    tracing::info!("Database connection pool established");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Database migrations completed");

    // Build application state
    let state = api::AppState::new(pool.clone(), settings.clone());

    // Create health check state
    let health_state = Arc::new(HealthState {
        pool,
        settings: settings.clone(),
        start_time: std::time::Instant::now(),
    });

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/live", get(liveness_check))
        .route("/health/ready", get(readiness_check))
        .route("/version", get(version_info))
        .with_state(health_state)
        .nest("/api/v1", api::routes(state))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Shared state for health endpoints
struct HealthState {
    pool: PgPool,
    settings: Settings,
    start_time: std::time::Instant,
}

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    deployment_mode: String,
    uptime_secs: u64,
}

/// Readiness response with component status
#[derive(Serialize)]
struct ReadinessResponse {
    status: &'static str,
    checks: ReadinessChecks,
}

#[derive(Serialize)]
struct ReadinessChecks {
    database: ComponentStatus,
}

#[derive(Serialize)]
struct ComponentStatus {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

/// Version info response
#[derive(Serialize)]
struct VersionResponse {
    version: &'static str,
    build_date: &'static str,
}

/// Simple health check - returns 200 if server is running
async fn health_check(State(state): State<Arc<HealthState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: VERSION,
        deployment_mode: state.settings.deployment_mode.to_string(),
        uptime_secs: state.start_time.elapsed().as_secs(),
    })
}

/// Liveness check - returns 200 if the process is alive
async fn liveness_check() -> &'static str {
    "OK"
}

/// Readiness check - returns 200 if all dependencies are available
async fn readiness_check(
    State(state): State<Arc<HealthState>>,
) -> Result<Json<ReadinessResponse>, (StatusCode, Json<ReadinessResponse>)> {
    // Check database connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.pool).await {
        Ok(_) => ComponentStatus {
            status: "healthy",
            message: None,
        },
        Err(e) => ComponentStatus {
            status: "unhealthy",
            message: Some(e.to_string()),
        },
    };

    let all_healthy = db_status.status == "healthy";

    let response = ReadinessResponse {
        status: if all_healthy { "ready" } else { "not_ready" },
        checks: ReadinessChecks {
            database: db_status,
        },
    };

    if all_healthy {
        Ok(Json(response))
    } else {
        Err((StatusCode::SERVICE_UNAVAILABLE, Json(response)))
    }
}

/// Version information endpoint
async fn version_info() -> Json<VersionResponse> {
    Json(VersionResponse {
        version: VERSION,
        build_date: BUILD_TIME,
    })
}
