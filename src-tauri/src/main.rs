// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod projects;

use projects::{cargo::CargoProject, ProjType, Project};
use std::env;

// this example exports your types on startup when in debug mode. You can do whatever.

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_proj])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("sus baka");
    format!("Hello, {}!", name)
}

#[tauri::command(rename_all = "snake_case")]
// TODO: Unhardcode project dir
/// Creates a project. Valid params are: "rs", "nx", "ts"
fn create_proj(name: &str, proj_type: &str) {
    let cur_dir = match env::current_dir() {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let cur_dir_str = match cur_dir.to_str() {
        Some(cur_dir_str) => cur_dir_str,
        None => panic!("Failed to convert current directory to string"),
    };

    let mut dirs: Vec<&str> = cur_dir_str.split("\\").collect();

    dirs.pop();

    dirs.push("test");

    let proj_dir = dirs.join("\\");

    let proj_type = match proj_type {
        "rs" => ProjType::RUST,
        "nx" => ProjType::NEXUS,
        "ts" => ProjType::TYPESCRIPT,
        _ => panic!("Invalid project type: {}", proj_type),
    };

    projects::create_proj(match proj_type {
        ProjType::RUST => CargoProject::new(name, proj_dir),
        ProjType::NEXUS => todo!(),
        ProjType::TYPESCRIPT => todo!(),
    });
}
