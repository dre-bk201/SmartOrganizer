// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![feature(lazy_cell)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
use std::sync::Arc;

use commands::{
    add_listener, load_from_database, load_settings, remove_listener, save_log, save_settings,
    start_listener, update_listener, get_dir_count
};
use organizer::{logger::LOGGER, SmartOrganizer};
use tauri::{generate_handler, Manager};
use window_shadows::set_shadow;

pub struct State {
    pub inner: Arc<tokio::sync::Mutex<SmartOrganizer>>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            match set_shadow(&window, true) {
                Ok(v) => println!("{v:?}"),
                Err(e) => println!("{e:?}"),
            }
            LOGGER.lock().unwrap().init_window(window);
            Ok(())
        })
        .manage(State {
            inner: Arc::new(tokio::sync::Mutex::new(SmartOrganizer::new().await)),
        })
        .invoke_handler(generate_handler![
            load_from_database,
            add_listener,
            remove_listener,
            update_listener,
            start_listener,
            load_settings,
            save_settings,
            save_log,
            get_dir_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
