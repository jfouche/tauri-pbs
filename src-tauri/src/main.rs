#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Deserialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Deserialize, Debug)]
struct NewItem<'a> {
    pn: &'a str,
    name: &'a str,
}

impl NewItem<'_> {
    fn check(&self) -> Result<(), String> {
        if self.pn.is_empty() {
            Err("PN is empty".into())
        } else if self.name.is_empty() {
            Err("Name is empty".into())
        } else {
            Ok(())
        }
    }
}

#[tauri::command]
async fn new_part_number(new_item: NewItem<'_>) -> Result<String, String> {
    println!("new_part_number - 1");
    new_item.check()?;
    println!("new_part_number - 2");
    Ok(format!("Item crated: {:?}", new_item))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![new_part_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
