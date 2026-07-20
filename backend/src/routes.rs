//! routes.rs
//! Route requests to where they need to go
use axum::routing::{get, post};
use axum::{Router, middleware, Json, extract::Path};
use crate::{auth_apikey, DbConn, Pool, IntErr};


/// API Backend routes (/api/v1/)
pub fn api_routes() -> Router<Pool> {
    Router::new()
        .route("/landing", get(landing))
        .route("/pagewithpath/{p}", get(page_with_path))
        .route("/addsong", post(add_new_song))
        .layer(middleware::from_fn(auth_apikey))
}

/// Retrieve and serve data for the landing page
async fn landing(DbConn(_db): DbConn) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Retrieve and serve data for a page containing a path string
async fn page_with_path(DbConn(_db): DbConn, Path(_p): Path<String>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Handle submission of a new song to put in the DB
async fn add_new_song(DbConn(_db): DbConn, Json(_query): Json<String>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}
