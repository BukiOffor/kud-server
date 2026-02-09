use crate::dto::attendance::{AttendanceWithUser, SignAttendanceRequest};

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

pub async fn sign_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignAttendanceRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::sign_attendance(state.pool.clone(), user_id, payload).await?;
    Ok(Json(response))
}

pub async fn admin_sign_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::admin_sign_attendance(state.pool.clone(), user_id, id).await?;
    Ok(Json(response))
}

pub async fn get_attendance_on_day(
    State(state): State<Arc<AppState>>,
    Path(date): Path<String>,
) -> Result<Json<Message<Vec<AttendanceWithUser>>>, ModuleError> {
    let response =
        services::user_attendance::get_attendance_on_day(state.pool.clone(), date).await?;
    Ok(Json(response))
}

pub async fn revoke_attendance(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::user_attendance::revoke_attendance(state.pool.clone(), id, user_id).await?;
    Ok(Json(response))
}
