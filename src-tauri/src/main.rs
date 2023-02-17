#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod new_item;

use std::sync::Mutex;

use database::Database;
use new_item::cmd::*;

#[derive(Default)]
pub struct TauriDatabase(Mutex<Database>);

fn main() {
    tauri::Builder::default()
        .manage(TauriDatabase::default())
        .invoke_handler(tauri::generate_handler![new_part_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
