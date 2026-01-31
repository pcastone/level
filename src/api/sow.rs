//! SOW API endpoints

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::NounRepository;
use crate::error::{LevelError, Result};
use crate::models::{CreateNounRequest, Noun, NounType};
use crate::services::{ShortNameService, VerbHandler};

use super::state::AppState;

/// SOW routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sows", get(list_sows).post(create_sow))
        .route("/sow/:sow_id", get(get_sow))
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize)]
pub struct SowListResponse {
    sows: Vec<Noun>,
    total: i64,
}

/// GET /sows - List all SOWs
async fn list_sows(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<SowListResponse>> {
    let sows = NounRepository::list_sows(&state.pool, query.limit, query.offset).await?;

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM nouns WHERE type = 'sow'"#,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(SowListResponse {
        sows,
        total: total.0,
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateSowRequest {
    short_name: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    mode: Option<String>,
}

/// POST /sows - Create new SOW
async fn create_sow(
    State(state): State<AppState>,
    Json(request): Json<CreateSowRequest>,
) -> Result<Json<Noun>> {
    // Validate short_name
    ShortNameService::validate_sow_short_name(&request.short_name)?;

    // Check uniqueness
    if !ShortNameService::is_sow_short_name_unique(&state.pool, &request.short_name).await? {
        return Err(LevelError::DuplicateShortName(request.short_name));
    }

    // Create the SOW - it references itself as sow_id
    let sow_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let mode = request.mode.unwrap_or_else(|| "standard".to_string());
    let custom_fields = serde_json::json!({
        "mode": mode,
        "roles": {"allow": [], "deny": []}
    });

    sqlx::query(
        r#"
        INSERT INTO nouns (id, type, sow_id, short_name, title, description, state, is_blocked, custom_fields, created_at, updated_at)
        VALUES ($1, $2, $1, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(sow_id)
    .bind(NounType::Sow.to_string())
    .bind(&request.short_name)
    .bind(&request.title)
    .bind(&request.description)
    .bind("Normal")
    .bind(false)
    .bind(&custom_fields)
    .bind(now)
    .bind(now)
    .execute(&state.pool)
    .await?;

    let sow = NounRepository::get_by_id(&state.pool, sow_id)
        .await?
        .ok_or_else(|| LevelError::Internal("Failed to create SOW".to_string()))?;

    Ok(Json(sow))
}

/// GET /sow/:sow_id - Get SOW details
async fn get_sow(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
) -> Result<Json<Noun>> {
    let sow = NounRepository::get_by_id(&state.pool, sow_id)
        .await?
        .ok_or_else(|| LevelError::SowNotFound(sow_id.to_string()))?;

    if sow.get_type() != NounType::Sow {
        return Err(LevelError::SowNotFound(sow_id.to_string()));
    }

    Ok(Json(sow))
}
