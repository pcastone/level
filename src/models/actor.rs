//! Actor models - Person, Group, Vendor, AI

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use uuid::Uuid;

/// Actor type with prefix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
pub enum ActorType {
    Person,  // + prefix
    Group,   // @ prefix
    Vendor,  // ! prefix
    Ai,      // * prefix
}

impl ActorType {
    /// Get the prefix for this actor type
    pub fn prefix(&self) -> char {
        match self {
            ActorType::Person => '+',
            ActorType::Group => '@',
            ActorType::Vendor => '!',
            ActorType::Ai => '*',
        }
    }

    /// Parse actor type from prefix
    pub fn from_prefix(c: char) -> Option<Self> {
        match c {
            '+' => Some(ActorType::Person),
            '@' => Some(ActorType::Group),
            '!' => Some(ActorType::Vendor),
            '*' => Some(ActorType::Ai),
            _ => None,
        }
    }
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorType::Person => write!(f, "Person"),
            ActorType::Group => write!(f, "Group"),
            ActorType::Vendor => write!(f, "Vendor"),
            ActorType::Ai => write!(f, "AI"),
        }
    }
}

/// Actor source (internal or LDAP)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "actor_source", rename_all = "snake_case")]
pub enum ActorSource {
    Internal,
    Ldap,
}

impl Default for ActorSource {
    fn default() -> Self {
        ActorSource::Internal
    }
}

/// Person actor
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub source: ActorSource,
    pub ldap_dn: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Person {
    /// Create new internal person
    pub fn new(username: String, email: String, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            display_name,
            source: ActorSource::Internal,
            ldap_dn: None,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get actor string (+username)
    pub fn actor_string(&self) -> String {
        format!("+{}", self.username)
    }
}

/// Group actor
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub source: ActorSource,
    pub ldap_dn: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Group {
    /// Create new internal group
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            source: ActorSource::Internal,
            ldap_dn: None,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get actor string (@name)
    pub fn actor_string(&self) -> String {
        format!("@{}", self.name)
    }
}

/// Group membership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub id: Uuid,
    pub group_id: Uuid,
    pub person_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl GroupMember {
    pub fn new(group_id: Uuid, person_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            person_id,
            created_at: Utc::now(),
        }
    }
}

/// Vendor actor
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vendor {
    pub id: Uuid,
    pub name: String,
    pub contact_email: Option<String>,
    pub description: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Vendor {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            contact_email: None,
            description: None,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get actor string (!name)
    pub fn actor_string(&self) -> String {
        format!("!{}", self.name)
    }
}

/// AI actor
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ai {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub description: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ai {
    pub fn new(name: String, model: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            model,
            description: None,
            active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get actor string (*name)
    pub fn actor_string(&self) -> String {
        format!("*{}", self.name)
    }
}

/// Parse an actor string to (type, name)
pub fn parse_actor_string(s: &str) -> Option<(ActorType, &str)> {
    let mut chars = s.chars();
    let prefix = chars.next()?;
    let actor_type = ActorType::from_prefix(prefix)?;
    let name = chars.as_str();
    if name.is_empty() {
        return None;
    }
    Some((actor_type, name))
}

/// Create request types
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePersonRequest {
    pub username: String,
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateVendorRequest {
    pub name: String,
    #[serde(default)]
    pub contact_email: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAiRequest {
    pub name: String,
    pub model: String,
    #[serde(default)]
    pub description: Option<String>,
}
