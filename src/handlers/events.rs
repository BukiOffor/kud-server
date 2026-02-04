use crate::dto::events::{CheckIntoEventRequest, CreateEventRequest, UpdateEventRequest};
use crate::models::events::Event;

use super::*;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = event_routes(state.clone());
    let api = Router::new().nest("/events", routes);
    Router::new().merge(api)
}

pub fn event_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/attendance/check-in", post(check_into_event))
        .route("/upcoming", get(get_upcoming_events))
        .route("/past", get(get_past_events))
        .route("/", get(get_events))
        .route("/get/{event_id}", get(get_event))
        .route("/user/{user_id}", get(get_events_by_user))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::authorize,
        )))
        .route("/create", post(create_event))
        .route("/update", patch(update_event))
        .route("/delete/{event_id}", delete(delete_event))
        .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::middleware::admin_authorize,
        )))
        .with_state(state)
}

pub async fn update_event(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateEventRequest>,
) -> Result<Json<Event>, ModuleError> {
    let response = services::events::update_event(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

pub async fn check_into_event(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CheckIntoEventRequest>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::events::check_into_event(state.pool.clone(), payload).await?;
    Ok(Json(response))
}

pub async fn create_event(
    Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEventRequest>,
) -> Result<Json<Event>, ModuleError> {
    let response = services::events::create_event(state.pool.clone(), user_id, payload).await?;
    Ok(Json(response))
}

pub async fn delete_event(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Message>, ModuleError> {
    let response = services::events::delete_event(state.pool.clone(), event_id).await?;
    Ok(Json(response))
}

pub async fn get_event(
    State(state): State<Arc<AppState>>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Event>, ModuleError> {
    let response = services::events::get_event(state.pool.clone(), event_id).await?;
    Ok(Json(response))
}

pub async fn get_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_events(state.pool.clone()).await?;
    Ok(Json(response))
}

pub async fn get_events_by_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_events_by_user(state.pool.clone(), user_id).await?;
    Ok(Json(response))
}

pub async fn get_upcoming_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_upcoming_events(state.pool.clone()).await?;
    Ok(Json(response))
}

pub async fn get_past_events(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Event>>, ModuleError> {
    let response = services::events::get_past_events(state.pool.clone()).await?;
    Ok(Json(response))
}
