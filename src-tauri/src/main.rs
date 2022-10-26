#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::create_dir_all, sync::Mutex};

use diesel::{prelude::*, Connection, SqliteConnection};
use tauri::{
    api::dir::{read_dir, DiskEntry},
    AppHandle, Manager, State,
};

#[derive(Default)]
struct Library {
    db: Mutex<Option<SqliteConnection>>,
}

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
    _library: State<Library>,
) -> Vec<String> {
    read_dir(path, recursive)
        .unwrap()
        .iter()
        .filter_map(check_if_rom)
        .map(|entry| entry.path.to_str().unwrap().to_owned())
        .collect::<Vec<String>>()
}

fn check_if_rom(entry: &DiskEntry) -> Option<&DiskEntry> {
    match entry.path.extension() {
        Some(ext) => match ext.to_str() {
            Some("gb") => Some(entry),
            Some("gbc") => Some(entry),
            Some("gba") => Some(entry),
            Some("nds") => Some(entry),
            _ => None,
        },
        _ => None,
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Library::default())
        .setup(|app| {
            let app_dir = app.path_resolver().app_dir().unwrap();
            create_dir_all(app_dir.clone()).expect("could not create app data directory");

            let library_path = app_dir.join(LIBRARY_FILENAME);
            *(app.state::<Library>()).db.lock().unwrap() = Some(
                SqliteConnection::establish(library_path.to_str().unwrap())
                    .expect("could not open library database"),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_path, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
