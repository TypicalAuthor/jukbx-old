// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod cmd;
mod db;
mod library;
mod log;
mod metadata;
mod models;
mod schema;
mod structs;
use tauri::Manager;
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            match set_shadow(&window, true) {
                Ok(_) => (),
                Err(e) => {
                    println!("Trying to set shadows on an unsupported platform!");
                    println!("{}", e.to_string());
                    window.set_decorations(true).unwrap();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::set_shadows,
            cmd::run_migrations,
            cmd::show_window,
            cmd::register_folder,
            cmd::request_all_libraries,
            cmd::request_folders,
            cmd::request_lyrics,
            cmd::request_song,
            cmd::search_string
        ])
        .plugin(tauri_plugin_persisted_scope::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
