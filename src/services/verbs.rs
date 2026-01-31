//! Verb handlers for all 12 verbs

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{LevelError, Result};
use crate::models::{
    CreateNounRequest, Noun, NounAssignment, NounBlock, NounState, NounType, UpdateNounRequest, Verb,
};

use super::{BlockingService, ShortNameService, StateMachine, TransactionService};

/// Verb handler service
pub struct VerbHandler;

impl VerbHandler {
    /// Open - create new noun
    pub async fn open(
        pool: &PgPool,
        sow_id: Uuid,
        request: CreateNounRequest,
        actor: &str,
    ) -> Result<Noun> {
        // Validate Artifact cannot have children
        if request.noun_type == NounType::Artifact && request.parent_id.is_some() {
            return Err(LevelError::ArtifactCannotHaveChildren);
        }

        // Get SOW short_name
        let sow: (String,) = sqlx::query_as(
            r#"SELECT short_name FROM nouns WHERE id = $1 AND type = 'sow'"#,
        )
        .bind(sow_id)
        .fetch_one(pool)
        .await
        .map_err(|_| LevelError::SowNotFound(sow_id.to_string()))?;

        // Generate short_name
        let short_name = ShortNameService::generate(pool, sow_id, &sow.0, request.noun_type).await?;

        // Create noun
        let now = Utc::now();
        let noun_id = Uuid::new_v4();

        let custom_fields = request.custom_fields.unwrap_or(serde_json::json!({}));

        sqlx::query(
            r#"
            INSERT INTO nouns (id, type, sow_id, parent_id, short_name, title, description, state, is_blocked, due_date, custom_fields, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
        )
        .bind(noun_id)
        .bind(request.noun_type.to_string())
        .bind(sow_id)
        .bind(request.parent_id)
        .bind(&short_name)
        .bind(&request.title)
        .bind(&request.description)
        .bind(NounState::Normal.to_string())
        .bind(false)
        .bind(request.due_date)
        .bind(&custom_fields)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        // Fetch created noun
        let noun = Self::get_noun(pool, noun_id).await?;

        // Log transaction
        let after = TransactionService::create_snapshot(&noun);
        TransactionService::create(
            pool,
            &noun,
            Verb::Open,
            actor,
            serde_json::json!({}),
            after,
            None,
        )
        .await?;

        Ok(noun)
    }

    /// Complete - mark noun as completed
    pub async fn complete(pool: &PgPool, noun_id: Uuid, actor: &str) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        // Check guards
        if noun.is_blocked {
            return Err(LevelError::NounBlocked(noun.short_name.clone()));
        }

        if !StateMachine::can_complete(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: NounState::Completed.to_string(),
            });
        }

        // Update noun
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE nouns
            SET state = $1, completed_at = $2, updated_at = $3
            WHERE id = $4
            "#,
        )
        .bind(NounState::Completed.to_string())
        .bind(now)
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        // If this is a Blocker, release all targets
        if noun.get_type() == NounType::Blocker {
            Self::release_blocker_targets(pool, noun_id).await?;
        }

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Complete, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Incomplete - mark noun as incompleted
    pub async fn incomplete(pool: &PgPool, noun_id: Uuid, actor: &str) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        // Check guards
        if noun.is_blocked {
            return Err(LevelError::NounBlocked(noun.short_name.clone()));
        }

        if !StateMachine::can_incomplete(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: NounState::Incompleted.to_string(),
            });
        }

        let now = Utc::now();
        sqlx::query(
            r#"UPDATE nouns SET state = $1, updated_at = $2 WHERE id = $3"#,
        )
        .bind(NounState::Incompleted.to_string())
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Incomplete, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Normal - reset to normal state
    pub async fn normal(pool: &PgPool, noun_id: Uuid, actor: &str) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        if !StateMachine::can_normalize(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: NounState::Normal.to_string(),
            });
        }

        let now = Utc::now();
        sqlx::query(
            r#"UPDATE nouns SET state = $1, updated_at = $2 WHERE id = $3"#,
        )
        .bind(NounState::Normal.to_string())
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Normal, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Escalate - mark as escalated/urgent
    pub async fn escalate(pool: &PgPool, noun_id: Uuid, actor: &str) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        if !StateMachine::can_escalate(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: NounState::Escalated.to_string(),
            });
        }

        let now = Utc::now();
        sqlx::query(
            r#"UPDATE nouns SET state = $1, updated_at = $2 WHERE id = $3"#,
        )
        .bind(NounState::Escalated.to_string())
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Escalate, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Close - close a completed/incompleted noun
    pub async fn close(pool: &PgPool, noun_id: Uuid, actor: &str) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        if !StateMachine::can_close(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: NounState::Closed.to_string(),
            });
        }

        // Check all children are closed
        let open_children: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM nouns
            WHERE parent_id = $1 AND state NOT IN ('Closed', 'Archived')
            "#,
        )
        .bind(noun_id)
        .fetch_one(pool)
        .await?;

        if open_children.0 > 0 {
            return Err(LevelError::ChildrenNotClosed(noun.short_name.clone()));
        }

        let now = Utc::now();
        sqlx::query(
            r#"UPDATE nouns SET state = $1, closed_at = $2, updated_at = $3 WHERE id = $4"#,
        )
        .bind(NounState::Closed.to_string())
        .bind(now)
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Close, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Update - modify noun fields
    pub async fn update(
        pool: &PgPool,
        noun_id: Uuid,
        request: UpdateNounRequest,
        actor: &str,
    ) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        if !StateMachine::can_modify(noun.get_state()) {
            return Err(LevelError::InvalidStateTransition {
                from: noun.state.clone(),
                to: "Update".to_string(),
            });
        }

        let now = Utc::now();
        let title = request.title.unwrap_or(noun.title.clone());
        let description = request.description.or(noun.description.clone());
        let due_date = request.due_date.or(noun.due_date);
        let custom_fields = request.custom_fields.unwrap_or(noun.custom_fields.clone());

        sqlx::query(
            r#"
            UPDATE nouns
            SET title = $1, description = $2, due_date = $3, custom_fields = $4, updated_at = $5
            WHERE id = $6
            "#,
        )
        .bind(&title)
        .bind(&description)
        .bind(due_date)
        .bind(&custom_fields)
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        TransactionService::create(pool, &noun, Verb::Update, actor, before, after, None).await?;

        Ok(noun)
    }

    /// Reparent - move noun to new parent or SOW root
    pub async fn reparent(
        pool: &PgPool,
        noun_id: Uuid,
        new_parent_id: Option<Uuid>,
        actor: &str,
    ) -> Result<Noun> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        let now = Utc::now();
        sqlx::query(
            r#"UPDATE nouns SET parent_id = $1, updated_at = $2 WHERE id = $3"#,
        )
        .bind(new_parent_id)
        .bind(now)
        .bind(noun_id)
        .execute(pool)
        .await?;

        let noun = Self::get_noun(pool, noun_id).await?;
        let after = TransactionService::create_snapshot(&noun);

        let context = serde_json::json!({
            "old_parent_id": before.get("parent_id"),
            "new_parent_id": new_parent_id.map(|id| id.to_string()),
        });

        TransactionService::create(pool, &noun, Verb::Reparent, actor, before, after, Some(context)).await?;

        Ok(noun)
    }

    /// Assign - add noun to container
    pub async fn assign(
        pool: &PgPool,
        noun_id: Uuid,
        container_id: Uuid,
        sort_order: Option<i32>,
        actor: &str,
    ) -> Result<NounAssignment> {
        // Validate container type
        let container = Self::get_noun(pool, container_id).await?;
        if !container.get_type().is_container() {
            return Err(LevelError::InvalidContainerType(container.noun_type.clone()));
        }

        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        let order = sort_order.unwrap_or(0);
        let assignment = NounAssignment::new(noun_id, container_id, order);

        sqlx::query(
            r#"
            INSERT INTO noun_assignments (id, noun_id, container_id, sort_order, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (noun_id, container_id) DO UPDATE SET sort_order = $4
            "#,
        )
        .bind(assignment.id)
        .bind(assignment.noun_id)
        .bind(assignment.container_id)
        .bind(assignment.sort_order)
        .bind(assignment.created_at)
        .execute(pool)
        .await?;

        let after = TransactionService::create_snapshot(&noun);
        let context = serde_json::json!({
            "container_id": container_id.to_string(),
            "sort_order": order,
        });

        TransactionService::create(pool, &noun, Verb::Assign, actor, before, after, Some(context)).await?;

        Ok(assignment)
    }

    /// Unassign - remove noun from container
    pub async fn unassign(
        pool: &PgPool,
        noun_id: Uuid,
        container_id: Uuid,
        actor: &str,
    ) -> Result<()> {
        let noun = Self::get_noun(pool, noun_id).await?;
        let before = TransactionService::create_snapshot(&noun);

        sqlx::query(
            r#"DELETE FROM noun_assignments WHERE noun_id = $1 AND container_id = $2"#,
        )
        .bind(noun_id)
        .bind(container_id)
        .execute(pool)
        .await?;

        let after = TransactionService::create_snapshot(&noun);
        let context = serde_json::json!({
            "container_id": container_id.to_string(),
        });

        TransactionService::create(pool, &noun, Verb::Unassign, actor, before, after, Some(context)).await?;

        Ok(())
    }

    /// Blocked - create block relationship
    pub async fn blocked(
        pool: &PgPool,
        blocker_id: Uuid,
        target_id: Uuid,
        reason: Option<String>,
        actor: &str,
    ) -> Result<NounBlock> {
        // Validate blocker is of type Blocker
        let blocker = Self::get_noun(pool, blocker_id).await?;
        if blocker.get_type() != NounType::Blocker {
            return Err(LevelError::InvalidBlockerType(blocker.noun_type.clone()));
        }

        let target = Self::get_noun(pool, target_id).await?;
        let before = TransactionService::create_snapshot(&target);

        let block = NounBlock::new(blocker_id, target_id, reason.clone());

        sqlx::query(
            r#"
            INSERT INTO noun_blocks (id, blocker_id, target_id, reason, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(block.id)
        .bind(block.blocker_id)
        .bind(block.target_id)
        .bind(&block.reason)
        .bind(block.created_at)
        .execute(pool)
        .await?;

        // Propagate blocking
        BlockingService::propagate_block_downward(pool, blocker_id, target_id).await?;

        let target = Self::get_noun(pool, target_id).await?;
        let after = TransactionService::create_snapshot(&target);
        let context = serde_json::json!({
            "blocker_id": blocker_id.to_string(),
            "reason": reason,
        });

        TransactionService::create(pool, &target, Verb::Blocked, actor, before, after, Some(context)).await?;

        Ok(block)
    }

    /// Release - remove block relationship
    pub async fn release(
        pool: &PgPool,
        blocker_id: Uuid,
        target_id: Uuid,
        actor: &str,
    ) -> Result<()> {
        let target = Self::get_noun(pool, target_id).await?;
        let before = TransactionService::create_snapshot(&target);

        BlockingService::release_block(pool, blocker_id, target_id).await?;

        let target = Self::get_noun(pool, target_id).await?;
        let after = TransactionService::create_snapshot(&target);
        let context = serde_json::json!({
            "blocker_id": blocker_id.to_string(),
        });

        TransactionService::create(pool, &target, Verb::Release, actor, before, after, Some(context)).await?;

        Ok(())
    }

    /// Helper: Get noun by ID
    async fn get_noun(pool: &PgPool, noun_id: Uuid) -> Result<Noun> {
        sqlx::query_as::<_, Noun>(
            r#"
            SELECT id, type as noun_type, sow_id, parent_id, short_name, title, description,
                   state, is_blocked, due_date, completed_at, closed_at, custom_fields, created_at, updated_at
            FROM nouns WHERE id = $1
            "#,
        )
        .bind(noun_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| LevelError::NounNotFound(noun_id.to_string()))
    }

    /// Helper: Release all targets when blocker is completed
    async fn release_blocker_targets(pool: &PgPool, blocker_id: Uuid) -> Result<()> {
        let targets: Vec<(Uuid,)> = sqlx::query_as(
            r#"SELECT target_id FROM noun_blocks WHERE blocker_id = $1"#,
        )
        .bind(blocker_id)
        .fetch_all(pool)
        .await?;

        for (target_id,) in targets {
            BlockingService::release_block(pool, blocker_id, target_id).await?;
        }

        Ok(())
    }
}
