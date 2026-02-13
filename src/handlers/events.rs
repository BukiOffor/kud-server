use crate::dto::events::{
    CheckInWithIdentifierRequest, CheckIntoEventRequest, CreateEventRequest, UpdateEventRequest,
};
use crate::dto::*;
use crate::models::events::Event;

use super::*;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = event_routes(state.clone());
    let api = Router::new().nest("/events", routes);
    Router::new().merge(api)
}

pub fn event_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", post(create_event))
        .route("/update", patch(update_event))
        .route("/delete/{event_id}", delete(delete_event))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        .route("/attendance/check-in", post(check_into_event))
        .route(
            "/attendance/check-in-identifier",
            post(check_into_event_with_identifier),
        )
        .route("/upcoming", get(get_upcoming_events))
        .route("/past", get(get_past_events))
        .route("/", get(get_events))
        .route("/get/{event_id}", get(get_event))
        .route("/user/{user_id}", get(get_events_by_user))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::authorize,
        )))
        .with_state(state)
}

#[utoipa::path(
    patch,
    path = "/api/v1/events/update",
    request_body = UpdateEventRequest,
    responses(
        (status = 200, description = "Event updated successfully", body = Event),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn update_event(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateEventRequest>,
) -> Result<Json<Event>, ModuleError> {
    let response =
        services::events::update_event(state.pool.clone(), payload, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/events/attendance/check-in",
    request_body = CheckIntoEventRequest,
    responses(
        (status = 200, description = "Checked into event successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn check_into_event(
    Claims {
        user_id: performer_id,
        role,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CheckIntoEventRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::events::check_into_event(state.pool.clone(), payload, role, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/events/attendance/check-in-identifier",
    request_body = CheckInWithIdentifierRequest,
    responses(
        (status = 200, description = "Checked into event successfully", body = MessageEmpty),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn check_into_event_with_identifier(
    Claims {
        user_id: performer_id,
        role,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::dto::events::CheckInWithIdentifierRequest>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::events::check_in_with_identifier(state.pool.clone(), payload, role, performer_id)
            .await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/events/create",
    request_body = CreateEventRequest,
    responses(
        (status = 200, description = "Event created successfully", body = Event),
        (status = 400, description = "Bad request")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn create_event(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<Json<Event>, ModuleError> {
    let response = services::events::create_event(state.pool.clone(), user_id, payload).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/api/v1/events/delete/{event_id}",
    params(
        ("event_id" = uuid::Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event deleted successfully", body = MessageEmpty),
        (status = 404, description = "Event not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_event(
    Claims {
        user_id: performer_id,
        ..
    }: Claims,
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Message<()>>, ModuleError> {
    let response =
        services::events::delete_event(state.pool.clone(), event_id, performer_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/get/{event_id}",
    params(
        ("event_id" = uuid::Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event details", body = Event),
        (status = 404, description = "Event not found")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_event(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Event>, ModuleError> {
    let response = services::events::get_event(state.pool.clone(), event_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/",
    responses(
        (status = 200, description = "List of all events", body = [Event])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_events(state.pool.clone()).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/user/{user_id}",
    params(
        ("user_id" = uuid::Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "List of events for user", body = [Event])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_events_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_events_by_user(state.pool.clone(), user_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/upcoming",
    responses(
        (status = 200, description = "Upcoming events", body = [Event])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_upcoming_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_upcoming_events(state.pool.clone()).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/events/past",
    responses(
        (status = 200, description = "Past events", body = [Event])
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn get_past_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_past_events(state.pool.clone()).await?;
    Ok(Json(response))
}
