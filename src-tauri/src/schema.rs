// @generated automatically by Diesel CLI.

diesel::table! {
    platforms (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        acronym -> Nullable<Text>,
        rom_extensions -> Nullable<Text>,
        save_extensions -> Nullable<Text>,
    }
}
