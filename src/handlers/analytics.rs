use crate::dto::*;
use chrono::NaiveDate;
use std::collections::HashMap;

use super::*;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = analytics_routes(state.clone());
    Router::new().nest("/analytics", routes)
}

pub fn analytics_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/total-users", get(get_total_users))
        .route("/users-on-day", get(get_users_present_on_day))
        .route("/attendance-rates", get(get_attendance_rates))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        .route("/users-on-day/{date}", get(get_users_present_on_day))
        .route("/user-attendance/{id}", get(get_user_attendance))
        .route("/upcoming-birthdays", get(get_upcoming_birthdays))
        .route("/event-report/{id}", get(get_event_stats_report))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/total-users",
    responses(
        (status = 200, description = "Total users list", body = MessageUserDtoVec)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_total_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Message<Vec<crate::dto::user::UserDto>>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let response = services::analytics::fetch_total_users(&mut conn).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/users-on-day",
    params(
        ("date" = NaiveDate, Query, description = "Date to fetch users for")
    ),
    responses(
        (status = 200, description = "Users present on day", body = MessageUserPresentStats)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_users_present_on_day(
    State(state): State<Arc<AppState>>,
    Query(date): Query<HashMap<String, NaiveDate>>,
) -> Result<Json<Message<crate::dto::analytics::UserPresentStats>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let date = date.get("date").ok_or(ModuleError::BadRequest(
        "Date is required".to_string().into(),
    ))?;
    let response =
        services::analytics::fetch_users_present_on_a_specific_day(&mut conn, *date).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/upcoming-birthdays",
    responses(
        (status = 200, description = "Upcoming birthdays", body = MessageUserDtoVec)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_upcoming_birthdays(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Message<Vec<crate::dto::user::UserDto>>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let response = services::analytics::fetch_upcoming_birthdays(&mut conn).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/attendance-rates",
    responses(
        (status = 200, description = "Attendance rates", body = MessageAttendanceStats)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_attendance_rates(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Message<crate::dto::analytics::AttendanceStats>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let response = services::analytics::fetch_attendance_rates(&mut conn).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/user-attendance/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User attendance history", body = MessageUserAttendanceHistory)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user_attendance(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<crate::dto::analytics::UserAttendanceHistory>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let response = services::analytics::fetch_user_attendance(&mut conn, id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/analytics/event-report/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event stats report", body = MessageEventStatsReport)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_event_stats_report(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<crate::dto::analytics::EventStatsReport>>, ModuleError> {
    let mut conn = state
        .pool
        .get()
        .await
        .map_err(|e| ModuleError::InternalError(e.to_string().into()))?;
    let response = services::analytics::fetch_event_stats_report(&mut conn, id).await?;
    Ok(Json(response))
}
