// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        path -> Nullable<Text>,
        platform_id -> Nullable<Integer>,
        shasum -> Nullable<Text>,
    }
}

diesel::table! {
    platforms (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        acronym -> Nullable<Text>,
        rom_extensions -> Nullable<Text>,
        save_extensions -> Nullable<Text>,
    }
}

diesel::joinable!(games -> platforms (platform_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    platforms,
);
