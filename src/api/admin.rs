//! Admin API endpoints (/sys0)

use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::error::Result;
use crate::models::{
    Ai, CreateAiRequest, CreateGroupRequest, CreatePersonRequest, CreateRoleMappingRequest,
    CreateRoleRequest, CreateVendorRequest, Group, Person, Role, RoleMapping, Vendor,
};

use super::state::AppState;

/// Admin routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Actors
        .route("/sys0/actors/persons", get(list_persons).post(create_person))
        .route("/sys0/actors/persons/:id", get(get_person).delete(delete_person))
        .route("/sys0/actors/groups", get(list_groups).post(create_group))
        .route("/sys0/actors/groups/:id", get(get_group).delete(delete_group))
        .route("/sys0/actors/vendors", get(list_vendors).post(create_vendor))
        .route("/sys0/actors/vendors/:id", get(get_vendor).delete(delete_vendor))
        .route("/sys0/actors/ai", get(list_ais).post(create_ai))
        .route("/sys0/actors/ai/:id", get(get_ai).delete(delete_ai))
        // Roles
        .route("/sys0/roles", get(list_roles).post(create_role))
        .route("/sys0/roles/:id", get(get_role).delete(delete_role))
        .route("/sys0/role-mappings", post(create_role_mapping))
}

// Person endpoints
async fn list_persons(State(state): State<AppState>) -> Result<Json<Vec<Person>>> {
    let persons = sqlx::query_as::<_, Person>(
        r#"SELECT * FROM persons WHERE active = true ORDER BY username"#,
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(persons))
}

async fn create_person(
    State(state): State<AppState>,
    Json(request): Json<CreatePersonRequest>,
) -> Result<Json<Person>> {
    let person = Person::new(request.username, request.email, request.display_name);
    sqlx::query(
        r#"INSERT INTO persons (id, username, email, display_name, source, active, created_at, updated_at)
           VALUES ($1, $2, $3, $4, 'internal', true, $5, $6)"#,
    )
    .bind(person.id)
    .bind(&person.username)
    .bind(&person.email)
    .bind(&person.display_name)
    .bind(person.created_at)
    .bind(person.updated_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(person))
}

async fn get_person(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Person>> {
    let person = sqlx::query_as::<_, Person>(r#"SELECT * FROM persons WHERE id = $1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(person))
}

async fn delete_person(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    sqlx::query(r#"UPDATE persons SET active = false WHERE id = $1"#)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

// Group endpoints
async fn list_groups(State(state): State<AppState>) -> Result<Json<Vec<Group>>> {
    let groups = sqlx::query_as::<_, Group>(
        r#"SELECT * FROM groups WHERE active = true ORDER BY name"#,
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(groups))
}

async fn create_group(
    State(state): State<AppState>,
    Json(request): Json<CreateGroupRequest>,
) -> Result<Json<Group>> {
    let group = Group::new(request.name, request.description);
    sqlx::query(
        r#"INSERT INTO groups (id, name, description, source, active, created_at, updated_at)
           VALUES ($1, $2, $3, 'internal', true, $4, $5)"#,
    )
    .bind(group.id)
    .bind(&group.name)
    .bind(&group.description)
    .bind(group.created_at)
    .bind(group.updated_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(group))
}

async fn get_group(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Group>> {
    let group = sqlx::query_as::<_, Group>(r#"SELECT * FROM groups WHERE id = $1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(group))
}

async fn delete_group(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    sqlx::query(r#"UPDATE groups SET active = false WHERE id = $1"#)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

// Vendor endpoints
async fn list_vendors(State(state): State<AppState>) -> Result<Json<Vec<Vendor>>> {
    let vendors = sqlx::query_as::<_, Vendor>(
        r#"SELECT * FROM vendors WHERE active = true ORDER BY name"#,
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(vendors))
}

async fn create_vendor(
    State(state): State<AppState>,
    Json(request): Json<CreateVendorRequest>,
) -> Result<Json<Vendor>> {
    let mut vendor = Vendor::new(request.name);
    vendor.contact_email = request.contact_email;
    vendor.description = request.description;
    sqlx::query(
        r#"INSERT INTO vendors (id, name, contact_email, description, active, created_at, updated_at)
           VALUES ($1, $2, $3, $4, true, $5, $6)"#,
    )
    .bind(vendor.id)
    .bind(&vendor.name)
    .bind(&vendor.contact_email)
    .bind(&vendor.description)
    .bind(vendor.created_at)
    .bind(vendor.updated_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(vendor))
}

async fn get_vendor(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Vendor>> {
    let vendor = sqlx::query_as::<_, Vendor>(r#"SELECT * FROM vendors WHERE id = $1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(vendor))
}

async fn delete_vendor(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    sqlx::query(r#"UPDATE vendors SET active = false WHERE id = $1"#)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

// AI endpoints
async fn list_ais(State(state): State<AppState>) -> Result<Json<Vec<Ai>>> {
    let ais = sqlx::query_as::<_, Ai>(
        r#"SELECT * FROM ais WHERE active = true ORDER BY name"#,
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(ais))
}

async fn create_ai(
    State(state): State<AppState>,
    Json(request): Json<CreateAiRequest>,
) -> Result<Json<Ai>> {
    let mut ai = Ai::new(request.name, request.model);
    ai.description = request.description;
    sqlx::query(
        r#"INSERT INTO ais (id, name, model, description, active, created_at, updated_at)
           VALUES ($1, $2, $3, $4, true, $5, $6)"#,
    )
    .bind(ai.id)
    .bind(&ai.name)
    .bind(&ai.model)
    .bind(&ai.description)
    .bind(ai.created_at)
    .bind(ai.updated_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(ai))
}

async fn get_ai(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Ai>> {
    let ai = sqlx::query_as::<_, Ai>(r#"SELECT * FROM ais WHERE id = $1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(ai))
}

async fn delete_ai(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    sqlx::query(r#"UPDATE ais SET active = false WHERE id = $1"#)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

// Role endpoints
async fn list_roles(State(state): State<AppState>) -> Result<Json<Vec<Role>>> {
    let roles = sqlx::query_as::<_, Role>(r#"SELECT * FROM roles ORDER BY name"#)
        .fetch_all(&state.pool)
        .await?;
    Ok(Json(roles))
}

async fn create_role(
    State(state): State<AppState>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<Json<Role>> {
    let mut role = Role::new(request.name, request.verbs);
    role.description = request.description;
    role.inherits_from = request.inherits_from;
    sqlx::query(
        r#"INSERT INTO roles (id, name, description, inherits_from, verbs, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
    )
    .bind(role.id)
    .bind(&role.name)
    .bind(&role.description)
    .bind(role.inherits_from)
    .bind(&role.verbs)
    .bind(role.created_at)
    .bind(role.updated_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(role))
}

async fn get_role(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Role>> {
    let role = sqlx::query_as::<_, Role>(r#"SELECT * FROM roles WHERE id = $1"#)
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(role))
}

async fn delete_role(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<serde_json::Value>> {
    sqlx::query(r#"DELETE FROM roles WHERE id = $1"#)
        .bind(id)
        .execute(&state.pool)
        .await?;
    Ok(Json(serde_json::json!({"status": "deleted"})))
}

async fn create_role_mapping(
    State(state): State<AppState>,
    Json(request): Json<CreateRoleMappingRequest>,
) -> Result<Json<RoleMapping>> {
    let mapping = RoleMapping::new(request.role_id, request.actor, request.sow_id);
    sqlx::query(
        r#"INSERT INTO role_mappings (id, role_id, actor, sow_id, created_at)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(mapping.id)
    .bind(mapping.role_id)
    .bind(&mapping.actor)
    .bind(mapping.sow_id)
    .bind(mapping.created_at)
    .execute(&state.pool)
    .await?;
    Ok(Json(mapping))
}
