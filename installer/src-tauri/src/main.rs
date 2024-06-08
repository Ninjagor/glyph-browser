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
        isUpdateNeeded = false;
      } else {
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
async fn update_glyph() {
  use std::fs;
  use std::io::copy;
  use std::os::unix::fs::PermissionsExt;
  use std::path::Path;
  use std::error::Error;
  use std::env;

  if cfg!(target_os = "windows") {

  } else {
    let url = "https://github.com/Ninjagor/glyph-browser/blob/main/bin/glyph-browser?raw=true";

    let home_dir = env::var("HOME").expect("home dir failed");

    let bin_dir = Path::new(&home_dir).join("bin");
    let glyph_dir = bin_dir.join("glyph");
    let glyph_exe = glyph_dir.join("glyph-browser");

    // Create the bin and glyph directories if they don't exist
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir).expect("bin creation failed");
    }

    if !glyph_dir.exists() {
        fs::create_dir_all(&glyph_dir).expect("glyph directory creation failed");
    }

    // Check if the glyph-browser file exists and remove it
    if glyph_exe.exists() && glyph_exe.is_file() {
        fs::remove_file(&glyph_exe).expect("failed to remove existing glyph-browser file");
    }

    let response = reqwest::Client::new()
      .get(url)
      .send()
      .await
      .expect("UH OH SUSSY BAKA")
      ;

    let mut exe = fs::File::create(&glyph_exe).expect("Failed to create exe");
    let body = response.bytes().await.expect("exe invalid");
    std::io::copy(&mut body.as_ref(), &mut exe).expect("Failed to copy content");

    let mut perms = exe.metadata().expect("Failed to get metadata").permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    exe.set_permissions(perms).expect("Failed to set permissions");
  }

  let app = TAURI_APP.lock().unwrap();
  if let Some(app_handle) = &*app {
      app_handle.emit_all("launch_glyph", Payload { message: "".into() }).unwrap();
  }
}

#[tauri::command]
fn launch_glyph() {
  use std::fs;
  use std::io::copy;
  use std::os::unix::fs::PermissionsExt;
  use std::path::Path;
  use std::error::Error;
  use std::env;
  use std::process::{Command, Stdio};

  if cfg!(target_os = "windows") {
  } else {
    let home_dir = env::var("HOME").expect("home dir failed");

    let bin_dir = Path::new(&home_dir).join("bin");
    let glyph_dir = bin_dir.join("glyph");
    let glyph_exe = glyph_dir.join("glyph-browser");

    println!("launch stage 1");

    if glyph_exe.exists() && glyph_exe.is_file() {
      println!("launch stage 2");

      let app = TAURI_APP.lock().unwrap();
      if let Some(app_handle) = &*app {
        app_handle.emit_all("launched", Payload { message: "".into() }).unwrap();
      }

      std::thread::sleep(std::time::Duration::from_secs(2));

      std::thread::spawn(move || {
        let mut child = Command::new(glyph_exe)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute process");

        let _ = child.wait().expect("child process wasn't running");
        println!("launch stage 3");
    });
      } else {
          eprintln!("Executable not found at {:?}", glyph_exe);
      }
  }
  println!("finished");
  std::process::exit(0);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_files, check_for_updates, update_glyph, launch_glyph])
    .setup(|app| {
      let app_handle = app.handle().clone();
      let mut app_lock = TAURI_APP.lock().unwrap();
      *app_lock = Some(app_handle);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}