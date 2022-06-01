#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod finite_state_machine;

use serde::Deserialize;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load_machine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn load_machine(file_name:String) {
  
}
