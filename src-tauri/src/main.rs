// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod injector;
mod util;

use injector::patch;
use util::get_config_json;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config() -> String {
    match get_config_json() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to get config: {}", e);
            "{}".to_string()
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![patch])
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
