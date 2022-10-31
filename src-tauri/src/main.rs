#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod library;
mod models;
mod schema;

use std::fs::create_dir_all;

use crate::library::Library;
use tauri::{
    api::dir::{read_dir, DiskEntry},
    AppHandle, Manager, State,
};

const LIBRARY_FILENAME: &str = "library.sqlite";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_path(
    path: &str,
    recursive: bool,
    _app_handle: AppHandle,
    library: State<Library>,
) -> Vec<String> {
    read_dir(path, recursive)
        .unwrap()
        .iter()
        .filter(|entry| check_if_rom(entry, &library).is_some())
        .map(|entry| entry.path.to_str().unwrap().to_owned())
        .collect::<Vec<String>>()
}

fn check_if_rom<'a>(entry: &'a DiskEntry, library: &'a Library) -> Option<&'a DiskEntry> {
    library
        .rom_extensions()
        .expect("could not query database")
        .contains(&entry.path.extension()?.to_str()?.to_owned())
        .then(|| entry)
}

fn main() {
    tauri::Builder::default()
        .manage(Library::default())
        .setup(|app| {
            let app_dir = app.path_resolver().app_dir().unwrap();
            create_dir_all(app_dir.clone()).expect("could not create app data directory");

            let library_path = app_dir.join(LIBRARY_FILENAME);
            app.state::<Library>()
                .connect(library_path.to_str().unwrap())
                .expect("could not open library database");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_path, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
