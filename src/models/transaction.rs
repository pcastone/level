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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_all() {
        assert_eq!(Verb::all().len(), 12);
    }

    #[test]
    fn test_verb_from_str() {
        assert_eq!(Verb::from_str("open"), Some(Verb::Open));
        assert_eq!(Verb::from_str("Open"), Some(Verb::Open));
        assert_eq!(Verb::from_str("complete"), Some(Verb::Complete));
        assert_eq!(Verb::from_str("incomplete"), Some(Verb::Incomplete));
        assert_eq!(Verb::from_str("normal"), Some(Verb::Normal));
        assert_eq!(Verb::from_str("escalate"), Some(Verb::Escalate));
        assert_eq!(Verb::from_str("close"), Some(Verb::Close));
        assert_eq!(Verb::from_str("update"), Some(Verb::Update));
        assert_eq!(Verb::from_str("reparent"), Some(Verb::Reparent));
        assert_eq!(Verb::from_str("assign"), Some(Verb::Assign));
        assert_eq!(Verb::from_str("unassign"), Some(Verb::Unassign));
        assert_eq!(Verb::from_str("blocked"), Some(Verb::Blocked));
        assert_eq!(Verb::from_str("release"), Some(Verb::Release));
        assert_eq!(Verb::from_str("invalid"), None);
    }

    #[test]
    fn test_verb_changes_state() {
        assert!(Verb::Complete.changes_state());
        assert!(Verb::Incomplete.changes_state());
        assert!(Verb::Normal.changes_state());
        assert!(Verb::Escalate.changes_state());
        assert!(Verb::Close.changes_state());
        assert!(!Verb::Open.changes_state());
        assert!(!Verb::Update.changes_state());
        assert!(!Verb::Assign.changes_state());
    }

    #[test]
    fn test_verb_modifies_relationships() {
        assert!(Verb::Assign.modifies_relationships());
        assert!(Verb::Unassign.modifies_relationships());
        assert!(Verb::Blocked.modifies_relationships());
        assert!(Verb::Release.modifies_relationships());
        assert!(Verb::Reparent.modifies_relationships());
        assert!(!Verb::Open.modifies_relationships());
        assert!(!Verb::Complete.modifies_relationships());
        assert!(!Verb::Update.modifies_relationships());
    }

    #[test]
    fn test_verb_display() {
        assert_eq!(Verb::Open.to_string(), "Open");
        assert_eq!(Verb::Complete.to_string(), "Complete");
        assert_eq!(Verb::Blocked.to_string(), "Blocked");
    }

    #[test]
    fn test_transaction_new() {
        let noun_id = Uuid::new_v4();
        let sow_id = Uuid::new_v4();
        let before = serde_json::json!({"state": "Normal"});
        let after = serde_json::json!({"state": "Completed"});

        let txn = Transaction::new(
            noun_id,
            sow_id,
            1,
            Verb::Complete,
            "+user".to_string(),
            before.clone(),
            after.clone(),
        );

        assert_eq!(txn.noun_id, noun_id);
        assert_eq!(txn.sow_id, sow_id);
        assert_eq!(txn.sequence, 1);
        assert_eq!(txn.get_verb(), Verb::Complete);
        assert_eq!(txn.actor, "+user");
        assert_eq!(txn.before_snapshot, before);
        assert_eq!(txn.after_snapshot, after);
    }

    #[test]
    fn test_transaction_with_context() {
        let noun_id = Uuid::new_v4();
        let sow_id = Uuid::new_v4();
        let context = serde_json::json!({"target_id": "some-uuid"});

        let txn = Transaction::new(
            noun_id,
            sow_id,
            1,
            Verb::Blocked,
            "+user".to_string(),
            serde_json::json!({}),
            serde_json::json!({}),
        )
        .with_context(context.clone());

        assert_eq!(txn.context, context);
    }

    #[test]
    fn test_transaction_get_verb_fallback() {
        let noun_id = Uuid::new_v4();
        let sow_id = Uuid::new_v4();

        let mut txn = Transaction::new(
            noun_id,
            sow_id,
            1,
            Verb::Complete,
            "+user".to_string(),
            serde_json::json!({}),
            serde_json::json!({}),
        );

        // Set invalid verb string
        txn.verb = "invalid".to_string();
        // Should fallback to Update
        assert_eq!(txn.get_verb(), Verb::Update);
    }
}
