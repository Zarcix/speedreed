// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, Window, App};
use std::{sync::{Mutex, Arc}, time::Duration};

pub struct ReadState(Arc<Mutex<String>>);
pub struct SpeedState(Arc<Mutex<Duration>>);

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

static DEFAULT_STRING: &str = "New Text Here";
static READING_SPEED: u128 = 300;

fn main() {
  tauri::Builder::default()
  .setup(|app| {
    init_reader(app.get_window("main").unwrap(), app.state(), app.state());
    Ok(())
  }) 
    .manage(ReadState(Arc::new(Mutex::new(DEFAULT_STRING.into()))))
    .manage(SpeedState(Arc::new(Mutex::new(Duration::from_millis(wpm_to_ms(READING_SPEED))))))
    .invoke_handler(tauri::generate_handler![
      get_current_text, 
      get_current_speed,
      set_current_text,
      set_current_speed
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init_reader(app: Window, reading_state: tauri::State<ReadState>, speed_state: tauri::State<SpeedState>) {
  println!("Reader initialized");
  let reading_state_lock = Arc::clone(&reading_state.0);
  let speed_state_lock = Arc::clone(&speed_state.0);
  std::thread::spawn(move || {
    loop {
      let sleep_guard = speed_state_lock.lock().unwrap();
      let sleep_time = sleep_guard.clone();
      drop(sleep_guard);
      std::thread::sleep(sleep_time);

      let mut guard = reading_state_lock.lock().unwrap(); // Lock

      let text = guard.clone();
      let text_parts = text.split_once(" ").or(Some((&text, ""))).unwrap();
      *guard = text_parts.1.to_string();
      drop(guard); // Unlock

      
      println!("Sending: {}", text_parts.0);
      app.emit_all("text-update", Payload { message: text_parts.0.to_string() }
        ).unwrap();
    }
  });
}

#[tauri::command]
fn get_current_text(reading_state: tauri::State<ReadState>) -> String {
  reading_state.0.lock().unwrap().clone()
}

#[tauri::command]
fn get_current_speed(speed_state: tauri::State<SpeedState>) -> u64 {
  ms_to_wpm(speed_state.0.lock().unwrap().clone().as_millis())
}

#[tauri::command]
fn set_current_text(reading_state: tauri::State<ReadState>, text: String) {
  println!("New Text: {}", text);
  reading_state.0.lock().unwrap().clone_from(&text);
}

#[tauri::command]
fn set_current_speed(speed_state: tauri::State<SpeedState>, speed: String) {
  let wpm = speed.parse::<u128>().unwrap_or(READING_SPEED);
  let time = Duration::from_millis(wpm_to_ms(wpm));

  println!("New Sleep Time: {}", time.as_millis());

  speed_state.0.lock().unwrap().clone_from(&time);
}

fn wpm_to_ms(wpm: u128) -> u64 {
  let mut wpm = wpm;
  if wpm < 60 {
    wpm = 60
  }

  let word_per_second = 60 as f64 / wpm as f64;
  let words_per_ms = word_per_second * 1000 as f64;
  return words_per_ms.ceil() as u64;
}

fn ms_to_wpm(ms: u128) -> u64 {
  let word_per_second = 60 as f64 / ms as f64;
  let words_per_ms = word_per_second * 1000 as f64;
  return words_per_ms.ceil() as u64;
}