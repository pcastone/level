//! Transaction model for event sourcing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

/// 12 verbs in Level system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Verb {
    Open,
    Complete,
    Incomplete,
    Normal,
    Escalate,
    Close,
    Update,
    Reparent,
    Assign,
    Unassign,
    Blocked,
    Release,
}

impl Verb {
    /// Get all verbs
    pub fn all() -> &'static [Verb] {
        &[
            Verb::Open,
            Verb::Complete,
            Verb::Incomplete,
            Verb::Normal,
            Verb::Escalate,
            Verb::Close,
            Verb::Update,
            Verb::Reparent,
            Verb::Assign,
            Verb::Unassign,
            Verb::Blocked,
            Verb::Release,
        ]
    }

    /// Check if this verb changes state
    pub fn changes_state(&self) -> bool {
        matches!(
            self,
            Verb::Complete | Verb::Incomplete | Verb::Normal | Verb::Escalate | Verb::Close
        )
    }

    /// Check if this verb modifies relationships
    pub fn modifies_relationships(&self) -> bool {
        matches!(
            self,
            Verb::Assign | Verb::Unassign | Verb::Blocked | Verb::Release | Verb::Reparent
        )
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "open" => Some(Verb::Open),
            "complete" => Some(Verb::Complete),
            "incomplete" => Some(Verb::Incomplete),
            "normal" => Some(Verb::Normal),
            "escalate" => Some(Verb::Escalate),
            "close" => Some(Verb::Close),
            "update" => Some(Verb::Update),
            "reparent" => Some(Verb::Reparent),
            "assign" => Some(Verb::Assign),
            "unassign" => Some(Verb::Unassign),
            "blocked" => Some(Verb::Blocked),
            "release" => Some(Verb::Release),
            _ => None,
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verb::Open => write!(f, "Open"),
            Verb::Complete => write!(f, "Complete"),
            Verb::Incomplete => write!(f, "Incomplete"),
            Verb::Normal => write!(f, "Normal"),
            Verb::Escalate => write!(f, "Escalate"),
            Verb::Close => write!(f, "Close"),
            Verb::Update => write!(f, "Update"),
            Verb::Reparent => write!(f, "Reparent"),
            Verb::Assign => write!(f, "Assign"),
            Verb::Unassign => write!(f, "Unassign"),
            Verb::Blocked => write!(f, "Blocked"),
            Verb::Release => write!(f, "Release"),
        }
    }
}

/// Transaction record for event sourcing
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub noun_id: Uuid,
    pub sow_id: Uuid,
    pub sequence: i64,
    pub verb: String,
    pub actor: String,
    pub before_snapshot: serde_json::Value,
    pub after_snapshot: serde_json::Value,
    pub context: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        noun_id: Uuid,
        sow_id: Uuid,
        sequence: i64,
        verb: Verb,
        actor: String,
        before_snapshot: serde_json::Value,
        after_snapshot: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            noun_id,
            sow_id,
            sequence,
            verb: verb.to_string(),
            actor,
            before_snapshot,
            after_snapshot,
            context: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }

    /// Get the parsed Verb
    pub fn get_verb(&self) -> Verb {
        Verb::from_str(&self.verb).unwrap_or(Verb::Update)
    }

    /// Create transaction with context
    pub fn with_context(mut self, context: serde_json::Value) -> Self {
        self.context = context;
        self
    }
}

/// Request to apply a verb action
#[derive(Debug, Clone, Deserialize)]
pub struct VerbRequest {
    pub verb: Verb,
    #[serde(default)]
    pub context: Option<serde_json::Value>,
}

/// Batch verb request for multiple nouns
#[derive(Debug, Clone, Deserialize)]
pub struct BatchVerbRequest {
    pub noun_ids: Vec<Uuid>,
    pub verb: Verb,
    #[serde(default)]
    pub context: Option<serde_json::Value>,
}

/// Transaction list response
#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<Transaction>,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
