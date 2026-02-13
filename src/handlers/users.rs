use super::*;
use crate::auth::middleware as auth_middleware;
use crate::dto::user::*;
use crate::dto::*;
use axum::extract::Multipart;
use axum::middleware as axum_middleware;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/users", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/admin/get_all", get(get_all_users))
        .route("/admin/delete/{id}", delete(delete_user))
        .route("/admin/deactivate/{id}", patch(deactivate_user))
        .route("/admin/activate/{id}", patch(activate_user))
        .route("/admin/update-role/{id}", patch(update_user_role))
        .route("/admin/reset-device-id/{id}", patch(reset_user_device_id))
        .route("/admin/register", post(register_user))
        .route("/admin/import", post(import_users))
        .route("/admin/export", get(export_users))
        .route("/admin/update/{id}", patch(admin_update_user))
        .layer(
            ServiceBuilder::new().layer(axum_middleware::from_fn_with_state(
                state.clone(),
                auth_middleware::admin_authorize,
            )),
        )
        .route("/get/{id}", get(get_user))
        .route("/update", patch(update_user))
        .route("/change-password", patch(change_password))
        .layer(
            ServiceBuilder::new().layer(axum_middleware::from_fn_with_state(
                state.clone(),
                auth_middleware::authorize,
            )),
        )
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/api/v1/users/admin/register",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registered successfully", body = MessageEmpty),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn register_user(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response = services::users::register_user(state.pool.clone(), payload, user_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/get/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = UserDto),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_user(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserDto>, ModuleError> {
    let response = services::users::get_user(state.pool.clone(), id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/admin/get_all",
    params(
        UserFilter
    ),
    responses(
        (status = 200, description = "List of users", body = [UserDto])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_all_users(
    State(state): State<Arc<AppState>>,
    Query(payload): Query<UserFilter>,
) -> Result<Json<Vec<UserDto>>, ModuleError> {
    let response = services::users::get_all_users(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/update",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::update_user(state.pool.clone(), payload, user_id, user_id).await?;
    Ok(Json(response))
}
#[allow(dead_code, unused)]
#[deprecated]
pub async fn delete_user(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    return Err(ModuleError::BadRequest(
        "This endpoint has been deprecated, please use deacyivate user".into(),
    ));
    let response = services::users::delete_user(state.pool.clone(), id, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/admin/deactivate/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deactivated successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn deactivate_user(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response = services::users::deactivate_user(state.pool.clone(), id, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/admin/activate/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User activated successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn activate_user(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response = services::users::activate_user(state.pool.clone(), id, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/admin/update-role/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRoleRequest,
    responses(
        (status = 200, description = "User role updated successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user_role(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::update_user_role(state.pool.clone(), id, payload, performer_id).await?;
    Ok(Json(response))
}

pub async fn import_users(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::import_users(state.pool.clone(), multipart, performer_id).await?;
    Ok(Json(response))
}

pub async fn export_users(
    State(state): State<Arc<AppState>>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let (headers, file_data) = services::users::export_users(state.pool.clone()).await?;
    Ok((headers, file_data))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/change-password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn change_password(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::change_password(state.pool.clone(), payload, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/admin/reset-device-id/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Device ID reset successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn reset_user_device_id(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::reset_user_device_id(id, state.pool.clone(), performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/admin/update/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "User ID")
    ),
    request_body = AdminUpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = MessageEmpty),
        (status = 404, description = "User not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn admin_update_user(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<AdminUpdateUserRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::users::admin_update_user(state.pool.clone(), payload, id, performer_id).await?;
    Ok(Json(response))
}
