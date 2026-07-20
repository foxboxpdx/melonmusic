//! melonmusic-backend
//! Creates a simple webserver to serve the frontend WASM file
//! and do API things
use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::{Router, http::{Request, StatusCode}, body::Body};
use diesel::SqliteConnection;
use log::warn;
use tower_http::{cors::CorsLayer, trace::TraceLayer, services::ServeDir};
use tower::ServiceExt;
use std::path::PathBuf;
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use melonmusic_backend::{do_migrations, FatalBackendError, routes::{api_routes}};

#[tokio::main]
async fn main() -> Result<(), FatalBackendError> {
    const PORT: i32 = 8000;
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    env_logger::builder().format_timestamp(None).init();

    let db_url = std::env::var("DATABASE_URL")?;

    // Set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::sync_connection_wrapper::SyncConnectionWrapper<SqliteConnection>>::new(db_url);
    let db = bb8::Pool::builder().build(config).await?;

    //let conn = db.get_owned().await?;
    do_migrations(&db).await?;

    // Establish routes
    let router = Router::new()
        .route("/health", get(health))
        .nest("/api/v1", api_routes())
        // fallback_service defines an action to take if an unknown route is requested
        // this should allow us to serve the frontend wasm...I think?
        .fallback_service(get(|req: Request<Body>| async move {
            let res = ServeDir::new("static").oneshot(req).await.unwrap();
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index = PathBuf::from("static").join("index.html");
                    tokio::fs::read_to_string(index).await
                        .map(|content| (StatusCode::OK, Html(content)).into_response())
                        .unwrap_or_else(|_| { (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found").into_response()})
                },
                _ => res.into_response()
            }
        }))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await?;
    warn!("Launching melonmusic-backend v{VERSION} on port {PORT}");
    match axum::serve(listener, router).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into())
    }
}

async fn health() -> &'static str {
    "OK"
}
