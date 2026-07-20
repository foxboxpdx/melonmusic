#![warn(missing_docs)]
//! melonmusic-common
//! Defines common structs and enums for passing data between the
//! backend api and frontend wasm.
#[macro_use] extern crate serde;

/// A struct representing data for the landing page
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LandingPage {
    /// Foo
    pub foo: String
}

/// A struct representing a song record
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SongMetadata {
    /// Foo
    pub foo: String
}

/// A struct representing a search query sent to the backend
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SearchQuery {
    /// String to search for
    pub query_string: String,
    /// What to search on
    pub query_type: String
}

/// A struct representing search results
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SearchResult {
    /// Query string searched for
    pub query_string: String,
    /// Results (if any)
    pub results: Vec<SongMetadata>
}
