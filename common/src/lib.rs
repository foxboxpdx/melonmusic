#![warn(missing_docs)]
//! melonmusic-common
//! Defines common structs and enums for passing data between the
//! backend api and frontend wasm.

use num_enum::{IntoPrimitive, TryFromPrimitive};
#[macro_use] extern crate serde;

/// A struct representing data for the landing page
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LandingPage {
    /// Total Users
    pub user_total: i64,
    /// Total Songs
    pub song_total: i64,
    /// Total Ratings
    pub rate_total: i64,
    /// Current user's submitted songs
    pub my_songs: i64,
    /// Current user's submitted ratings
    pub my_ratings: i64,
    /// Songs current user hasn't rated yet
    pub my_unrated: i64
}

/// An enum representing possible ratings
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum YeahNah {
    /// Positive
    YEAH,
    /// Sorta Positive
    SARIGHT,
    /// Neutral
    #[default]
    NOTHINSPECIAL,
    /// Sorta Negative
    NOTFEELINIT,
    /// Negative
    NAH
}

impl YeahNah {
    /// Function to give 'weight' to a given rating
    pub fn weight(&self) -> i32 {
        match self {
            Self::YEAH          => 2,
            Self::SARIGHT       => 1,
            Self::NOTHINSPECIAL => 0,
            Self::NOTFEELINIT   => -1,
            Self::NAH           => -2
        }
    }
}

/// A struct representing incoming data for a user login
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LoginQuery {
    /// Username
    pub username: String,
    /// Pin/Pass
    pub pin: String
}

/// A struct representing outgoing data for a user login
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct LoginResult {
    /// DB ID of user
    pub user_id: i32,
    /// Username
    pub username: String,
    /// Hashed username
    pub hashedname: String
}

/// A struct representing a submitted song record
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SubmittedSongMetadata {
    /// Title of song
    pub title: String,
    /// Artist of song
    pub artist: String,
    /// Youtube link
    pub link: String,
    /// Genre of song
    pub genre: String,
    /// Submitter's comments
    pub comments: String
}

/// A struct representing a submitted rating
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SubmittedRating {
    /// DB ID of song
    pub song_id: i32,
    /// DB ID of user submitting the rating
    pub user_id: i32,
    /// Rating
    pub rating: i32,
    /// Comments
    pub comments: String
}

/// A struct representing a search query sent to the backend
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SearchQuery {
    /// String to search for
    pub query_string: String,
    /// What to search on
    pub query_type: QueryType
}

/// A struct representing search results
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SearchResult {
    /// Query string searched for
    pub query_string: String,
    /// Results (if any)
    pub results: QueryResult
}

/// An enum specifying search query types
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq)]
#[repr(i32)]
pub enum QueryType {
    /// Song query (default)
    #[default]
    SONG,
    /// Rating query
    RATING,
    /// User query
    USER
}

/// An enum specifying search result types
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[repr(i32)]
pub enum QueryResult {
    /// Empty result (default)
    #[default]
    EMPTY,
    /// Song list result
    SONG(Vec<SongMetadata>),
    /// Rating list result
    RATING(Vec<RatingMetadata>),
    /// User list result
    USER(Vec<UserMetadata>)
}

/// A struct representing general data about a single song
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SongMetadata {
    /// Internal db id
    pub id: i32,
    /// Title of song
    pub title: String,
    /// Artist of song
    pub artist: String,
    /// Genre of song
    pub genre: String,
    /// epoch time when added
    pub stamp: i32,
    /// Number of ratings
    pub rating_count: i64,
    /// Rating given by current user (if rated)
    pub current_user_rating: i32,
    /// Comments from current user's rating (if rated)
    pub current_user_coments: String
}

/// A struct representing detailed data about a single song
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct SongDetails {
    /// Title of song
    pub title: String,
    /// Artist of song
    pub artist: String,
    /// Youtube link
    pub link: String,
    /// Genre of song
    pub genre: String,
    /// Submitter's comments
    pub comment: String,
    /// epoch time when added
    pub stamp: i32,
    /// DB ID of user whomst did add
    pub addedby: i32,
    /// Username of user whomst did add
    pub addedby_name: String,
    /// Ratings
    pub ratings: Vec<RatingMetadata>
}

/// A struct representing rating data for a song
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct RatingMetadata {
    /// DB ID of song
    pub song_id: i32,
    /// Title + Artist of song
    pub song_name: String,
    /// DB ID of user
    pub user_id: i32,
    /// Name of user
    pub user_name: String,
    /// Rating
    pub rating: i32,
    /// Comments
    pub comments: String
}

/// A struct representing data about a user
#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub struct UserMetadata {
    /// Username
    pub user_name: String,
    /// Ratings
    pub ratings: Vec<RatingMetadata>,
    /// Songs
    pub songs: Vec<SongMetadata>
}
