// @generated automatically by Diesel CLI.
#![allow(missing_docs)]

diesel::table! {
    rating (id) {
        id -> Integer,
        song -> Integer,
        user -> Integer,
        yeahnah -> Integer,
        comment -> Text,
    }
}

diesel::table! {
    song (id) {
        id -> Integer,
        title -> Text,
        artist -> Text,
        link -> Text,
        genre -> Text,
        comment -> Text,
        stamp -> Integer,
        addedby -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        hashedname -> Text,
    }
}

diesel::joinable!(rating -> song (song));
diesel::joinable!(rating -> user (user));
diesel::joinable!(song -> user (addedby));

diesel::allow_tables_to_appear_in_same_query!(rating, song, user,);
