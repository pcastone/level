//! REST API routes

mod state;
mod middleware;
mod sow;
mod noun;
mod transaction;
mod views;
mod admin;
mod user;
mod help;

use axum::Router;

pub use state::AppState;

/// Build all API routes
pub fn routes(state: AppState) -> Router {
    Router::new()
        .merge(sow::routes())
        .merge(noun::routes())
        .merge(transaction::routes())
        .merge(views::routes())
        .merge(admin::routes())
        .merge(user::routes())
        .merge(help::routes())
        .layer(middleware::auth_layer())
        .with_state(state)
}
