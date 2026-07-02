#![warn(missing_docs)]
#![recursion_limit="1024"]
//! melonmusic-frontend - lib.rs
//! Define the various pages and components comprising the MelonMusic
//! web front-end as well as helper functions, types, etc.
use wasm_bindgen::prelude::*;
use crate::app::App;

/// Module containing structs and functions for full pages
mod pages;

/// Module defining the WASM application and its routes
pub mod app;

/// Module defining sub-components used by pages
mod components;

// Allow reading environment variables from env.js defining the backend's
// URL and api key
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    static APIURL: JsValue;
    #[wasm_bindgen(thread_local_v2)]
    static APIKEY: JsValue;
}

/// Get the String representing the backend URL from the JsThreadLocal struct
fn get_apiurl() -> String {
    let val = APIURL.with(JsValue::clone);
    val.as_string().unwrap_or("".to_string())
}

/// Get the String representing the backend api key
fn get_apikey() -> String {
    let val = APIKEY.with(JsValue::clone);
    val.as_string().unwrap_or("".to_string())
}

/// Helper function to get the auth header and set a user-agent
fn authheader() -> gloo_net::http::Headers {
    let headers = gloo_net::http::Headers::new();
    headers.append("User-Agent", format!("melonmusic-frontend v{}", env!("CARGO_PKG_VERSION")).as_str());
    headers.append("X-MelonMusic-Api-Key", &get_apikey());
    headers
}

/// Function that executes when the WASM binary starts up
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}

/* Enum and accessories for available backend endpoints */

/// Backend Endpoints
#[derive(Debug, Clone)]
pub enum Endpoint {
    /// Server status information
    Status
}

// Impl to_string()
impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let top = "api/v1/frontend";
        match self {
            Endpoint::Status => write!(f, "{top}/status"),
        }
    }
}
