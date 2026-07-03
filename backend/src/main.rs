//! melonmusic-backend
//! Creates a simple webserver to serve the frontend WASM file
//! and do API things
use axum::routing::get;
use axum::Router;
use diesel::SqliteConnection;
use log::warn;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use melonmusic_backend::routes::api_routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PORT: i32 = 8000;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    env_logger::builder().format_timestamp(None).init();

    // do_migrations()?;

    let db_url = std::env::var("DATABASE_URL")?;

    // Set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::sync_connection_wrapper::SyncConnectionWrapper<SqliteConnection>>::new(db_url);
    let db = bb8::Pool::builder().build(config).await?;

    // Establish routes
    let router = Router::new()
        .route("/health", get(health))
        .nest("/api/v1", api_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await?;
    warn!("Launching melonmusic-backend v{VERSION} on port {PORT}");
    axum::serve(listener, router).await?;
    Ok(())
}

async fn health() -> &'static str {
    "OK"
}
