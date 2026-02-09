use super::*;
use crate::dto::roster::*;
use crate::models::roster::Roster;

pub fn routes(state: Arc<AppState>) -> Router {
    let routes = user_routes(state.clone());
    let api = Router::new().nest("/roster", routes);
    Router::new().merge(api)
}

pub fn user_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create", get(create_roster))
        .route("/{id}", get(get_roster))
        .route("/update", patch(update_roster))
        .route("/all", get(get_all_rosters))
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

pub async fn create_roster(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewRoster>,
) -> Result<Json<Message<Roster>>, ModuleError> {
    let response = services::roster::create_roster(state.pool.clone(), payload).await?;
    Ok(Json(Message::new(
        "Roster created successfully",
        Some(response),
    )))
}

pub async fn get_roster(
    Path(id): Path<uuid::Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Roster>, ModuleError> {
    let response = services::roster::get_roster(state.pool.clone(), id).await?;
    Ok(Json(response))
}

pub async fn update_roster(
    // Claims { user_id, .. }: Claims,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateRosterRequest>,
) -> Result<Json<Message<Roster>>, ModuleError> {
    let response = services::roster::update_roster(state.pool.clone(), payload).await?;
    Ok(Json(Message::new(
        "Roster updated successfully",
        Some(response),
    )))
}

pub async fn get_all_rosters(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Roster>>, ModuleError> {
    let response = services::roster::get_all_rosters(state.pool.clone()).await?;
    Ok(Json(response))
}
