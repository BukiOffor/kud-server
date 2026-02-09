use crate::{
    AppState,
    auth::jwt::Claims,
    dto::pagination::{PaginatedResult, PaginationWithContext},
    errors::ModuleError,
    models::activity_logs::{ActivityLog, ActivityLogResponse},
    services,
};
use axum::extract::Query;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(state: Arc<AppState>) -> Router {
    let log_routes = log_routes(state.clone());
    let api = Router::new().nest("/logs", log_routes);
    Router::new().merge(api)
}

pub fn log_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", crate::log_route!(get, "/", get_logs))
        .route("/{id}", get(get_user_activity))
        .with_state(state)
}

pub async fn get_logs(
    Claims { .. }: Claims,
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationWithContext<Option<String>>>,
) -> Result<Json<PaginatedResult<ActivityLogResponse>>, ModuleError> {
    let search = pagination.context.clone();
    let logs =
        services::activity_logs::get_logs(state.pool.clone(), pagination.into(), search).await?;
    Ok(Json(logs))
}

pub async fn get_user_activity(
    Claims { .. }: Claims,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ActivityLog>>, ModuleError> {
    let logs = services::activity_logs::get_user_activity(state.pool.clone(), id).await?;
    Ok(Json(logs))
}
