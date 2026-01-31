//! Blocking propagation service

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;

/// Blocking propagation service
pub struct BlockingService;

impl BlockingService {
    /// Propagate block downward to target and all children
    pub async fn propagate_block_downward(
        pool: &PgPool,
        _blocker_id: Uuid,
        target_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let mut affected = vec![target_id];

        // Get all descendants
        let descendants: Vec<(Uuid,)> = sqlx::query_as(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id FROM nouns WHERE parent_id = $1
                UNION ALL
                SELECT n.id FROM nouns n
                INNER JOIN descendants d ON n.parent_id = d.id
            )
            SELECT id FROM descendants
            "#,
        )
        .bind(target_id)
        .fetch_all(pool)
        .await?;

        affected.extend(descendants.into_iter().map(|(id,)| id));

        // Update is_blocked for all affected nouns
        sqlx::query(
            r#"
            UPDATE nouns SET is_blocked = true, updated_at = NOW()
            WHERE id = ANY($1)
            "#,
        )
        .bind(&affected)
        .execute(pool)
        .await?;

        Ok(affected)
    }

    /// Recompute is_blocked for a noun based on NounBlock relationships
    pub async fn compute_is_blocked(pool: &PgPool, noun_id: Uuid) -> Result<bool> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM noun_blocks
            WHERE target_id = $1
            AND EXISTS (
                SELECT 1 FROM nouns WHERE id = noun_blocks.blocker_id
                AND state NOT IN ('Completed', 'Incompleted', 'Closed', 'Archived')
            )
            "#,
        )
        .bind(noun_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0 > 0)
    }

    /// Update is_blocked for noun and propagate upward
    pub async fn update_blocked_status(pool: &PgPool, noun_id: Uuid) -> Result<()> {
        let is_blocked = Self::compute_is_blocked(pool, noun_id).await?;

        sqlx::query(
            r#"
            UPDATE nouns SET is_blocked = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(is_blocked)
        .bind(noun_id)
        .execute(pool)
        .await?;

        // Propagate upward - update parent's is_blocked if any child is blocked
        Self::propagate_blocked_upward(pool, noun_id).await?;

        Ok(())
    }

    /// Propagate is_blocked status upward to parent
    pub async fn propagate_blocked_upward(pool: &PgPool, noun_id: Uuid) -> Result<()> {
        // Get parent_id
        let parent: Option<(Option<Uuid>,)> = sqlx::query_as(
            r#"SELECT parent_id FROM nouns WHERE id = $1"#,
        )
        .bind(noun_id)
        .fetch_optional(pool)
        .await?;

        if let Some((Some(parent_id),)) = parent {
            // Check if any child of parent is blocked
            let any_blocked: (bool,) = sqlx::query_as(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM nouns
                    WHERE parent_id = $1 AND is_blocked = true
                )
                "#,
            )
            .bind(parent_id)
            .fetch_one(pool)
            .await?;

            sqlx::query(
                r#"
                UPDATE nouns SET is_blocked = $1, updated_at = NOW()
                WHERE id = $2
                "#,
            )
            .bind(any_blocked.0)
            .bind(parent_id)
            .execute(pool)
            .await?;

            // Recurse upward
            Box::pin(Self::propagate_blocked_upward(pool, parent_id)).await?;
        }

        Ok(())
    }

    /// Handle blocking for Project Goals (special case)
    /// Goals increment blocked_goals counter but do NOT set is_blocked on Project
    pub async fn update_project_blocked_goals(pool: &PgPool, project_id: Uuid) -> Result<()> {
        // Get goal_noun_ids from custom_fields
        let project: Option<(serde_json::Value,)> = sqlx::query_as(
            r#"SELECT custom_fields FROM nouns WHERE id = $1 AND type = 'project'"#,
        )
        .bind(project_id)
        .fetch_optional(pool)
        .await?;

        if let Some((custom_fields,)) = project {
            if let Some(goal_ids) = custom_fields.get("goal_noun_ids").and_then(|v| v.as_array()) {
                let goal_uuids: Vec<Uuid> = goal_ids
                    .iter()
                    .filter_map(|v| v.as_str())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect();

                if !goal_uuids.is_empty() {
                    // Count blocked goals
                    let blocked_count: (i64,) = sqlx::query_as(
                        r#"
                        SELECT COUNT(*) FROM nouns
                        WHERE id = ANY($1) AND is_blocked = true
                        "#,
                    )
                    .bind(&goal_uuids)
                    .fetch_one(pool)
                    .await?;

                    // Update blocked_goals in custom_fields
                    sqlx::query(
                        r#"
                        UPDATE nouns
                        SET custom_fields = jsonb_set(custom_fields, '{blocked_goals}', $1::jsonb),
                            updated_at = NOW()
                        WHERE id = $2
                        "#,
                    )
                    .bind(serde_json::json!(blocked_count.0))
                    .bind(project_id)
                    .execute(pool)
                    .await?;
                }
            }
        }

        Ok(())
    }

    /// Release block and recompute affected nouns
    pub async fn release_block(
        pool: &PgPool,
        blocker_id: Uuid,
        target_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        // Delete the block relationship
        sqlx::query(
            r#"DELETE FROM noun_blocks WHERE blocker_id = $1 AND target_id = $2"#,
        )
        .bind(blocker_id)
        .bind(target_id)
        .execute(pool)
        .await?;

        // Get all affected nouns (target and descendants)
        let mut affected = vec![target_id];
        let descendants: Vec<(Uuid,)> = sqlx::query_as(
            r#"
            WITH RECURSIVE descendants AS (
                SELECT id FROM nouns WHERE parent_id = $1
                UNION ALL
                SELECT n.id FROM nouns n
                INNER JOIN descendants d ON n.parent_id = d.id
            )
            SELECT id FROM descendants
            "#,
        )
        .bind(target_id)
        .fetch_all(pool)
        .await?;

        affected.extend(descendants.into_iter().map(|(id,)| id));

        // Recompute is_blocked for each affected noun
        for noun_id in &affected {
            Self::update_blocked_status(pool, *noun_id).await?;
        }

        Ok(affected)
    }
}
