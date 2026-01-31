//! User API endpoints (/user)

use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::error::{LevelError, Result};
use crate::models::{Alias, CreateAliasRequest, UpdatePreferencesRequest, UserPreferences};

use super::state::AppState;

/// User routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/aliases", get(list_aliases).post(create_alias))
        .route("/user/aliases/:name", delete(delete_alias))
        .route("/user/profile", get(get_profile))
        .route("/user/preferences", get(get_preferences).put(update_preferences))
        .route("/user/roles", get(get_user_roles))
        .route("/user/sows", get(get_user_sows))
}

fn get_current_actor() -> String {
    // TODO: Extract from request/session
    "+system".to_string()
}

/// GET /user/aliases - List user's aliases
async fn list_aliases(State(state): State<AppState>) -> Result<Json<Vec<Alias>>> {
    let actor = get_current_actor();
    let aliases = sqlx::query_as::<_, Alias>(
        r#"SELECT * FROM aliases WHERE actor = $1 ORDER BY name"#,
    )
    .bind(&actor)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(aliases))
}

/// POST /user/aliases - Create alias
async fn create_alias(
    State(state): State<AppState>,
    Json(request): Json<CreateAliasRequest>,
) -> Result<Json<Alias>> {
    let actor = get_current_actor();

    // Check uniqueness
    let exists: (bool,) = sqlx::query_as(
        r#"SELECT EXISTS(SELECT 1 FROM aliases WHERE actor = $1 AND name = $2)"#,
    )
    .bind(&actor)
    .bind(&request.name)
    .fetch_one(&state.pool)
    .await?;

    if exists.0 {
        return Err(LevelError::DuplicateShortName(format!(
            "Alias {} already exists",
            request.name
        )));
    }

    let alias = Alias::new(actor, request.name, request.noun_id);
    sqlx::query(
        r#"INSERT INTO aliases (id, actor, name, noun_id, created_at)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(alias.id)
    .bind(&alias.actor)
    .bind(&alias.name)
    .bind(alias.noun_id)
    .bind(alias.created_at)
    .execute(&state.pool)
    .await?;

    Ok(Json(alias))
}

/// DELETE /user/aliases/:name - Delete alias
async fn delete_alias(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let actor = get_current_actor();
    sqlx::query(r#"DELETE FROM aliases WHERE actor = $1 AND name = $2"#)
        .bind(&actor)
        .bind(&name)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

/// GET /user/profile - Get user profile
async fn get_profile(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    let actor = get_current_actor();

    // For person actors, get from persons table
    if actor.starts_with('+') {
        let username = &actor[1..];
        let person: Option<(Uuid, String, String, String)> = sqlx::query_as(
            r#"SELECT id, username, email, display_name FROM persons WHERE username = $1"#,
        )
        .bind(username)
        .fetch_optional(&state.pool)
        .await?;

        if let Some((id, username, email, display_name)) = person {
            return Ok(Json(serde_json::json!({
                "actor": actor,
                "type": "person",
                "id": id.to_string(),
                "username": username,
                "email": email,
                "display_name": display_name
            })));
        }
    }

    Ok(Json(serde_json::json!({
        "actor": actor,
        "type": "unknown"
    })))
}

/// GET /user/preferences - Get user preferences
async fn get_preferences(State(state): State<AppState>) -> Result<Json<UserPreferences>> {
    let actor = get_current_actor();

    let prefs = sqlx::query_as::<_, UserPreferences>(
        r#"SELECT * FROM user_preferences WHERE actor = $1"#,
    )
    .bind(&actor)
    .fetch_optional(&state.pool)
    .await?;

    match prefs {
        Some(p) => Ok(Json(p)),
        None => {
            // Create default preferences
            let prefs = UserPreferences::new(actor.clone());
            sqlx::query(
                r#"INSERT INTO user_preferences (id, actor, timezone, theme, date_format, notifications, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            )
            .bind(prefs.id)
            .bind(&prefs.actor)
            .bind(&prefs.timezone)
            .bind(&prefs.theme)
            .bind(&prefs.date_format)
            .bind(&prefs.notifications)
            .bind(prefs.created_at)
            .bind(prefs.updated_at)
            .execute(&state.pool)
            .await?;
            Ok(Json(prefs))
        }
    }
}

/// PUT /user/preferences - Update preferences
async fn update_preferences(
    State(state): State<AppState>,
    Json(request): Json<UpdatePreferencesRequest>,
) -> Result<Json<UserPreferences>> {
    let actor = get_current_actor();
    let now = chrono::Utc::now();

    // Ensure preferences exist
    let _ = get_preferences(State(state.clone())).await?;

    // Update
    if let Some(default_sow) = request.default_sow {
        sqlx::query(r#"UPDATE user_preferences SET default_sow = $1, updated_at = $2 WHERE actor = $3"#)
            .bind(default_sow)
            .bind(now)
            .bind(&actor)
            .execute(&state.pool)
            .await?;
    }
    if let Some(timezone) = request.timezone {
        sqlx::query(r#"UPDATE user_preferences SET timezone = $1, updated_at = $2 WHERE actor = $3"#)
            .bind(&timezone)
            .bind(now)
            .bind(&actor)
            .execute(&state.pool)
            .await?;
    }
    if let Some(theme) = request.theme {
        sqlx::query(r#"UPDATE user_preferences SET theme = $1, updated_at = $2 WHERE actor = $3"#)
            .bind(&theme)
            .bind(now)
            .bind(&actor)
            .execute(&state.pool)
            .await?;
    }
    if let Some(date_format) = request.date_format {
        sqlx::query(r#"UPDATE user_preferences SET date_format = $1, updated_at = $2 WHERE actor = $3"#)
            .bind(&date_format)
            .bind(now)
            .bind(&actor)
            .execute(&state.pool)
            .await?;
    }
    if let Some(notifications) = request.notifications {
        sqlx::query(r#"UPDATE user_preferences SET notifications = $1, updated_at = $2 WHERE actor = $3"#)
            .bind(&notifications)
            .bind(now)
            .bind(&actor)
            .execute(&state.pool)
            .await?;
    }

    // Fetch updated
    let prefs = sqlx::query_as::<_, UserPreferences>(
        r#"SELECT * FROM user_preferences WHERE actor = $1"#,
    )
    .bind(&actor)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(prefs))
}

/// GET /user/roles - Get user's roles
async fn get_user_roles(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    let actor = get_current_actor();

    let roles: Vec<(String, Option<Uuid>)> = sqlx::query_as(
        r#"
        SELECT r.name, rm.sow_id
        FROM roles r
        INNER JOIN role_mappings rm ON rm.role_id = r.id
        WHERE rm.actor = $1
        "#,
    )
    .bind(&actor)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "actor": actor,
        "roles": roles.into_iter().map(|(name, sow_id)| {
            serde_json::json!({
                "role": name,
                "sow_id": sow_id.map(|id| id.to_string())
            })
        }).collect::<Vec<_>>()
    })))
}

/// GET /user/sows - Get user's accessible SOWs
async fn get_user_sows(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    let actor = get_current_actor();

    // Get SOWs where user has role mapping
    let sows: Vec<(Uuid, String, String)> = sqlx::query_as(
        r#"
        SELECT DISTINCT n.id, n.short_name, n.title
        FROM nouns n
        INNER JOIN role_mappings rm ON rm.sow_id = n.id
        WHERE n.type = 'sow' AND rm.actor = $1
        ORDER BY n.short_name
        "#,
    )
    .bind(&actor)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "sows": sows.into_iter().map(|(id, short_name, title)| {
            serde_json::json!({
                "id": id.to_string(),
                "short_name": short_name,
                "title": title
            })
        }).collect::<Vec<_>>()
    })))
}
