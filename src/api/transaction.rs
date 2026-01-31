//! Transaction API endpoints

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::{LevelError, Result};
use crate::models::{BatchVerbRequest, Transaction, TransactionListResponse, Verb, VerbRequest};
use crate::services::{TransactionService, VerbHandler};

use super::state::AppState;

/// Transaction routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/nouns/:sow_id/transactions", get(list_sow_transactions))
        .route(
            "/nouns/:sow_id/noun/:noun_id/transactions",
            get(list_noun_transactions).post(apply_verb),
        )
        .route("/nouns/:sow_id/batch/transactions", post(batch_apply))
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
    actor: Option<String>,
}

fn default_limit() -> i64 {
    50
}

/// GET /nouns/:sow_id/transactions - List SOW-wide transactions
async fn list_sow_transactions(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<TransactionListResponse>> {
    let transactions = TransactionService::get_for_sow(
        &state.pool,
        sow_id,
        query.limit,
        query.offset,
        query.actor.as_deref(),
    )
    .await?;

    Ok(Json(TransactionListResponse {
        transactions,
        total: 0, // TODO: Add count query
        next_cursor: None,
    }))
}

/// GET /nouns/:sow_id/noun/:noun_id/transactions - List noun transactions
async fn list_noun_transactions(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<TransactionQuery>,
) -> Result<Json<TransactionListResponse>> {
    let transactions =
        TransactionService::get_for_noun(&state.pool, noun_id, query.limit, query.offset).await?;

    Ok(Json(TransactionListResponse {
        transactions,
        total: 0,
        next_cursor: None,
    }))
}

/// POST /nouns/:sow_id/noun/:noun_id/transactions - Apply verb
async fn apply_verb(
    State(state): State<AppState>,
    Path((sow_id, noun_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<VerbRequest>,
) -> Result<Json<serde_json::Value>> {
    let actor = "+system"; // TODO: Extract from request

    match request.verb {
        Verb::Complete => {
            let noun = VerbHandler::complete(&state.pool, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "completed", "noun": noun})))
        }
        Verb::Incomplete => {
            let noun = VerbHandler::incomplete(&state.pool, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "incompleted", "noun": noun})))
        }
        Verb::Escalate => {
            let noun = VerbHandler::escalate(&state.pool, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "escalated", "noun": noun})))
        }
        Verb::Normal => {
            let noun = VerbHandler::normal(&state.pool, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "normalized", "noun": noun})))
        }
        Verb::Close => {
            let noun = VerbHandler::close(&state.pool, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "closed", "noun": noun})))
        }
        Verb::Reparent => {
            let parent_id = request
                .context
                .as_ref()
                .and_then(|c| c.get("parent_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok());
            let noun = VerbHandler::reparent(&state.pool, noun_id, parent_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "reparented", "noun": noun})))
        }
        Verb::Assign => {
            let container_id = request
                .context
                .as_ref()
                .and_then(|c| c.get("container_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| LevelError::ValidationError("container_id required".to_string()))?;
            let sort_order = request
                .context
                .as_ref()
                .and_then(|c| c.get("sort_order"))
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            let assignment =
                VerbHandler::assign(&state.pool, noun_id, container_id, sort_order, actor).await?;
            Ok(Json(serde_json::json!({"status": "assigned", "assignment": assignment})))
        }
        Verb::Unassign => {
            let container_id = request
                .context
                .as_ref()
                .and_then(|c| c.get("container_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| LevelError::ValidationError("container_id required".to_string()))?;
            VerbHandler::unassign(&state.pool, noun_id, container_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "unassigned"})))
        }
        Verb::Blocked => {
            let blocker_id = request
                .context
                .as_ref()
                .and_then(|c| c.get("blocker_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| LevelError::ValidationError("blocker_id required".to_string()))?;
            let reason = request
                .context
                .as_ref()
                .and_then(|c| c.get("reason"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let block = VerbHandler::blocked(&state.pool, blocker_id, noun_id, reason, actor).await?;
            Ok(Json(serde_json::json!({"status": "blocked", "block": block})))
        }
        Verb::Release => {
            let blocker_id = request
                .context
                .as_ref()
                .and_then(|c| c.get("blocker_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| LevelError::ValidationError("blocker_id required".to_string()))?;
            VerbHandler::release(&state.pool, blocker_id, noun_id, actor).await?;
            Ok(Json(serde_json::json!({"status": "released"})))
        }
        _ => Err(LevelError::ValidationError(format!(
            "Verb {} not supported via this endpoint",
            request.verb
        ))),
    }
}

/// POST /nouns/:sow_id/batch/transactions - Batch apply verbs
async fn batch_apply(
    State(state): State<AppState>,
    Path(sow_id): Path<Uuid>,
    Json(request): Json<BatchVerbRequest>,
) -> Result<Json<serde_json::Value>> {
    let actor = "+system";
    let mut results = vec![];
    let mut errors = vec![];

    // All-or-nothing: validate all first
    for noun_id in &request.noun_ids {
        // Verify noun exists and belongs to SOW
        let noun = crate::db::NounRepository::get_by_id(&state.pool, *noun_id).await?;
        if noun.is_none() {
            errors.push(serde_json::json!({
                "noun_id": noun_id.to_string(),
                "error": "Noun not found"
            }));
        }
    }

    if !errors.is_empty() {
        return Ok(Json(serde_json::json!({
            "status": "failed",
            "errors": errors
        })));
    }

    // Execute all
    for noun_id in request.noun_ids {
        match request.verb {
            Verb::Complete => {
                let result = VerbHandler::complete(&state.pool, noun_id, actor).await;
                match result {
                    Ok(noun) => results.push(serde_json::json!({"noun_id": noun_id.to_string(), "status": "completed"})),
                    Err(e) => errors.push(serde_json::json!({"noun_id": noun_id.to_string(), "error": e.to_string()})),
                }
            }
            Verb::Escalate => {
                let result = VerbHandler::escalate(&state.pool, noun_id, actor).await;
                match result {
                    Ok(noun) => results.push(serde_json::json!({"noun_id": noun_id.to_string(), "status": "escalated"})),
                    Err(e) => errors.push(serde_json::json!({"noun_id": noun_id.to_string(), "error": e.to_string()})),
                }
            }
            Verb::Close => {
                let result = VerbHandler::close(&state.pool, noun_id, actor).await;
                match result {
                    Ok(noun) => results.push(serde_json::json!({"noun_id": noun_id.to_string(), "status": "closed"})),
                    Err(e) => errors.push(serde_json::json!({"noun_id": noun_id.to_string(), "error": e.to_string()})),
                }
            }
            _ => {
                errors.push(serde_json::json!({
                    "noun_id": noun_id.to_string(),
                    "error": format!("Verb {} not supported in batch", request.verb)
                }));
            }
        }
    }

    if errors.is_empty() {
        Ok(Json(serde_json::json!({
            "status": "success",
            "results": results
        })))
    } else {
        Ok(Json(serde_json::json!({
            "status": "partial",
            "results": results,
            "errors": errors
        })))
    }
}
