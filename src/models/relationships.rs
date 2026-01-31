//! Relationship models - junction tables

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::ActorRole;

/// NounActor - M:N actor-role relationship on nouns
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NounActor {
    pub id: Uuid,
    pub noun_id: Uuid,
    pub actor: String, // Actor string with prefix (+, @, !, *)
    pub role: ActorRole,
    pub created_at: DateTime<Utc>,
}

impl NounActor {
    pub fn new(noun_id: Uuid, actor: String, role: ActorRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            noun_id,
            actor,
            role,
            created_at: Utc::now(),
        }
    }
}

/// NounAssignment - assigns nouns to containers (Group, Project, MileStone)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NounAssignment {
    pub id: Uuid,
    pub noun_id: Uuid,
    pub container_id: Uuid,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

impl NounAssignment {
    pub fn new(noun_id: Uuid, container_id: Uuid, sort_order: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            noun_id,
            container_id,
            sort_order,
            created_at: Utc::now(),
        }
    }
}

/// NounBlock - blocker-target relationship
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NounBlock {
    pub id: Uuid,
    pub blocker_id: Uuid, // Must be type=Blocker
    pub target_id: Uuid,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl NounBlock {
    pub fn new(blocker_id: Uuid, target_id: Uuid, reason: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            blocker_id,
            target_id,
            reason,
            created_at: Utc::now(),
        }
    }
}

/// NounHashTag - global tagging
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NounHashTag {
    pub id: Uuid,
    pub noun_id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
}

impl NounHashTag {
    pub fn new(noun_id: Uuid, tag: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            noun_id,
            tag: tag.to_lowercase().trim_start_matches('#').to_string(),
            created_at: Utc::now(),
        }
    }
}

/// Request types
#[derive(Debug, Clone, Deserialize)]
pub struct AssignNounRequest {
    pub noun_id: Uuid,
    pub container_id: Uuid,
    #[serde(default)]
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockNounRequest {
    pub blocker_id: Uuid,
    pub target_id: Uuid,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddActorRequest {
    pub actor: String,
    pub role: ActorRole,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddTagRequest {
    pub tag: String,
}
