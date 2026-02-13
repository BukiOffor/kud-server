use std::collections::HashMap;

use axum::extract::Multipart;

use super::*;
use crate::dto::roster::*;
use crate::dto::*;
use crate::models::roster::{Hall, Roster};

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/roster", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(create_roster))
        .route("/{id}", get(get_roster))
        .route("/update", patch(update_roster))
        .route("/all", get(get_all_rosters))
        .route("/activate/{id}", patch(activate_roster))
        .route("/{id}", delete(delete_roster))
        .route("/{id}/assignments", get(view_roster_assignments))
        .route("/export/{id}", get(export_roster))
        .route("/export/{id}/hall", get(export_roster_by_hall))
        .route("/import/{id}", post(import_roster))
        .route("/hall", patch(update_user_hall))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        // .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     crate::auth::middleware::authorize,
        // )))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/api/v1/roster/create",
    request_body = NewRoster,
    responses(
        (status = 200, description = "Roster created successfully", body = MessageRosterDto),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_roster(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewRoster>,
) -> Result<Json<Message<Roster>>, ModuleError> {
    let response = services::roster::create_roster(state.pool.clone(), payload, user_id).await?;
    Ok(Json(Message::new(
        "Roster created successfully",
        Some(response),
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/roster/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "Roster ID")
    ),
    responses(
        (status = 200, description = "Roster details", body = Roster),
        (status = 404, description = "Roster not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_roster(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Roster>, ModuleError> {
    let response = services::roster::get_roster(state.pool.clone(), id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/roster/update",
    request_body = UpdateRosterRequest,
    responses(
        (status = 200, description = "Roster updated successfully", body = MessageRosterDto),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_roster(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateRosterRequest>,
) -> Result<Json<Message<Roster>>, ModuleError> {
    let response = services::roster::update_roster(state.pool.clone(), payload, user_id).await?;
    Ok(Json(Message::new(
        "Roster updated successfully",
        Some(response),
    )))
}

#[utoipa::path(
    get,
    path = "/api/v1/roster/all",
    responses(
        (status = 200, description = "List of all rosters", body = [Roster])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_all_rosters(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Roster>>, ModuleError> {
    let response = services::roster::get_all_rosters(state.pool.clone()).await?;
    Ok(Json(response))
}

#[utoipa::path(
    patch,
    path = "/api/v1/roster/activate/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "Roster ID")
    ),
    responses(
        (status = 200, description = "Roster activated successfully", body = MessageEmpty),
        (status = 404, description = "Roster not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn activate_roster(
    Path(id): Path<uuid::Uuid>,
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Message<()>>, ModuleError> {
    services::roster::activate_roster(state.pool.clone(), id, user_id).await?;
    Ok(Json(Message::new("Roster activated successfully", None)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/roster/{id}",
    params(
        ("id" = uuid::Uuid, Path, description = "Roster ID")
    ),
    responses(
        (status = 200, description = "Roster deleted successfully", body = MessageEmpty),
        (status = 404, description = "Roster not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_roster(
    Path(id): Path<uuid::Uuid>,
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Message<()>>, ModuleError> {
    services::roster::delete_roster(state.pool.clone(), id, user_id).await?;
    Ok(Json(Message::new("Roster deleted successfully", None)))
}

#[utoipa::path(
    get,
    path = "/api/v1/roster/{id}/assignments",
    params(
        ("id" = uuid::Uuid, Path, description = "Roster ID")
    ),
    responses(
        (status = 200, description = "Roster assignments list", body = [RosterAssignmentDto])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn view_roster_assignments(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RosterAssignmentDto>>, ModuleError> {
    let response = services::roster::view_roster_assignments(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn export_roster(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let response = services::roster::export_roster(id, state.pool.clone()).await?;
    Ok(response)
}

pub async fn export_roster_by_hall(
    Path(id): Path<uuid::Uuid>,
    Query(hall): Query<HashMap<String, Hall>>,
    State(state): State<Arc<AppState>>,
) -> Result<(axum::http::HeaderMap, Vec<u8>), ModuleError> {
    let hall = hall
        .get("hall")
        .ok_or(ModuleError::BadRequest("Hall not found".into()))?;
    let response =
        services::roster::export_roster_by_hall(id, state.pool.clone(), hall.clone()).await?;
    Ok(response)
}

pub async fn import_roster(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Path(roster_id): Path<uuid::Uuid>,
    multipart: Multipart,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::roster::import_roster(state.pool.clone(), roster_id, multipart, user_id).await?;
    Ok(Json(Message::new(
        "Roster uploaded successfully",
        Some(response),
    )))
}

#[utoipa::path(
    patch,
    path = "/api/v1/roster/user-hall",
    request_body = UpdateUserHallRequest,
    responses(
        (status = 200, description = "User hall updated successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_user_hall(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserHallRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response = services::roster::update_user_hall(
        state.pool.clone(),
        payload.user_id,
        payload.user_roster_id,
        payload.hall,
        user_id,
    )
    .await?;
    Ok(Json(response))
}
