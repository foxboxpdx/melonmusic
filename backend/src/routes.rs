//! routes.rs
//! Route requests to where they need to go
use axum::routing::{get, post};
use axum::{Router, middleware, Json, extract::Path};
use crate::{auth_apikey, DbConn, Pool, IntErr};


/// API Backend routes (/api/v1/)
pub fn api_routes() -> Router<Pool> {
    Router::new()
        .route("/status", get(api_status))
        .route("/landing", get(landing))
        .route("/addsong", post(add_new_song))
        .route("/ratesong", post(rate_song))
        .route("/list/all", get(list_all))
        .route("/list/by/{t}/{q}", get(list_by))
        .layer(middleware::from_fn(auth_apikey))
}

/// Make sure the server is alive and ready
async fn api_status(DbConn(_db): DbConn) -> Result<Json<String>, IntErr> {
    Ok(Json(format!("Yep it's alive")))
}

/// Retrieve and serve data for the landing page
async fn landing(DbConn(_db): DbConn) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Handle submission of a new song to put in the DB
async fn add_new_song(DbConn(_db): DbConn, Json(_query): Json<String>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Handle submission of a user rating a song
async fn rate_song(DbConn(_db): DbConn, Json(_query): Json<String>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/* Routes for lists */

/// List all songs
async fn list_all(DbConn(_db): DbConn) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// List songs by [genre|user|artist|rated|unrated]
async fn list_by(DbConn(_db): DbConn, Path(t): Path<String>, Path(q): Path<String>) -> Result<Json<String>, IntErr> {
    // t = type, q = query
    // Technically I could just make the Router do this but meh
    match t.as_str() {
        "genre" => {},
        "user" => {},
        "artist" => {},
        "rated" => {},
        "unrated" => {},
        _ => {}
    }
    Ok(Json(format!("type: {t}, query: {q}")))
}
