// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        platform_id -> Integer,
        shasum -> Text,
    }
}

diesel::table! {
    platforms (id) {
        id -> Integer,
        name -> Text,
        acronym -> Text,
        rom_extensions -> Text,
        save_extensions -> Text,
    }
}

diesel::joinable!(games -> platforms (platform_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    platforms,
);
