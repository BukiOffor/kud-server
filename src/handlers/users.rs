use super::*;
use crate::{dto::user::*, models::users::*};
use axum::extract::Multipart;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/users", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get", post(get_user))
        .route("/update", post(update_user))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::authorize,
        )))
        .route("/get_all", post(get_all_users))
        .route("/delete", post(delete_user))
        .route("/deactive", post(deactive_user))
        .route("/register", post(register_user))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        .with_state(state)
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::register_user(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

pub async fn get_user(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserDto>, ModuleError> {
    let response = services::users::get_user(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UserFilter>,
) -> Result<Json<Vec<UserDto>>, ModuleError> {
    let response = services::users::get_all_users(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::update_user(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

pub async fn delete_user(
    Claims { .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::delete_user(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn deactive_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::deactive_user(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn active_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::active_user(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn import_users(
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::import_users(state.pool.clone(), multipart).await?;
    Ok(Json(response))
}

pub async fn export_users(
    State(state): State<Arc<AppState>>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let (headers, file_data) = services::users::export_users(state.pool.clone()).await?;
    Ok((headers, file_data))
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::users::change_password(state.pool.clone(), payload).await?;
    Ok(Json(response))
}
