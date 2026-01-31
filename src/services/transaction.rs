//! Transaction service for event sourcing

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Result;
use crate::models::{Noun, Transaction, Verb};

/// Transaction service for event sourcing
pub struct TransactionService;

impl TransactionService {
    /// Create a transaction record
    pub async fn create(
        pool: &PgPool,
        noun: &Noun,
        verb: Verb,
        actor: &str,
        before: serde_json::Value,
        after: serde_json::Value,
        context: Option<serde_json::Value>,
    ) -> Result<Transaction> {
        // Get next sequence number for this noun
        let sequence = Self::next_sequence(pool, noun.id).await?;

        let transaction = Transaction::new(
            noun.id,
            noun.sow_id,
            sequence,
            verb,
            actor.to_string(),
            before,
            after,
        );

        let transaction = if let Some(ctx) = context {
            transaction.with_context(ctx)
        } else {
            transaction
        };

        sqlx::query(
            r#"
            INSERT INTO transactions (id, noun_id, sow_id, sequence, verb, actor, before_snapshot, after_snapshot, context, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(transaction.id)
        .bind(transaction.noun_id)
        .bind(transaction.sow_id)
        .bind(transaction.sequence)
        .bind(&transaction.verb.to_string())
        .bind(&transaction.actor)
        .bind(&transaction.before_snapshot)
        .bind(&transaction.after_snapshot)
        .bind(&transaction.context)
        .bind(transaction.created_at)
        .execute(pool)
        .await?;

        Ok(transaction)
    }

    /// Get next sequence number for a noun
    async fn next_sequence(pool: &PgPool, noun_id: Uuid) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COALESCE(MAX(sequence), 0) + 1
            FROM transactions
            WHERE noun_id = $1
            "#,
        )
        .bind(noun_id)
        .fetch_one(pool)
        .await?;

        Ok(result.0)
    }

    /// Get transactions for a noun
    pub async fn get_for_noun(
        pool: &PgPool,
        noun_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as::<_, Transaction>(
            r#"
            SELECT id, noun_id, sow_id, sequence, verb, actor, before_snapshot, after_snapshot, context, created_at
            FROM transactions
            WHERE noun_id = $1
            ORDER BY sequence DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(noun_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }

    /// Get transactions for a SOW
    pub async fn get_for_sow(
        pool: &PgPool,
        sow_id: Uuid,
        limit: i64,
        offset: i64,
        actor_filter: Option<&str>,
    ) -> Result<Vec<Transaction>> {
        let transactions = if let Some(actor) = actor_filter {
            sqlx::query_as::<_, Transaction>(
                r#"
                SELECT id, noun_id, sow_id, sequence, verb, actor, before_snapshot, after_snapshot, context, created_at
                FROM transactions
                WHERE sow_id = $1 AND actor = $4
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(sow_id)
            .bind(limit)
            .bind(offset)
            .bind(actor)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, Transaction>(
                r#"
                SELECT id, noun_id, sow_id, sequence, verb, actor, before_snapshot, after_snapshot, context, created_at
                FROM transactions
                WHERE sow_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(sow_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok(transactions)
    }

    /// Create snapshot of noun state
    pub fn create_snapshot(noun: &Noun) -> serde_json::Value {
        serde_json::json!({
            "id": noun.id.to_string(),
            "type": noun.noun_type.to_string(),
            "sow_id": noun.sow_id.to_string(),
            "parent_id": noun.parent_id.map(|id| id.to_string()),
            "short_name": noun.short_name,
            "title": noun.title,
            "description": noun.description,
            "state": noun.state.to_string(),
            "is_blocked": noun.is_blocked,
            "due_date": noun.due_date,
            "completed_at": noun.completed_at,
            "closed_at": noun.closed_at,
            "custom_fields": noun.custom_fields,
        })
    }
}
