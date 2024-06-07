// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

lazy_static! {
  static ref TAURI_APP: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
}

#[tauri::command]
fn check_files() {
  use std::fs;
  use std::process::{Command, Stdio};
  use std::env;
  use std::path::PathBuf;
  use std::io::Write;

  if cfg!(target_os = "windows") {
      // windows config (TO DO)
  } else {
      // MAC and LINUX config
      let home_dir = env::var("HOME").expect("Failed to get HOME directory");
      let config_dir = PathBuf::from(home_dir).join(".config").join("glyph");

      if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
      }

      let config_file_path = config_dir.join("config.json");

      if !config_file_path.exists() {
        let mut file = fs::File::create(&config_file_path).expect("Failed to create config file");
        file.write_all(b"{}").expect("Failed to write to config");
      }
  };
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_files])
    .setup(|app| {
      let app_handle = app.handle().clone();
      let mut app_lock = TAURI_APP.lock().unwrap();
      *app_lock = Some(app_handle);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
