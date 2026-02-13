use axum::Router;
use axum::http::{Method, header::*};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use server::{AppState, handlers, swagger};

use std::sync::Arc;

use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_level(true)
        .with_env_filter("info,tokio_postgres::query=off,tokio_postgres::prepare=off")
        .init();

    let pool: Arc<
        bb8::Pool<server::AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
    > = server::config::create_pool().await.into();

    // Run database migrations
    if let Err(e) = run_migration().await {
        tracing::error!("{}", e.to_string());
        std::process::exit(1);
    }

    if let Err(e) = server::services::users::seed_default_admin(pool.clone()).await {
        tracing::error!("Failed to seed default admin: {}", e.to_string());
    }

    let state: Arc<AppState> = AppState { pool: pool.clone() }.into();

    server::CHIDA_LOCATION
        .set(server::dto::attendance::GeoPoint {
            lat: 9.070818996337124,
            lng: 7.434377769114212,
        })
        .unwrap();
    server::DOA_LOCATION
        .set(server::dto::attendance::GeoPoint {
            lat: 9.076381,
            lng: 7.431592,
        })
        .unwrap();
    server::HOME_LOCATION_CHECKIN_RADIUS
        .set(server::dto::attendance::GeoPoint {
            lat: 9.110356556451427,
            lng: 7.380244362342951,
        })
        .unwrap();
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers([
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_HEADERS,
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .allow_credentials(true)
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "https://kud-server.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://app.koinoniaushers.cloud"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://koinoniaushers.cloud"
                .parse::<HeaderValue>()
                .unwrap(),
        ]);
    server::info!("Starting Web Server ............");
    let app = Router::new()
        .merge(handlers::get_routes(state.clone()))
        .merge(swagger::swagger_routes())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let api = axum::Router::new().nest("/api/v1", app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9898").await.unwrap();
    server::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, api).await.unwrap();
}

pub async fn run_migration() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;

    // Get a synchronous connection string from your config
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Run migrations in a blocking thread
    tokio::task::spawn_blocking(move || {
        let mut conn = diesel::PgConnection::establish(&database_url)?;
        conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
            tracing::error!("{}", e.to_string());
            std::process::exit(1);
        })?;
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await?
}
