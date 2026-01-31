//! Role and permission models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashSet;
use uuid::Uuid;

use super::Verb;

/// Actor role on a noun
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "actor_role", rename_all = "snake_case")]
pub enum ActorRole {
    Owner,
    Sme,
    Assignee,
    Resource,
    Stakeholder,
    Awareness,
}

impl ActorRole {
    /// Get all roles
    pub fn all() -> &'static [ActorRole] {
        &[
            ActorRole::Owner,
            ActorRole::Sme,
            ActorRole::Assignee,
            ActorRole::Resource,
            ActorRole::Stakeholder,
            ActorRole::Awareness,
        ]
    }

    /// Get default verbs for this role
    pub fn default_verbs(&self) -> Vec<Verb> {
        match self {
            ActorRole::Owner => Verb::all().to_vec(), // All verbs
            ActorRole::Sme => Verb::all().to_vec(),   // All verbs
            ActorRole::Assignee => vec![
                Verb::Complete,
                Verb::Incomplete,
                Verb::Escalate,
                Verb::Update,
            ],
            ActorRole::Resource => vec![Verb::Update],
            ActorRole::Stakeholder => vec![], // Read-only
            ActorRole::Awareness => vec![],   // Read-only
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "owner" => Some(ActorRole::Owner),
            "sme" => Some(ActorRole::Sme),
            "assignee" => Some(ActorRole::Assignee),
            "resource" => Some(ActorRole::Resource),
            "stakeholder" => Some(ActorRole::Stakeholder),
            "awareness" => Some(ActorRole::Awareness),
            _ => None,
        }
    }
}

/// Role definition with inheritance
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub inherits_from: Option<Uuid>,
    pub verbs: Vec<String>, // List of allowed verb names
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(name: String, verbs: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            inherits_from: None,
            verbs,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get verbs as Verb enum set
    pub fn verb_set(&self) -> HashSet<Verb> {
        self.verbs
            .iter()
            .filter_map(|s| Verb::from_str(s))
            .collect()
    }
}

/// Role mapping - links roles to actors
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RoleMapping {
    pub id: Uuid,
    pub role_id: Uuid,
    pub actor: String, // Actor string with prefix
    pub sow_id: Option<Uuid>, // Scope to specific SOW, or global if None
    pub created_at: DateTime<Utc>,
}

impl RoleMapping {
    pub fn new(role_id: Uuid, actor: String, sow_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role_id,
            actor,
            sow_id,
            created_at: Utc::now(),
        }
    }
}

/// SOW-level role rules (allow/deny)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SowRoles {
    #[serde(default)]
    pub allow: Vec<RoleRule>,
    #[serde(default)]
    pub deny: Vec<RoleRule>,
}

/// Individual role rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRule {
    pub actor: String,
    #[serde(rename = "type")]
    pub rule_type: Option<String>,
    pub verbs: Vec<String>,
}

impl SowRoles {
    /// Check if actor is allowed any access
    pub fn has_access(&self, actor: &str, actor_groups: &[String]) -> bool {
        let all_actors: Vec<&str> = std::iter::once(actor)
            .chain(actor_groups.iter().map(|s| s.as_str()))
            .collect();

        self.allow.iter().any(|rule| all_actors.contains(&rule.actor.as_str()))
    }

    /// Get allowed verbs for actor
    pub fn allowed_verbs(&self, actor: &str, actor_groups: &[String]) -> HashSet<String> {
        let all_actors: Vec<&str> = std::iter::once(actor)
            .chain(actor_groups.iter().map(|s| s.as_str()))
            .collect();

        let mut allowed: HashSet<String> = HashSet::new();

        // Collect allowed verbs
        for rule in &self.allow {
            if all_actors.contains(&rule.actor.as_str()) {
                allowed.extend(rule.verbs.clone());
            }
        }

        // Remove denied verbs
        for rule in &self.deny {
            if all_actors.contains(&rule.actor.as_str()) {
                for verb in &rule.verbs {
                    allowed.remove(verb);
                }
            }
        }

        allowed
    }
}

/// Request types
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub inherits_from: Option<Uuid>,
    pub verbs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoleMappingRequest {
    pub role_id: Uuid,
    pub actor: String,
    #[serde(default)]
    pub sow_id: Option<Uuid>,
}
