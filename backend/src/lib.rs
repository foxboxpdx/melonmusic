#![warn(missing_docs)]
//! melonmusic-backend lib.rs
//! Provides helper functions, datatypes, etc
use lazy_static::lazy_static;
use std::env::var;
use axum::{body::Body, middleware::Next,
          {http::{Request, StatusCode, request::Parts}},
          {response::Response, {extract::{FromRef, FromRequestParts}}}};
use diesel::SqliteConnection;
use diesel_async::{pooled_connection::{bb8, AsyncDieselConnectionManager},
                   sync_connection_wrapper::SyncConnectionWrapper, AsyncMigrationHarness};
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use log::warn;

/// Database model structs
pub mod models;

/// Diesel schema
pub mod schema;

/// Routes
pub mod routes;

/// Error types
pub mod error;

//pub mod utils;

/* Reexports for convenience */
pub use error::{BackendError, FatalBackendError, IntErr, internal_error};


/// Shortcut type for a DB Pool
pub type Pool = bb8::Pool<SyncConnectionWrapper<SqliteConnection>>;

/// Shortcut type for a DB connection
pub type Db = bb8::PooledConnection<'static, AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>>>;

/// Run any pending migrations
pub async fn do_migrations(db: &Pool) -> Result<(), FatalBackendError> {
    let migrations = FileBasedMigrations::find_migrations_directory()?;
    let pool = Pool::from_ref(db);
    let conn = pool.get_owned().await?;
    let mut harness = AsyncMigrationHarness::new(conn);
    harness.run_pending_migrations(migrations)?;
    Ok(())
}

/// Custom extractor that automatically grabs a connection from the pool
/// Syntactic sugar so sweet it'll give you Syntactic Diabetes
pub struct DbConn(bb8::PooledConnection<'static, SyncConnectionWrapper<SqliteConnection>>);
impl<S> FromRequestParts<S> for DbConn where S: Send + Sync, Pool: FromRef<S> {
    type Rejection = (StatusCode, String);
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let conn = pool.get_owned().await.map_err(internal_error)?;
        Ok(Self(conn))
    }
}

// Pull the API keys from environment variables
// (defaults to systemtime::now to avoid hard-coding a fallback)
lazy_static! {
    static ref APIKEY: String = var("API_KEY").unwrap_or(format!("{:#?}", std::time::SystemTime::now()));
}

/// API Key check
fn valid_key(key: &str) -> bool { key == APIKEY.to_string() }

/// API key auth middleware function to check for header and value
async fn auth_apikey(request: Request<Body>, next: Next) -> Result<Response, BackendError> {
    let auth_header = request.headers().get("X-MelonMusic-Api-Key").and_then(|h| h.to_str().ok());
    match auth_header {
        Some(key) if valid_key(key) => Ok(next.run(request).await),
        _                           => Err(BackendError::Unauthorized("Bad or missing auth key".to_string()))
    }
}
