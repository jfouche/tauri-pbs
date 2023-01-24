#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod new_item;

use database::Database;
use new_item::cmd::*;

fn main() {
    tauri::Builder::default()
        .manage(Database::default())
        .invoke_handler(tauri::generate_handler![new_part_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
