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

#[cfg(test)]
mod tests {
    use super::*;

    // NounType tests
    #[test]
    fn test_noun_type_all() {
        assert_eq!(NounType::all().len(), 12);
    }

    #[test]
    fn test_noun_type_from_str() {
        assert_eq!(NounType::from_str("sow"), Some(NounType::Sow));
        assert_eq!(NounType::from_str("SOW"), Some(NounType::Sow));
        assert_eq!(NounType::from_str("task"), Some(NounType::Task));
        assert_eq!(NounType::from_str("Task"), Some(NounType::Task));
        assert_eq!(NounType::from_str("request"), Some(NounType::Request));
        assert_eq!(NounType::from_str("meeting"), Some(NounType::Meeting));
        assert_eq!(NounType::from_str("deliverable"), Some(NounType::Deliverable));
        assert_eq!(NounType::from_str("event"), Some(NounType::Event));
        assert_eq!(NounType::from_str("blocker"), Some(NounType::Blocker));
        assert_eq!(NounType::from_str("artifact"), Some(NounType::Artifact));
        assert_eq!(NounType::from_str("group"), Some(NounType::Group));
        assert_eq!(NounType::from_str("project"), Some(NounType::Project));
        assert_eq!(NounType::from_str("milestone"), Some(NounType::MileStone));
        assert_eq!(NounType::from_str("invalid"), None);
        assert_eq!(NounType::from_str(""), None);
    }

    #[test]
    fn test_noun_type_is_container() {
        assert!(NounType::Group.is_container());
        assert!(NounType::Project.is_container());
        assert!(NounType::MileStone.is_container());
        assert!(!NounType::Task.is_container());
        assert!(!NounType::Sow.is_container());
        assert!(!NounType::Artifact.is_container());
    }

    #[test]
    fn test_noun_type_is_leaf_only() {
        assert!(NounType::Artifact.is_leaf_only());
        assert!(!NounType::Task.is_leaf_only());
        assert!(!NounType::Project.is_leaf_only());
    }

    #[test]
    fn test_noun_type_can_be_goal() {
        assert!(NounType::Deliverable.can_be_goal());
        assert!(NounType::Event.can_be_goal());
        assert!(!NounType::Task.can_be_goal());
        assert!(!NounType::Project.can_be_goal());
    }

    #[test]
    fn test_noun_type_display() {
        assert_eq!(NounType::Sow.to_string(), "SOW");
        assert_eq!(NounType::Task.to_string(), "Task");
        assert_eq!(NounType::MileStone.to_string(), "MileStone");
    }

    // NounState tests
    #[test]
    fn test_noun_state_all() {
        assert_eq!(NounState::all().len(), 6);
    }

    #[test]
    fn test_noun_state_from_str() {
        assert_eq!(NounState::from_str("normal"), Some(NounState::Normal));
        assert_eq!(NounState::from_str("Normal"), Some(NounState::Normal));
        assert_eq!(NounState::from_str("escalated"), Some(NounState::Escalated));
        assert_eq!(NounState::from_str("completed"), Some(NounState::Completed));
        assert_eq!(NounState::from_str("incompleted"), Some(NounState::Incompleted));
        assert_eq!(NounState::from_str("closed"), Some(NounState::Closed));
        assert_eq!(NounState::from_str("archived"), Some(NounState::Archived));
        assert_eq!(NounState::from_str("invalid"), None);
    }

    #[test]
    fn test_noun_state_is_terminal() {
        assert!(NounState::Archived.is_terminal());
        assert!(!NounState::Normal.is_terminal());
        assert!(!NounState::Closed.is_terminal());
    }

    #[test]
    fn test_noun_state_can_complete() {
        assert!(NounState::Normal.can_complete());
        assert!(NounState::Escalated.can_complete());
        assert!(!NounState::Completed.can_complete());
        assert!(!NounState::Closed.can_complete());
    }

    #[test]
    fn test_noun_state_can_close() {
        assert!(NounState::Completed.can_close());
        assert!(NounState::Incompleted.can_close());
        assert!(!NounState::Normal.can_close());
        assert!(!NounState::Escalated.can_close());
    }

    #[test]
    fn test_noun_state_default() {
        assert_eq!(NounState::default(), NounState::Normal);
    }

    // Noun struct tests
    #[test]
    fn test_noun_new() {
        let sow_id = Uuid::new_v4();
        let noun = Noun::new(NounType::Task, sow_id, "IT-TASK-001".to_string(), "Test Task".to_string());

        assert_eq!(noun.get_type(), NounType::Task);
        assert_eq!(noun.get_state(), NounState::Normal);
        assert_eq!(noun.sow_id, sow_id);
        assert_eq!(noun.short_name, "IT-TASK-001");
        assert_eq!(noun.title, "Test Task");
        assert!(!noun.is_blocked);
        assert!(noun.parent_id.is_none());
    }

    #[test]
    fn test_noun_is_sow() {
        let sow_id = Uuid::new_v4();
        let sow = Noun::new(NounType::Sow, sow_id, "IT".to_string(), "IT SOW".to_string());
        let task = Noun::new(NounType::Task, sow_id, "IT-TASK-001".to_string(), "Task".to_string());

        assert!(sow.is_sow());
        assert!(!task.is_sow());
    }

    #[test]
    fn test_noun_is_container() {
        let sow_id = Uuid::new_v4();
        let project = Noun::new(NounType::Project, sow_id, "IT-PROJ-001".to_string(), "Project".to_string());
        let task = Noun::new(NounType::Task, sow_id, "IT-TASK-001".to_string(), "Task".to_string());

        assert!(project.is_container());
        assert!(!task.is_container());
    }

    #[test]
    fn test_noun_blocked_goals() {
        let sow_id = Uuid::new_v4();
        let mut noun = Noun::new(NounType::Project, sow_id, "IT-PROJ-001".to_string(), "Project".to_string());

        assert_eq!(noun.blocked_goals(), 0);

        noun.custom_fields = serde_json::json!({"blocked_goals": 3});
        assert_eq!(noun.blocked_goals(), 3);
    }

    #[test]
    fn test_noun_goal_noun_ids() {
        let sow_id = Uuid::new_v4();
        let goal1 = Uuid::new_v4();
        let goal2 = Uuid::new_v4();
        let mut noun = Noun::new(NounType::Project, sow_id, "IT-PROJ-001".to_string(), "Project".to_string());

        assert!(noun.goal_noun_ids().is_empty());

        noun.custom_fields = serde_json::json!({
            "goal_noun_ids": [goal1.to_string(), goal2.to_string()]
        });
        let goals = noun.goal_noun_ids();
        assert_eq!(goals.len(), 2);
        assert!(goals.contains(&goal1));
        assert!(goals.contains(&goal2));
    }

    #[test]
    fn test_noun_auto_complete() {
        let sow_id = Uuid::new_v4();
        let mut noun = Noun::new(NounType::MileStone, sow_id, "IT-MILE-001".to_string(), "Milestone".to_string());

        assert!(!noun.auto_complete());

        noun.custom_fields = serde_json::json!({"auto_complete": true});
        assert!(noun.auto_complete());
    }
}
