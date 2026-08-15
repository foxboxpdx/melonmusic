//! routes.rs
//! Route requests to where they need to go
use axum::routing::{get, post};
use axum::{Router, middleware, Json, extract::Path};
use crate::{auth_apikey, DbConn, DbPass, Pool, IntErr, error::internal_error};
use diesel_async::RunQueryDsl;
use diesel::prelude::*;
use melonmusic_common::*;

/// API Backend routes (/api/v1/)
pub fn api_routes() -> Router<Pool> {
    Router::new()
        .route("/status", get(api_status))
        .route("/landing", get(landing))
        .route("/login", post(login))
        .route("/addsong", post(add_new_song))
        .route("/ratesong", post(rate_song))
        .route("/list/all", get(list_all))
        .nest("/list/by", list_routes())
        //.route("/list/by/{t}/{q}", get(list_by))
        .layer(middleware::from_fn(auth_apikey))
}

/// Routes for listing songs by a given field matching a given value
pub fn list_routes() -> Router<Pool> {
    Router::new()
        .route("/genre/{q}", get(list_by_genre))
        .route("/user/{q}", get(list_by_user))
        .route("/artist/{q}", get(list_by_artist))
        .route("/rated/{q}", get(list_by_rated))
        .route("/unrated/{q}", get(list_by_unrated))
        .route("/highest", get(list_by_highest))
        .route("/lowest", get(list_by_lowest))
}

/// Make sure the server is alive and ready
async fn api_status(DbConn(mut db): DbConn) -> Result<Json<bool>, IntErr> {
    use crate::schema::user::dsl::user;
    // Try to get the user count as something quick 'n simple
    let users_count = user.count().get_result(&mut db).await.unwrap_or(-1);
    if users_count == -1 { Ok(Json(false)) }
    else { Ok(Json(true)) }
}

/// Retrieve and serve data for the landing page
async fn landing(DbConn(mut db): DbConn) -> Result<Json<LandingPage>, IntErr> {
    use crate::schema::user::dsl::user;
    use crate::schema::song::dsl::{song, addedby, id as sid};
    use crate::schema::rating::dsl::{rating, user as ruid, song as rsid};
    // LandingPage is just a bunch of counts starting with totals
    let total_users = user.count().get_result(&mut db).await.unwrap_or(-1);
    let total_songs = song.count().get_result(&mut db).await.unwrap_or(-1);
    let total_rates = rating.count().get_result(&mut db).await.unwrap_or(-1);
    // Now user-specific data (note to self: uh how are we getting the uid exactly)
    let current_user_id = 1;
    let user_songs = song.filter(addedby.eq(current_user_id)).count().get_result(&mut db).await.unwrap_or(-1);
    let user_rates = rating.filter(ruid.eq(current_user_id)).count().get_result(&mut db).await.unwrap_or(-1);
    // Could probably turn this into a big mess of inner_joins but I'm so tired
    // Subquery grabs songids of songs rated by current_user
    let rated_songs = rating.filter(ruid.eq(current_user_id)).select(rsid).into_boxed();
    let user_unrated = song.filter(sid.ne_all(rated_songs)).count().get_result(&mut db).await.unwrap_or(-1);
    // Assemble the struct and send
    let retval = LandingPage {
        user_total: total_users,
        rate_total: total_rates,
        song_total: total_songs,
        my_songs: user_songs,
        my_ratings: user_rates,
        my_unrated: user_unrated
    };
    Ok(Json(retval))
}

/// Handle submission of a user login
async fn login(DbConn(_db): DbConn, Json(_query): Json<LoginQuery>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Handle submission of a new song to put in the DB
async fn add_new_song(DbConn(_db): DbConn, Json(_query): Json<SubmittedSongMetadata>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/// Handle submission of a user rating a song
async fn rate_song(DbConn(_db): DbConn, Json(_query): Json<SubmittedRating>) -> Result<Json<String>, IntErr> {
    Ok(Json(String::new()))
}

/* Routes for lists */

/// List all songs
async fn list_all(DbConn(mut db): DbConn) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::song;
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Start by grabbing all songs
    let songs: Vec<Song> = song.select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List by genre
async fn list_by_genre(DbConn(mut db): DbConn, Path(q): Path<String>) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, genre};
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Grab songs with matching genre
    let songs: Vec<Song> = song.filter(genre.like(&q)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List by user
async fn list_by_user(DbConn(mut db): DbConn, Path(q): Path<String>) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, addedby};
    use crate::schema::user::dsl::{user, id, username};
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Grab ids of any users matching query
    let user_ids = user.filter(username.like(&q)).select(id).into_boxed();
    // Grab songs added by any of the above userids
    let songs: Vec<Song> = song.filter(addedby.eq_any(user_ids)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List by artist
async fn list_by_artist(DbConn(mut db): DbConn, Path(q): Path<String>) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, artist};
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Grab songs with matching artist
    let songs: Vec<Song> = song.filter(artist.like(&q)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List songs that have been rated by the current user
async fn list_by_rated(DbConn(mut db): DbConn, Path(_q): Path<String>) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, id as sid};
    use crate::schema::rating::dsl::{rating, user as ruid, song as rsid};
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Grab song ids of all songs the current user has rated
    let rated_songs = rating.filter(ruid.eq(current_uid)).select(rsid).into_boxed();
    // Grab song data of song ids appearing above
    let songs: Vec<Song> = song.filter(sid.eq_any(rated_songs)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List songs that have not been rated by the current user
async fn list_by_unrated(DbConn(mut db): DbConn, Path(_q): Path<String>) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, id as sid};
    use crate::schema::rating::dsl::{rating, user as ruid, song as rsid};
    use crate::models::Song;
    // placeholder
    let current_uid = 1;

    // Grab song ids of all songs the current user has rated
    let rated_songs = rating.filter(ruid.eq(current_uid)).select(rsid).into_boxed();
    // Grab song data of song ids NOT appearing above
    let songs: Vec<Song> = song.filter(sid.ne_all(rated_songs)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List the 10 highest rated songs
async fn list_by_highest(DbConn(mut db): DbConn) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, id as sid};
    use crate::schema::rating::dsl::rating;
    use crate::models::{Song, Rating};
    use std::collections::HashMap;
    // placeholder
    let current_uid = 1;

    // Grab all ratings
    let ratings: Vec<Rating> = rating.select(Rating::as_select()).load(&mut db).await.map_err(internal_error)?;
    // Iterate through ratings and generate a hashmap correlating every song to an overall rating
    let mut rate_mapping = HashMap::new();
    for r in ratings {
        let rating_enum = YeahNah::try_from(r.yeahnah).unwrap_or(YeahNah::NOTHINSPECIAL);
        let x = rate_mapping.entry(r.id).or_insert(0);
        *x += rating_enum.weight();
    }
    // Convert the hashmap into a vec of tuples (k,v) and sort by v descending
    let mut tuplevec: Vec<_> = rate_mapping.iter().collect();
    tuplevec.sort_by(|a, b| b.1.cmp(&a.1));

    // Take the first 10 (or all of them if len < 10)
    // Kinda running out of steam right now so this is gonna look dumb til i have a nap
    let mut top_ten = Vec::new();
    for t in tuplevec {
        top_ten.push(t.0);
        if top_ten.len() == 10 { break; }
    }

    // Finally, grab songs filtered by song id being in the top_ten
    let songs: Vec<Song> = song.filter(sid.eq_any(top_ten)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}

/// List the 10 lowest rated songs
async fn list_by_lowest(DbConn(mut db): DbConn) -> Result<Json<Vec<SongMetadata>>, IntErr> {
    use crate::schema::song::dsl::{song, id as sid};
    use crate::schema::rating::dsl::rating;
    use crate::models::{Song, Rating};
    use std::collections::HashMap;
    // placeholder
    let current_uid = 1;

    // Grab all ratings
    let ratings: Vec<Rating> = rating.select(Rating::as_select()).load(&mut db).await.map_err(internal_error)?;
    // Iterate through ratings and generate a hashmap correlating every song to an overall rating
    let mut rate_mapping = HashMap::new();
    for r in ratings {
        let rating_enum = YeahNah::try_from(r.yeahnah).unwrap_or(YeahNah::NOTHINSPECIAL);
        let x = rate_mapping.entry(r.id).or_insert(0);
        *x += rating_enum.weight();
    }
    // Convert the hashmap into a vec of tuples (k,v) and sort by v ascending
    let mut tuplevec: Vec<_> = rate_mapping.iter().collect();
    tuplevec.sort_by(|a, b| a.1.cmp(&b.1));

    // Take the first 10 (or all of them if len < 10)
    let mut bot_ten = Vec::new();
    for t in tuplevec {
        bot_ten.push(t.0);
        if bot_ten.len() == 10 { break; }
    }

    // Finally, grab songs filtered by song id being in the top_ten
    let songs: Vec<Song> = song.filter(sid.eq_any(bot_ten)).select(Song::as_select()).load(&mut db).await.map_err(internal_error)?;

    // Iterate through song list to grab additional metadata:
    // ratings count and current user's rating/comments
    let mut songs_with_metadata: Vec<SongMetadata> = Vec::new();
    for s in songs {
        let sm = fetch_rating_data(&mut db, &s, current_uid).await?;
        songs_with_metadata.push(sm);
    }
    Ok(Json(songs_with_metadata))
}


/* Helper functions */


/// Function that grabs the ratings count and current user ratings for
/// the given song and user id and returns a SongMetadata struct
async fn fetch_rating_data(mut db: &mut DbPass, s: &crate::models::Song, uid: i32) -> Result<SongMetadata, IntErr> {
    use crate::schema::rating::dsl::{rating, user as ruid, song as rsid};
    use crate::models::Rating;

    let ratings_count: i64 = rating.filter(rsid.eq(s.id)).count().get_result(&mut db).await.map_err(internal_error)?;
    let user_rating = rating.filter(rsid.eq(s.id)).filter(ruid.eq(uid))
        .select(Rating::as_select()).first(&mut db).await.map_err(internal_error)?;
    Ok(SongMetadata {
        id: s.id, title: s.title.to_string(), artist: s.artist.to_string(), genre: s.genre.to_string(),
        stamp: s.stamp, rating_count: ratings_count, current_user_rating: user_rating.yeahnah,
        current_user_coments: user_rating.comment
    })
}
