use std::{time::Duration, sync::{Arc, Mutex}};

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

static DEFAULT_TEXT: &str = "";
static READING_SPEED: u128 = 300;

#[wasm_bindgen]
struct SpeedReed {
    display: Arc<Mutex<String>>,
    speed: Arc<Mutex<Duration>>
}

#[wasm_bindgen]
impl SpeedReed {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SpeedReed {
        let display: Arc<Mutex<String>> = Mutex::new(DEFAULT_TEXT.into()).into();
        let speed: Arc<Mutex<Duration>> = Mutex::new(Duration::from_millis(wpm_to_ms(READING_SPEED))).into();

        Self {
            display,
            speed
        }
        /* Need to spawn thread with emit capability
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
         */
    }

    pub fn get_current_text(&self) -> String {
        return self.display.lock().unwrap().clone();
    }

    pub fn get_current_speed(&self) -> u64 {
        ms_to_wpm(self.speed.lock().unwrap().clone().as_millis())
    }

    pub fn set_current_text(&mut self, text: &str) {
        self.display.lock().unwrap().clone_from(&text.into());
    }

    pub fn set_current_speed(&mut self, speed: &str) {
        let wpm = speed.parse::<u128>().unwrap_or(READING_SPEED);
        let time = Duration::from_millis(wpm_to_ms(wpm));
      
        println!("New Sleep Time: {}", time.as_millis());
      
        self.speed.lock().unwrap().clone_from(&time);
      }
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