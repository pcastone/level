//! API middleware

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

/// Create authentication layer
pub fn auth_layer() -> tower::layer::util::Identity {
    // For now, return identity layer. Full auth middleware can be added later.
    tower::layer::util::Identity::new()
}

/// API key authentication middleware
pub async fn require_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    let api_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok());

    match api_key {
        Some(_key) => {
            // TODO: Validate API key against database
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Extract actor from request
pub fn extract_actor(request: &Request) -> String {
    // For now, use a default actor. In production, extract from JWT or session.
    request
        .headers()
        .get("X-Actor")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "+system".to_string())
}
