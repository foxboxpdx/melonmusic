#![warn(missing_docs)]
//! melonmusic-backend lib.rs
//! Provides helper functions, datatypes, etc
use lazy_static::lazy_static;
use std::env::var;
use serde_json::json;
use axum::http::{Request, StatusCode};
use axum::{Json, body::Body, middleware::Next};
use axum::response::{Response, IntoResponse};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel::SqliteConnection;
use diesel_async::pooled_connection::{bb8, AsyncDieselConnectionManager};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::Connection;
use dotenvy::dotenv;
use log::warn;

/// Database model structs
pub mod models;

/// Diesel schema
pub mod schema;

/// Routes
pub mod routes;

//pub mod utils;

/// Helper type for a DB Pool
pub type Pool = bb8::Pool<SyncConnectionWrapper<SqliteConnection>>;

/// Helper type for a DB connection
pub type Db = bb8::PooledConnection<'static, AsyncDieselConnectionManager<SyncConnectionWrapper<SqliteConnection>>>;

/// Part of diesel_migrations for updating the DB schema
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// Run any pending migrations
// pub fn do_migrations() -> Result<(), FatalBackendError> { }

/// Custom extractor that automatically grabs a connection from the pool
pub struct DbConn(bb8::PooledConnection<'static, SyncConnectionWrapper<SqliteConnection>>);
impl<S> FromRequestParts<S> for DbConn where S: Send + Sync, Pool: FromRef<S> {
    type Rejection = (StatusCode, String);
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);
        let conn = pool.get_owned().await.map_err(internal_error)?;
        Ok(Self(conn))
    }
}

/// Helper function for mapping any error into an HTTP500
pub fn internal_error<E: std::error::Error>(err: E) -> IntErr {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
/// Helper type for internal_error
pub type IntErr = (StatusCode, String);

// Pull the API keys from environment variables
// (defaults to systemtime::now to avoid hard-coding a fallback)
lazy_static! {
    static ref APIKEY: String = var("API_KEY").unwrap_or(format!("{:#?}", std::time::SystemTime::now()));
}

/// API Key check
fn is_valid_api_key(key: &str) -> bool { key == APIKEY.to_string() }

/// API key auth middleware function to check for header and value
async fn auth_apikey(request: Request<Body>, next: Next) -> Result<Response, BackendError> {
    let auth_header = request.headers().get("X-MelonMusic-Api-Key").and_then(|h| h.to_str().ok());
    match auth_header { Some(key) if is_valid_api_key(key) => { Ok(next.run(request).await) },
        _ => { Err(BackendError::Unauthorized("Bad or missing auth key".to_string()))}}
}

/* Custom error type for nonfatal HTTP errors */

/// Error enum
pub enum BackendError {
    /// 404 variant
    NotFound(String),
    /// 401 variant
    Unauthorized(String),
    /// 400 variant
    BadRequest(String),
    /// 500 variant
    Internal(String)
}

impl IntoResponse for BackendError {
    /// Convert errors into HTTP responses
    fn into_response(self) -> Response {
        let (status, message) = match self {
            BackendError::NotFound(msg)     => (StatusCode::NOT_FOUND, msg),
            BackendError::BadRequest(msg)   => (StatusCode::BAD_REQUEST, msg),
            BackendError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            BackendError::Internal(msg)     => (StatusCode::INTERNAL_SERVER_ERROR, msg)
        };
        // Return a JSON error response
        let body = Json(json!({"error": message, "status": status.as_u16()}));
        (status, body).into_response()
    }
}

/* Custom error type for fatal program errors */

/// Fatal error
pub struct FatalBackendError {
    /// Why go boom?
    msg: String
}
// Impl to_string()
impl std::fmt::Display for FatalBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fatal error encountered, bailing out: {}", self.msg)
    }
}
// Impl From<String>
impl From<String> for FatalBackendError {
    fn from(msg: String) -> Self {
        FatalBackendError { msg }
    }
}
