//! Noun entity - core domain model with 12 types and 6 states

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

/// 12 noun types in Level system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NounType {
    Sow,
    Item,
    Task,
    Request,
    Meeting,
    Deliverable,
    Event,
    Blocker,
    Artifact,
    Group,
    Project,
    MileStone,
}

impl NounType {
    /// Get all 12 noun types
    pub fn all() -> &'static [NounType] {
        &[
            NounType::Sow,
            NounType::Item,
            NounType::Task,
            NounType::Request,
            NounType::Meeting,
            NounType::Deliverable,
            NounType::Event,
            NounType::Blocker,
            NounType::Artifact,
            NounType::Group,
            NounType::Project,
            NounType::MileStone,
        ]
    }

    /// Get abbreviation for short_name generation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            NounType::Sow => "SOW",
            NounType::Item => "ITEM",
            NounType::Task => "TASK",
            NounType::Request => "REQ",
            NounType::Meeting => "MEET",
            NounType::Deliverable => "DELIV",
            NounType::Event => "EVT",
            NounType::Blocker => "BLOCK",
            NounType::Artifact => "ART",
            NounType::Group => "GRP",
            NounType::Project => "PROJ",
            NounType::MileStone => "MILE",
        }
    }

    /// Check if this type is a container (can have assignments)
    pub fn is_container(&self) -> bool {
        matches!(self, NounType::Group | NounType::Project | NounType::MileStone)
    }

    /// Check if this type is leaf-only (cannot have children)
    pub fn is_leaf_only(&self) -> bool {
        matches!(self, NounType::Artifact)
    }

    /// Check if this type can be a Goal (for Projects)
    pub fn can_be_goal(&self) -> bool {
        matches!(self, NounType::Deliverable | NounType::Event)
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sow" => Some(NounType::Sow),
            "item" => Some(NounType::Item),
            "task" => Some(NounType::Task),
            "request" => Some(NounType::Request),
            "meeting" => Some(NounType::Meeting),
            "deliverable" => Some(NounType::Deliverable),
            "event" => Some(NounType::Event),
            "blocker" => Some(NounType::Blocker),
            "artifact" => Some(NounType::Artifact),
            "group" => Some(NounType::Group),
            "project" => Some(NounType::Project),
            "milestone" => Some(NounType::MileStone),
            _ => None,
        }
    }
}

impl fmt::Display for NounType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NounType::Sow => write!(f, "SOW"),
            NounType::Item => write!(f, "Item"),
            NounType::Task => write!(f, "Task"),
            NounType::Request => write!(f, "Request"),
            NounType::Meeting => write!(f, "Meeting"),
            NounType::Deliverable => write!(f, "Deliverable"),
            NounType::Event => write!(f, "Event"),
            NounType::Blocker => write!(f, "Blocker"),
            NounType::Artifact => write!(f, "Artifact"),
            NounType::Group => write!(f, "Group"),
            NounType::Project => write!(f, "Project"),
            NounType::MileStone => write!(f, "MileStone"),
        }
    }
}

/// 6 noun states - state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NounState {
    Normal,
    Escalated,
    Completed,
    Incompleted,
    Closed,
    Archived,
}

impl NounState {
    /// Get all states
    pub fn all() -> &'static [NounState] {
        &[
            NounState::Normal,
            NounState::Escalated,
            NounState::Completed,
            NounState::Incompleted,
            NounState::Closed,
            NounState::Archived,
        ]
    }

    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, NounState::Archived)
    }

    /// Check if completion verbs can be applied
    pub fn can_complete(&self) -> bool {
        matches!(self, NounState::Normal | NounState::Escalated)
    }

    /// Check if Close verb can be applied
    pub fn can_close(&self) -> bool {
        matches!(self, NounState::Completed | NounState::Incompleted)
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "normal" => Some(NounState::Normal),
            "escalated" => Some(NounState::Escalated),
            "completed" => Some(NounState::Completed),
            "incompleted" => Some(NounState::Incompleted),
            "closed" => Some(NounState::Closed),
            "archived" => Some(NounState::Archived),
            _ => None,
        }
    }
}

impl Default for NounState {
    fn default() -> Self {
        NounState::Normal
    }
}

impl fmt::Display for NounState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NounState::Normal => write!(f, "Normal"),
            NounState::Escalated => write!(f, "Escalated"),
            NounState::Completed => write!(f, "Completed"),
            NounState::Incompleted => write!(f, "Incompleted"),
            NounState::Closed => write!(f, "Closed"),
            NounState::Archived => write!(f, "Archived"),
        }
    }
}

/// Core Noun entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Noun {
    pub id: Uuid,
    #[sqlx(rename = "noun_type")]
    #[serde(rename = "type")]
    pub noun_type: String,
    pub sow_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub short_name: String,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub is_blocked: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub custom_fields: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Noun {
    /// Create a new Noun with required fields
    pub fn new(
        noun_type: NounType,
        sow_id: Uuid,
        short_name: String,
        title: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            noun_type: noun_type.to_string().to_lowercase(),
            sow_id,
            parent_id: None,
            short_name,
            title,
            description: None,
            state: NounState::Normal.to_string(),
            is_blocked: false,
            due_date: None,
            completed_at: None,
            closed_at: None,
            custom_fields: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the parsed NounType
    pub fn get_type(&self) -> NounType {
        NounType::from_str(&self.noun_type).unwrap_or(NounType::Item)
    }

    /// Get the parsed NounState
    pub fn get_state(&self) -> NounState {
        NounState::from_str(&self.state).unwrap_or(NounState::Normal)
    }

    /// Check if this noun is a SOW
    pub fn is_sow(&self) -> bool {
        self.get_type() == NounType::Sow
    }

    /// Check if this noun is a container
    pub fn is_container(&self) -> bool {
        self.get_type().is_container()
    }

    /// Get blocked_goals count from custom_fields (for Projects)
    pub fn blocked_goals(&self) -> i32 {
        self.custom_fields
            .get("blocked_goals")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32
    }

    /// Get goal_noun_ids from custom_fields (for Projects)
    pub fn goal_noun_ids(&self) -> Vec<Uuid> {
        self.custom_fields
            .get("goal_noun_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if auto_complete is enabled (for MileStones)
    pub fn auto_complete(&self) -> bool {
        self.custom_fields
            .get("auto_complete")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

/// Request to create a new noun
#[derive(Debug, Clone, Deserialize)]
pub struct CreateNounRequest {
    #[serde(rename = "type")]
    pub noun_type: NounType,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parent_id: Option<Uuid>,
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub custom_fields: Option<serde_json::Value>,
}

/// Request to update a noun
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNounRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub custom_fields: Option<serde_json::Value>,
}

/// Noun list response with pagination
#[derive(Debug, Serialize)]
pub struct NounListResponse {
    pub nouns: Vec<Noun>,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
