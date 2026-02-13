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

#[utoipa::path(
    get,
    path = "/api/v1/logs/",
    params(
        ("page" = i32, Query, description = "Page number"),
        ("size" = i32, Query, description = "Page size"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("filter" = Option<String>, Query, description = "Filter query"),
        ("context" = Option<String>, Query, description = "Log context")
    ),
    responses(
        (status = 200, description = "List of activity logs", body = PaginatedResult<ActivityLogResponse>)
    ),
    security(
        ("jwt" = [])
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/v1/logs/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User activity logs", body = [ActivityLog])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user_activity(
    Claims { .. }: Claims,
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ActivityLog>>, ModuleError> {
    let logs = services::activity_logs::get_user_activity(state.pool.clone(), id).await?;
    Ok(Json(logs))
}
