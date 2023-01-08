#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenvy::dotenv;
use guilded_university_app::{read_token, write_token, StorageError};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn store_token(token: &str) -> Result<(), StorageError> {
    write_token(token)?;
    Ok(())
}

#[tauri::command]
fn get_token() -> Result<String, StorageError> {
    let token = read_token()?;
    Ok(token)
}

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![store_token, get_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
