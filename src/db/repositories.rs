//! Repository implementations

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{LevelError, Result};
use crate::models::{Noun, NounState, NounType};

/// Noun repository for database operations
pub struct NounRepository;

impl NounRepository {
    /// Get noun by ID
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Noun>> {
        let noun = sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(noun)
    }

    /// Get noun by short_name
    pub async fn get_by_short_name(pool: &PgPool, short_name: &str) -> Result<Option<Noun>> {
        let noun = sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE short_name = $1
            "#,
        )
        .bind(short_name)
        .fetch_optional(pool)
        .await?;

        Ok(noun)
    }

    /// List nouns for a SOW with filters
    pub async fn list_for_sow(
        pool: &PgPool,
        sow_id: Uuid,
        noun_type: Option<NounType>,
        state: Option<NounState>,
        is_blocked: Option<bool>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Noun>> {
        let mut query = String::from(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE sow_id = $1
            "#,
        );

        let mut params: Vec<String> = vec![];
        let mut param_idx = 2;

        if noun_type.is_some() {
            query.push_str(&format!(" AND type = ${}", param_idx));
            param_idx += 1;
        }
        if state.is_some() {
            query.push_str(&format!(" AND state = ${}", param_idx));
            param_idx += 1;
        }
        if is_blocked.is_some() {
            query.push_str(&format!(" AND is_blocked = ${}", param_idx));
            param_idx += 1;
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_idx, param_idx + 1));

        // Build query dynamically based on filters
        let nouns = if let (Some(nt), Some(st), Some(blocked)) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(nt.to_string())
                .bind(st.to_string())
                .bind(blocked)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (Some(nt), Some(st), None) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(nt.to_string())
                .bind(st.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (Some(nt), None, Some(blocked)) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(nt.to_string())
                .bind(blocked)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (None, Some(st), Some(blocked)) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(st.to_string())
                .bind(blocked)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (Some(nt), None, None) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(nt.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (None, Some(st), None) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(st.to_string())
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else if let (None, None, Some(blocked)) = (noun_type, state, is_blocked) {
            sqlx::query_as::<_, Noun>(&query)
                .bind(sow_id)
                .bind(blocked)
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?
        } else {
            sqlx::query_as::<_, Noun>(
                r#"
                SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                       state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
                FROM nouns WHERE sow_id = $1
                ORDER BY created_at DESC LIMIT $2 OFFSET $3
                "#,
            )
            .bind(sow_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok(nouns)
    }

    /// Count nouns for a SOW
    pub async fn count_for_sow(pool: &PgPool, sow_id: Uuid) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM nouns WHERE sow_id = $1"#,
        )
        .bind(sow_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// List all SOWs
    pub async fn list_sows(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Noun>> {
        let nouns = sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE type = 'sow'
            ORDER BY created_at DESC LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(nouns)
    }

    /// Delete noun by ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<()> {
        sqlx::query(r#"DELETE FROM nouns WHERE id = $1"#)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Get children of a noun
    pub async fn get_children(pool: &PgPool, parent_id: Uuid) -> Result<Vec<Noun>> {
        let nouns = sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE parent_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await?;

        Ok(nouns)
    }
}
