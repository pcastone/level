//! Error types for Level application

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum LevelError {
    #[error("Noun not found: {0}")]
    NounNotFound(String),

    #[error("SOW not found: {0}")]
    SowNotFound(String),

    #[error("Invalid state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Noun is blocked: {0}")]
    NounBlocked(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid reference: {0}")]
    InvalidReference(String),

    #[error("Duplicate short name: {0}")]
    DuplicateShortName(String),

    #[error("Invalid noun type: {0}")]
    InvalidNounType(String),

    #[error("Children not closed: noun {0} has open children")]
    ChildrenNotClosed(String),

    #[error("Artifact cannot have children")]
    ArtifactCannotHaveChildren,

    #[error("Invalid container type: {0}")]
    InvalidContainerType(String),

    #[error("Invalid blocker type: {0}")]
    InvalidBlockerType(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Error response body
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl LevelError {
    /// Get the error code string
    pub fn code(&self) -> &'static str {
        match self {
            LevelError::NounNotFound(_) => "NOUN_NOT_FOUND",
            LevelError::SowNotFound(_) => "SOW_NOT_FOUND",
            LevelError::InvalidStateTransition { .. } => "INVALID_STATE_TRANSITION",
            LevelError::NounBlocked(_) => "NOUN_BLOCKED",
            LevelError::PermissionDenied(_) => "PERMISSION_DENIED",
            LevelError::ValidationError(_) => "VALIDATION_ERROR",
            LevelError::InvalidReference(_) => "INVALID_REFERENCE",
            LevelError::DuplicateShortName(_) => "DUPLICATE_SHORT_NAME",
            LevelError::InvalidNounType(_) => "INVALID_NOUN_TYPE",
            LevelError::ChildrenNotClosed(_) => "CHILDREN_NOT_CLOSED",
            LevelError::ArtifactCannotHaveChildren => "ARTIFACT_CANNOT_HAVE_CHILDREN",
            LevelError::InvalidContainerType(_) => "INVALID_CONTAINER_TYPE",
            LevelError::InvalidBlockerType(_) => "INVALID_BLOCKER_TYPE",
            LevelError::Database(_) => "DATABASE_ERROR",
            LevelError::Config(_) => "CONFIG_ERROR",
            LevelError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// Get HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            LevelError::NounNotFound(_) | LevelError::SowNotFound(_) => StatusCode::NOT_FOUND,
            LevelError::InvalidStateTransition { .. }
            | LevelError::NounBlocked(_)
            | LevelError::ValidationError(_)
            | LevelError::InvalidReference(_)
            | LevelError::DuplicateShortName(_)
            | LevelError::InvalidNounType(_)
            | LevelError::ChildrenNotClosed(_)
            | LevelError::ArtifactCannotHaveChildren
            | LevelError::InvalidContainerType(_)
            | LevelError::InvalidBlockerType(_) => StatusCode::BAD_REQUEST,
            LevelError::PermissionDenied(_) => StatusCode::FORBIDDEN,
            LevelError::Database(_) | LevelError::Config(_) | LevelError::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for LevelError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = ErrorResponse {
            error: ErrorBody {
                code: self.code().to_string(),
                message: self.to_string(),
                details: None,
            },
        };
        (status, Json(body)).into_response()
    }
}

/// Result type alias for Level operations
pub type Result<T> = std::result::Result<T, LevelError>;
