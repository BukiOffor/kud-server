use crate::dto::attendance::{AttendanceWithUser, SignAttendanceRequest};
use crate::dto::*;

use super::*;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/attendance", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/admin/sign/{id}", get(admin_sign_attendance))
        .route("/admin/revoke/{id}", delete(revoke_attendance))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        .route("/check-in", post(sign_attendance))
        .route("/on-day/{date}", get(get_attendance_on_day))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::authorize,
        )))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/api/v1/attendance/check-in",
    request_body = SignAttendanceRequest,
    responses(
        (status = 200, description = "Attendance signed successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn sign_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignAttendanceRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::sign_attendance(state.pool.clone(), user_id, payload).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/attendance/admin/sign/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Attendance signed by admin successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn admin_sign_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::admin_sign_attendance(state.pool.clone(), user_id, id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/attendance/on-day/{date}",
    params(
        ("date" = String, Path, description = "Date in YYYY-MM-DD format")
    ),
    responses(
        (status = 200, description = "Attendance list for the day", body = MessageAttendanceVec)
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_attendance_on_day(
    State(state): State<Arc<AppState>>,
    Path(date): Path<String>,
) -> Result<Json<Message<Vec<AttendanceWithUser>>>, ModuleError> {
    let response =
        services::user_attendance::get_attendance_on_day(state.pool.clone(), date).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/v1/attendance/admin/revoke/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "Attendance ID")
    ),
    responses(
        (status = 200, description = "Attendance revoked successfully", body = MessageEmpty),
        (status = 404, description = "Attendance not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn revoke_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::revoke_attendance(state.pool.clone(), id, user_id).await?;
    Ok(Json(response))
}
