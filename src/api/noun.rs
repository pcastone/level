//! Noun API endpoints

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::NounRepository;
use crate::error::{LevelError, Result};
use crate::models::{CreateNounRequest, Noun, NounListResponse, NounState, NounType, UpdateNounRequest};
use crate::services::VerbHandler;

use super::middleware::extract_actor;
use super::state::AppState;

/// Noun routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/nouns/:sow_id/nouns", get(list_nouns).post(create_noun))
        .route(
            "/nouns/:sow_id/noun/:noun_id",
            get(get_noun).put(update_noun).delete(delete_noun),
        )
        .route("/nouns/:sow_id/noun/:noun_id/children", get(get_children))
}

#[derive(Debug, Deserialize)]
pub struct NounListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
    #[serde(rename = "type")]
    noun_type: Option<String>,
    state: Option<String>,
    blocked: Option<bool>,
}

fn default_limit() -> i64 {
    50
}

/// GET /nouns/:sow_id/nouns - List nouns in SOW
async fn list_nouns(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Query(query): Query<NounListQuery>,
) -> Result<Json<NounListResponse>> {
    let noun_type = query.noun_type.and_then(|s| NounType::from_str(&s));
    let noun_state = query.state.and_then(|s| NounState::from_str(&s));

    let nouns = NounRepository::list_for_sow(
        &state.pool,
        sow_id,
        noun_type,
        noun_state,
        query.blocked,
        query.limit,
        query.offset,
    )
    .await?;

    let total = NounRepository::count_for_sow(&state.pool, sow_id).await?;

    Ok(Json(NounListResponse {
        nouns,
        total,
        next_cursor: None,
    }))
}

/// POST /nouns/:sow_id/nouns - Create noun
async fn create_noun(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Json(request): Json<CreateNounRequest>,
) -> Result<Json<Noun>> {
    let actor = "+system"; // TODO: Extract from request
    let noun = VerbHandler::open(&state.pool, sow_id, request, actor).await?;
    Ok(Json(noun))
}

/// GET /nouns/:sow_id/noun/:noun_id - Get noun details
async fn get_noun(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Noun>> {
    let noun = NounRepository::get_by_id(&state.pool, noun_id)
        .await?
        .ok_or_else(|| LevelError::NounNotFound(noun_id.to_string()))?;

    if noun.sow_id != sow_id {
        return Err(LevelError::NounNotFound(noun_id.to_string()));
    }

    Ok(Json(noun))
}

/// PUT /nouns/:sow_id/noun/:noun_id - Update noun
async fn update_noun(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateNounRequest>,
) -> Result<Json<Noun>> {
    // Verify noun belongs to SOW
    let existing = NounRepository::get_by_id(&state.pool, noun_id)
        .await?
        .ok_or_else(|| LevelError::NounNotFound(noun_id.to_string()))?;

    if existing.sow_id != sow_id {
        return Err(LevelError::NounNotFound(noun_id.to_string()));
    }

    let actor = "+system"; // TODO: Extract from request
    let noun = VerbHandler::update(&state.pool, noun_id, request, actor).await?;
    Ok(Json(noun))
}

/// DELETE /nouns/:sow_id/noun/:noun_id - Delete noun
async fn delete_noun(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<serde_json::Value>> {
    // Verify noun belongs to SOW
    let existing = NounRepository::get_by_id(&state.pool, noun_id)
        .await?
        .ok_or_else(|| LevelError::NounNotFound(noun_id.to_string()))?;

    if existing.sow_id != sow_id {
        return Err(LevelError::NounNotFound(noun_id.to_string()));
    }

    NounRepository::delete(&state.pool, noun_id).await?;

    Ok(Json(serde_json::json!({"status": "deleted", "id": noun_id.to_string()})))
}

/// GET /nouns/:sow_id/noun/:noun_id/children - Get children
async fn get_children(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<Noun>>> {
    let children = NounRepository::get_children(&state.pool, noun_id).await?;
    Ok(Json(children))
}
