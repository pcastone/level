//! Auxiliary models - Alias, Instruction, UserPreferences, etc.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Alias - user-defined shortcuts to nouns
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Alias {
    pub id: Uuid,
    pub actor: String, // Owner of the alias
    pub name: String,  // Alias name (without * prefix)
    pub noun_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl Alias {
    pub fn new(actor: String, name: String, noun_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor,
            name: name.trim_start_matches('*').to_string(),
            noun_id,
            created_at: Utc::now(),
        }
    }
}

/// Instruction - operational context documents
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Instruction {
    pub id: Uuid,
    pub scope: InstructionScope,
    pub sow_id: Option<Uuid>,
    pub category: String,
    pub title: String,
    pub content: String,
    pub applies_to: Vec<String>, // Noun types this applies to
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "instruction_scope", rename_all = "snake_case")]
pub enum InstructionScope {
    Global,
    Sow,
}

impl Default for InstructionScope {
    fn default() -> Self {
        InstructionScope::Global
    }
}

impl Instruction {
    pub fn new(
        scope: InstructionScope,
        sow_id: Option<Uuid>,
        category: String,
        title: String,
        content: String,
        created_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            scope,
            sow_id,
            category,
            title,
            content,
            applies_to: vec![],
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}

/// UserPreferences - user settings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub id: Uuid,
    pub actor: String,
    pub default_sow: Option<Uuid>,
    pub timezone: String,
    pub theme: String,
    pub date_format: String,
    pub notifications: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserPreferences {
    pub fn new(actor: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            actor,
            default_sow: None,
            timezone: "UTC".to_string(),
            theme: "light".to_string(),
            date_format: "YYYY-MM-DD".to_string(),
            notifications: serde_json::json!({
                "email": true,
                "in_app": true
            }),
            created_at: now,
            updated_at: now,
        }
    }
}

/// SOWDatabase - federation routing
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SowDatabase {
    pub id: Uuid,
    pub sow_id: Uuid,
    pub database_url: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SowDatabase {
    pub fn new(sow_id: Uuid, database_url: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            sow_id,
            database_url,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

/// NounSequence - per-SOW+type sequence for short_name generation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NounSequence {
    pub id: Uuid,
    pub sow_id: Uuid,
    pub noun_type: String,
    pub next_value: i32,
}

impl NounSequence {
    pub fn new(sow_id: Uuid, noun_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            sow_id,
            noun_type,
            next_value: 1,
        }
    }
}

/// Request types
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAliasRequest {
    pub name: String,
    pub noun_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstructionRequest {
    pub scope: InstructionScope,
    #[serde(default)]
    pub sow_id: Option<Uuid>,
    pub category: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub applies_to: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePreferencesRequest {
    #[serde(default)]
    pub default_sow: Option<Uuid>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub date_format: Option<String>,
    #[serde(default)]
    pub notifications: Option<serde_json::Value>,
}
