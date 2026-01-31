//! Help API endpoints

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::models::{ActorRole, NounState, NounType, Verb};

use super::state::AppState;

/// Help routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/help/nouns", get(list_noun_help))
        .route("/help/nouns/:noun_type", get(get_noun_help))
        .route("/help/verbs", get(list_verb_help))
        .route("/help/verbs/:verb", get(get_verb_help))
        .route("/help/containers", get(list_container_help))
        .route("/help/roles", get(list_role_help))
        .route("/help/states", get(list_state_help))
}

#[derive(Debug, Serialize)]
pub struct NounHelp {
    name: String,
    abbreviation: String,
    description: String,
    is_container: bool,
    is_leaf_only: bool,
    can_be_goal: bool,
}

/// GET /help/nouns - List all noun types
async fn list_noun_help() -> Result<Json<Vec<NounHelp>>> {
    let nouns: Vec<NounHelp> = NounType::all()
        .iter()
        .map(|nt| NounHelp {
            name: nt.to_string(),
            abbreviation: nt.abbreviation().to_string(),
            description: get_noun_description(*nt),
            is_container: nt.is_container(),
            is_leaf_only: nt.is_leaf_only(),
            can_be_goal: nt.can_be_goal(),
        })
        .collect();
    Ok(Json(nouns))
}

/// GET /help/nouns/:noun_type - Get specific noun type help
async fn get_noun_help(Path(noun_type): Path<String>) -> Result<Json<NounHelp>> {
    let nt = NounType::from_str(&noun_type)
        .ok_or_else(|| crate::error::LevelError::InvalidNounType(noun_type))?;

    Ok(Json(NounHelp {
        name: nt.to_string(),
        abbreviation: nt.abbreviation().to_string(),
        description: get_noun_description(nt),
        is_container: nt.is_container(),
        is_leaf_only: nt.is_leaf_only(),
        can_be_goal: nt.can_be_goal(),
    }))
}

fn get_noun_description(nt: NounType) -> String {
    match nt {
        NounType::Sow => "Statement of Work - Genesis noun that creates and owns all other nouns".to_string(),
        NounType::Item => "Generic work item for tracking any type of work".to_string(),
        NounType::Task => "Specific action item that needs to be completed".to_string(),
        NounType::Request => "Request for action, resource, or decision".to_string(),
        NounType::Meeting => "Scheduled meeting with participants and agenda".to_string(),
        NounType::Deliverable => "Tangible output or result to be produced".to_string(),
        NounType::Event => "Time-bound occurrence or milestone date".to_string(),
        NounType::Blocker => "Issue preventing progress on other nouns".to_string(),
        NounType::Artifact => "Document, file, or other reference material (leaf-only)".to_string(),
        NounType::Group => "Container for organizing related nouns".to_string(),
        NounType::Project => "Container with goals (Deliverables/Events) tracking".to_string(),
        NounType::MileStone => "Container with auto-complete capability".to_string(),
    }
}

#[derive(Debug, Serialize)]
pub struct VerbHelp {
    name: String,
    description: String,
    changes_state: bool,
    modifies_relationships: bool,
    required_states: Vec<String>,
    side_effects: Vec<String>,
}

/// GET /help/verbs - List all verbs
async fn list_verb_help() -> Result<Json<Vec<VerbHelp>>> {
    let verbs: Vec<VerbHelp> = Verb::all()
        .iter()
        .map(|v| get_verb_details(*v))
        .collect();
    Ok(Json(verbs))
}

/// GET /help/verbs/:verb - Get specific verb help
async fn get_verb_help(Path(verb): Path<String>) -> Result<Json<VerbHelp>> {
    let v = Verb::from_str(&verb)
        .ok_or_else(|| crate::error::LevelError::ValidationError(format!("Unknown verb: {}", verb)))?;
    Ok(Json(get_verb_details(v)))
}

fn get_verb_details(v: Verb) -> VerbHelp {
    match v {
        Verb::Open => VerbHelp {
            name: "Open".to_string(),
            description: "Create a new noun".to_string(),
            changes_state: false,
            modifies_relationships: false,
            required_states: vec![],
            side_effects: vec!["Generates short_name".to_string(), "Creates transaction".to_string()],
        },
        Verb::Complete => VerbHelp {
            name: "Complete".to_string(),
            description: "Mark noun as completed successfully".to_string(),
            changes_state: true,
            modifies_relationships: false,
            required_states: vec!["Normal".to_string(), "Escalated".to_string()],
            side_effects: vec![
                "Sets completed_at timestamp".to_string(),
                "If Blocker, releases all targets".to_string(),
            ],
        },
        Verb::Incomplete => VerbHelp {
            name: "Incomplete".to_string(),
            description: "Mark noun as not completed".to_string(),
            changes_state: true,
            modifies_relationships: false,
            required_states: vec!["Normal".to_string(), "Escalated".to_string()],
            side_effects: vec![],
        },
        Verb::Normal => VerbHelp {
            name: "Normal".to_string(),
            description: "Reset state to Normal (de-escalate)".to_string(),
            changes_state: true,
            modifies_relationships: false,
            required_states: vec!["Escalated".to_string()],
            side_effects: vec![],
        },
        Verb::Escalate => VerbHelp {
            name: "Escalate".to_string(),
            description: "Mark noun as urgent/escalated".to_string(),
            changes_state: true,
            modifies_relationships: false,
            required_states: vec!["Normal".to_string()],
            side_effects: vec![],
        },
        Verb::Close => VerbHelp {
            name: "Close".to_string(),
            description: "Close a completed/incompleted noun".to_string(),
            changes_state: true,
            modifies_relationships: false,
            required_states: vec!["Completed".to_string(), "Incompleted".to_string()],
            side_effects: vec![
                "Sets closed_at timestamp".to_string(),
                "Requires all children to be closed".to_string(),
            ],
        },
        Verb::Update => VerbHelp {
            name: "Update".to_string(),
            description: "Modify noun fields".to_string(),
            changes_state: false,
            modifies_relationships: false,
            required_states: vec!["Normal".to_string(), "Escalated".to_string()],
            side_effects: vec![],
        },
        Verb::Reparent => VerbHelp {
            name: "Reparent".to_string(),
            description: "Move noun to new parent or SOW root".to_string(),
            changes_state: false,
            modifies_relationships: true,
            required_states: vec![],
            side_effects: vec![],
        },
        Verb::Assign => VerbHelp {
            name: "Assign".to_string(),
            description: "Add noun to a container (Group/Project/MileStone)".to_string(),
            changes_state: false,
            modifies_relationships: true,
            required_states: vec![],
            side_effects: vec!["Creates NounAssignment with sort_order".to_string()],
        },
        Verb::Unassign => VerbHelp {
            name: "Unassign".to_string(),
            description: "Remove noun from a container".to_string(),
            changes_state: false,
            modifies_relationships: true,
            required_states: vec![],
            side_effects: vec![],
        },
        Verb::Blocked => VerbHelp {
            name: "Blocked".to_string(),
            description: "Create blocking relationship".to_string(),
            changes_state: false,
            modifies_relationships: true,
            required_states: vec![],
            side_effects: vec![
                "Sets is_blocked on target".to_string(),
                "Propagates to all children".to_string(),
            ],
        },
        Verb::Release => VerbHelp {
            name: "Release".to_string(),
            description: "Remove blocking relationship".to_string(),
            changes_state: false,
            modifies_relationships: true,
            required_states: vec![],
            side_effects: vec!["Recomputes is_blocked for affected nouns".to_string()],
        },
    }
}

/// GET /help/containers - List container types
async fn list_container_help() -> Result<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!([
        {
            "type": "Group",
            "description": "Basic container for organizing related nouns",
            "features": ["Noun assignments with sort_order", "Used for Kanban columns"]
        },
        {
            "type": "Project",
            "description": "Container with goal tracking",
            "features": ["goal_noun_ids for Deliverables/Events", "blocked_goals counter", "Goals don't set is_blocked on Project"]
        },
        {
            "type": "MileStone",
            "description": "Container with auto-complete capability",
            "features": ["auto_complete flag", "Automatically completes when all assigned nouns complete", "is_blocked if ANY assigned noun is blocked"]
        }
    ])))
}

/// GET /help/roles - List actor roles
async fn list_role_help() -> Result<Json<serde_json::Value>> {
    let roles: Vec<serde_json::Value> = ActorRole::all()
        .iter()
        .map(|r| {
            let verbs: Vec<String> = r.default_verbs().iter().map(|v| v.to_string()).collect();
            serde_json::json!({
                "role": format!("{:?}", r),
                "default_verbs": verbs,
                "description": get_role_description(*r)
            })
        })
        .collect();
    Ok(Json(serde_json::json!(roles)))
}

fn get_role_description(role: ActorRole) -> String {
    match role {
        ActorRole::Owner => "Full control over the noun".to_string(),
        ActorRole::Sme => "Subject Matter Expert with full access".to_string(),
        ActorRole::Assignee => "Assigned to work on the noun".to_string(),
        ActorRole::Resource => "Contributing resource with limited access".to_string(),
        ActorRole::Stakeholder => "Read-only stakeholder visibility".to_string(),
        ActorRole::Awareness => "Read-only awareness/notification".to_string(),
    }
}

/// GET /help/states - List noun states
async fn list_state_help() -> Result<Json<serde_json::Value>> {
    let states: Vec<serde_json::Value> = NounState::all()
        .iter()
        .map(|s| {
            serde_json::json!({
                "state": s.to_string(),
                "description": get_state_description(*s),
                "is_terminal": s.is_terminal(),
                "can_complete": s.can_complete(),
                "can_close": s.can_close()
            })
        })
        .collect();
    Ok(Json(serde_json::json!(states)))
}

fn get_state_description(state: NounState) -> String {
    match state {
        NounState::Normal => "Initial state, noun is active and in progress".to_string(),
        NounState::Escalated => "Noun has been escalated/marked as urgent".to_string(),
        NounState::Completed => "Noun has been completed successfully".to_string(),
        NounState::Incompleted => "Noun has been marked as not completed".to_string(),
        NounState::Closed => "Noun is closed and cannot be modified".to_string(),
        NounState::Archived => "Terminal state, noun is archived".to_string(),
    }
}
