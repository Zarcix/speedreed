// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

pub struct ReadState(Mutex<String>);
pub struct SpeedState(Mutex<isize>);

static DEFAULT_STRING: &str = "New Text Here";
static READING_SPEED: isize = 200;

fn main() {
  tauri::Builder::default()
    .manage(ReadState(Mutex::new(DEFAULT_STRING.into())))
    .manage(SpeedState(Mutex::new(READING_SPEED)))
    .invoke_handler(tauri::generate_handler![
      get_current_text, 
      get_current_speed,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_current_text(reading_state: tauri::State<ReadState>) -> String {
  reading_state.0.lock().unwrap().clone()
}

#[tauri::command]
fn get_current_speed(speed_state: tauri::State<SpeedState>) -> isize {
  speed_state.0.lock().unwrap().clone()
}