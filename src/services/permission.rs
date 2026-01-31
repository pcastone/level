//! Permission evaluation service

use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use crate::error::{LevelError, Result};
use crate::models::{SowRoles, Verb};

/// Permission evaluation service
pub struct PermissionService;

impl PermissionService {
    /// Evaluate permissions for an actor on a SOW
    pub async fn evaluate(
        pool: &PgPool,
        actor: &str,
        sow_id: Uuid,
    ) -> Result<HashSet<Verb>> {
        // 1. Resolve actor's groups
        let groups = Self::resolve_actor_groups(pool, actor).await?;

        // 2. Get all actor strings (person + groups)
        let all_actors: Vec<String> = std::iter::once(actor.to_string())
            .chain(groups.clone())
            .collect();

        // 3. Get role mappings for all actors
        let mut allowed_verbs: HashSet<Verb> = HashSet::new();

        for actor_str in &all_actors {
            let verbs = Self::get_role_verbs(pool, actor_str, Some(sow_id)).await?;
            allowed_verbs.extend(verbs);
        }

        // 4. Apply SOW-level allow/deny rules
        let sow_roles = Self::get_sow_roles(pool, sow_id).await?;
        let group_strs: Vec<String> = groups.iter().cloned().collect();
        let allowed_by_sow = sow_roles.allowed_verbs(actor, &group_strs);

        // Filter by SOW rules if they exist
        if !sow_roles.allow.is_empty() {
            allowed_verbs.retain(|v| allowed_by_sow.contains(&v.to_string()));
        }

        Ok(allowed_verbs)
    }

    /// Check if actor has specific verb permission
    pub async fn can_perform(
        pool: &PgPool,
        actor: &str,
        sow_id: Uuid,
        verb: Verb,
    ) -> Result<bool> {
        let allowed = Self::evaluate(pool, actor, sow_id).await?;
        Ok(allowed.contains(&verb))
    }

    /// Require permission or return error
    pub async fn require(
        pool: &PgPool,
        actor: &str,
        sow_id: Uuid,
        verb: Verb,
    ) -> Result<()> {
        if Self::can_perform(pool, actor, sow_id, verb).await? {
            Ok(())
        } else {
            Err(LevelError::PermissionDenied(format!(
                "Actor {} cannot perform {} on this SOW",
                actor, verb
            )))
        }
    }

    /// Resolve groups for an actor (person)
    async fn resolve_actor_groups(pool: &PgPool, actor: &str) -> Result<Vec<String>> {
        // Only resolve groups for person actors (+prefix)
        if !actor.starts_with('+') {
            return Ok(vec![]);
        }

        let username = &actor[1..];

        let groups: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT g.name
            FROM groups g
            INNER JOIN group_members gm ON gm.group_id = g.id
            INNER JOIN persons p ON p.id = gm.person_id
            WHERE p.username = $1 AND g.active = true
            "#,
        )
        .bind(username)
        .fetch_all(pool)
        .await?;

        Ok(groups.into_iter().map(|(name,)| format!("@{}", name)).collect())
    }

    /// Get verbs from role mappings for an actor
    async fn get_role_verbs(
        pool: &PgPool,
        actor: &str,
        sow_id: Option<Uuid>,
    ) -> Result<HashSet<Verb>> {
        let verbs_arrays: Vec<(Vec<String>,)> = if let Some(sow) = sow_id {
            sqlx::query_as(
                r#"
                SELECT r.verbs
                FROM roles r
                INNER JOIN role_mappings rm ON rm.role_id = r.id
                WHERE rm.actor = $1 AND (rm.sow_id IS NULL OR rm.sow_id = $2)
                "#,
            )
            .bind(actor)
            .bind(sow)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as(
                r#"
                SELECT r.verbs
                FROM roles r
                INNER JOIN role_mappings rm ON rm.role_id = r.id
                WHERE rm.actor = $1 AND rm.sow_id IS NULL
                "#,
            )
            .bind(actor)
            .fetch_all(pool)
            .await?
        };

        let mut verbs = HashSet::new();
        for (verb_strs,) in verbs_arrays {
            for verb_str in verb_strs {
                if let Some(verb) = Verb::from_str(&verb_str) {
                    verbs.insert(verb);
                }
            }
        }

        Ok(verbs)
    }

    /// Get SOW-level roles (allow/deny rules)
    async fn get_sow_roles(pool: &PgPool, sow_id: Uuid) -> Result<SowRoles> {
        let result: Option<(serde_json::Value,)> = sqlx::query_as(
            r#"
            SELECT custom_fields->'roles'
            FROM nouns
            WHERE id = $1 AND type = 'sow'
            "#,
        )
        .bind(sow_id)
        .fetch_optional(pool)
        .await?;

        if let Some((roles_json,)) = result {
            let roles: SowRoles = serde_json::from_value(roles_json).unwrap_or_default();
            Ok(roles)
        } else {
            Ok(SowRoles::default())
        }
    }

    /// Check if actor has any access to SOW
    pub async fn has_sow_access(pool: &PgPool, actor: &str, sow_id: Uuid) -> Result<bool> {
        let sow_roles = Self::get_sow_roles(pool, sow_id).await?;
        let groups = Self::resolve_actor_groups(pool, actor).await?;

        // If no allow rules, check role mappings
        if sow_roles.allow.is_empty() {
            let has_mapping: (bool,) = sqlx::query_as(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM role_mappings
                    WHERE actor = $1 AND (sow_id IS NULL OR sow_id = $2)
                )
                "#,
            )
            .bind(actor)
            .bind(sow_id)
            .fetch_one(pool)
            .await?;

            return Ok(has_mapping.0);
        }

        Ok(sow_roles.has_access(actor, &groups))
    }
}
