// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

lazy_static! {
  static ref TAURI_APP: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
}

#[tauri::command]
async fn check_files() {
  use std::fs;
  use std::process::{Command, Stdio};
  use std::env;
  use std::path::PathBuf;
  use std::io::Write;

  let mut updatedNeeded: bool = false;

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

      let commit_file_path = config_dir.join("commit.txt");

      #[derive(Debug, Serialize, Deserialize)]
      struct Commit {
        sha: String
      }

      if !commit_file_path.exists() { 

        // let app = TAURI_APP.lock().unwrap();
        // if let Some(app_handle) = &*app {
        //     // You can now use the app_handle
        //     app_handle.emit_all("install_needed", Payload { message: "".into() }).unwrap();
        // }

        updatedNeeded = true;

        let repo_url = "https://api.github.com/repos/Ninjagor/glyph-browser/commits/main";

        let response: Commit = reqwest::Client::new()
          .get(repo_url)
          .header("User-Agent", "request")
          .send()
          .await.expect("send fail")
          .json()
          .await.expect("text fail");

        println!("{:#?}", response);

        let sha = response.sha;

        let mut file = fs::File::create(&commit_file_path).expect("Failed to create commit file");
        file.write_all(sha.as_bytes()).expect("Failed to write to commit file");
      }

      let app = TAURI_APP.lock().unwrap();
      if let Some(app_handle) = &*app {
          // You can now use the app_handle
          if updatedNeeded {
            app_handle.emit_all("install_needed", Payload { message: "".into() }).unwrap();
          } else {
            app_handle.emit_all("check_files_done", Payload { message: "".into() }).unwrap();
          }
      }
  };
}

#[tauri::command]
async fn check_for_updates() {
  use std::fs;
  use std::process::{Command, Stdio};
  use std::env;
  use std::path::PathBuf;
  use std::io::Write;

  let mut isUpdateNeeded: bool = false;


  if cfg!(target_os = "windows") {

  } else {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let config_dir = PathBuf::from(home_dir).join(".config").join("glyph");

    if !config_dir.exists() {
      panic!("Corrupted Configuration.");
    }

    let commit_file_path = config_dir.join("commit.txt");

    if !commit_file_path.exists() {
      panic!("Corrupted Configuration.");
    }

    let stored_commit_sha = std::fs::read_to_string(commit_file_path).expect("fs failure. there is nothing that can be done. i think. idk.");

    #[derive(Debug, Serialize, Deserialize)]
      struct Commit {
        sha: String
    }

    let repo_url = "https://api.github.com/repos/Ninjagor/glyph-browser/commits/main";

    let response: Commit = reqwest::Client::new()
      .get(repo_url)
      .header("User-Agent", "request")
      .send()
      .await.expect("send fail")
      .json()
      .await.expect("text fail");

      println!("{:#?}", response);

      let sha = response.sha;

      if sha == stored_commit_sha {
        isUpdateNeeded = true;
      }

  }

  let app = TAURI_APP.lock().unwrap();
  if let Some(app_handle) = &*app {
      // You can now use the app_handle
      if isUpdateNeeded {
        app_handle.emit_all("install_needed", Payload { message: "".into() }).unwrap();
      } else {
        app_handle.emit_all("launch_glyph", Payload { message: "".into() }).unwrap();
      }
  }
}

#[tauri::command]
fn update_glyph() {

}

#[tauri::command]
fn launch_glpyh() {

}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_files, check_for_updates, update_glyph, launch_glpyh])
    .setup(|app| {
      let app_handle = app.handle().clone();
      let mut app_lock = TAURI_APP.lock().unwrap();
      *app_lock = Some(app_handle);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}