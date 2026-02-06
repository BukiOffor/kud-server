use crate::dto::attendance::SignAttendanceRequest;

use super::*;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/attendance", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/check-in", post(sign_attendance))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::authorize,
        )))
        .route("/admin/sign/{id}", get(admin_sign_attendance))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
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
