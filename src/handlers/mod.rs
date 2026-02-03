pub mod auth;
pub mod events;
pub mod user_attendance;
pub mod users;
pub use super::*;

use crate::auth::jwt::Claims;
use axum::middleware;
pub use axum::{
    extract::{Json, Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::*,
};
use tower::ServiceBuilder;

pub fn get_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(auth::routes(state.clone()))
        .merge(users::routes(state.clone()))
        .merge(user_attendance::routes(state.clone()))
        .merge(events::routes(state.clone()))
}
