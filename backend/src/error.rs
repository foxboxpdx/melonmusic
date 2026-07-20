//! error.rs
//! Define custom error types and handling
/// Helper function for mapping any error into an HTTP500
use std::error::Error;
use serde_json::json;
use axum::{Json, http::StatusCode, {response::{Response, IntoResponse}}};
use diesel_async::pooled_connection::{PoolError, bb8::RunError};
use diesel_migrations::MigrationError;

/// Define a function we can pass to map_err() to convert errors to HTTP500s
pub fn internal_error<E: Error>(err: E) -> IntErr {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Helper type for internal_error
pub type IntErr = (StatusCode, String);

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
#[derive(Debug)]
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

/* Impl From for each error - tedious but makes sure we cover everything */
impl From<Box<dyn Error + Send + Sync>> for FatalBackendError {
    fn from(e: Box<dyn Error + Send + Sync>) -> Self {
        FatalBackendError { msg: format!("{:?}", e)}
    }
}
impl From<MigrationError> for FatalBackendError {
    fn from(e: MigrationError) -> Self {
        FatalBackendError { msg: e.to_string() }
    }
}
impl From<std::env::VarError> for FatalBackendError {
    fn from(e: std::env::VarError) -> Self {
        FatalBackendError { msg: e.to_string() }
    }
}
impl From<PoolError> for FatalBackendError {
    fn from(e: PoolError) -> Self {
        FatalBackendError { msg: e.to_string() }
    }
}
impl From<RunError> for FatalBackendError {
    fn from(e: RunError) -> Self {
        FatalBackendError { msg: e.to_string() }
    }
}
impl From<std::io::Error> for FatalBackendError {
    fn from(e: std::io::Error) -> Self {
        FatalBackendError { msg: e.to_string() }
    }
}
