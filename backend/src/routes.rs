//! routes.rs
//! Routes for which on to listen
use axum::routing::get;
use axum::{Router, middleware, Json, extract::Path};
use crate::{auth_apikey, DbConn, Pool, IntErr};


/// Router for routing routes
pub fn api_routes() -> Router<Pool> {
    Router::new()
        .route("/landing", get(landing))
        .route("/pagewithpath/{p}", get(page_with_path))
        .layer(middleware::from_fn(auth_apikey))
}

async fn landing(DbConn(_db): DbConn) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

async fn page_with_path(DbConn(_db): DbConn, Path(_p): Path<String>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}
