//! This module defines structs utilizing Diesel derives for accessing the
//! database.
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::*;

/// A User
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct User {
    /// Internal db id
    pub id: i32,
    /// User's chosen username
    pub username: String,
    /// Hash of username with pin
    pub hashedname: String
}

/// A Song
#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, AsChangeset, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=addedby))]
#[diesel(table_name = song)]
pub struct Song {
    /// Internal db id
    pub id: i32,
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
    /// DB ID of user
    pub addedby: i32
}

/// A rating of a song by a user
#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, AsChangeset, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key=user))]
#[diesel(belongs_to(Song, foreign_key=song))]
#[diesel(table_name = rating)]
pub struct Rating {
    /// Internal db id
    pub id: i32,
    /// DB ID of song
    pub song: i32,
    /// DB ID of user
    pub user: i32,
    /// Liked/disliked/neutral
    pub yeahnah: i32,
    /// Comments
    pub comment: String
}
