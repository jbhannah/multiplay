#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod library;
mod models;
mod schema;

use std::{fs::create_dir_all, sync::Mutex};

use library::Library;
use tauri::{
    api::dir::{read_dir, DiskEntry},
    AppHandle, Manager, State,
};

#[derive(Default)]
struct MultiplayState {
    library: Mutex<Option<Library>>,
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
    _app_state: State<MultiplayState>,
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
        .manage(MultiplayState::default())
        .setup(|app| {
            let app_dir = app.path_resolver().app_dir().unwrap();
            create_dir_all(app_dir.clone()).expect("could not create app data directory");

            let library_path = app_dir.join(LIBRARY_FILENAME);
            *(app.state::<MultiplayState>()).library.lock().unwrap() =
                Library::new(library_path.to_str().unwrap()).ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_path, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
