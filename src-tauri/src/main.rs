// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod tray;
use std::sync::Arc;

use commands::{
    add_listener, get_dir_count, load_from_database, load_settings, remove_listener, save_log,
    save_settings, start_listener, update_listener,
};
use organizer::{logger::LOGGER, SmartOrganizer};
use tauri::{generate_handler, Manager};

pub struct State {
    pub inner: Arc<tokio::sync::Mutex<SmartOrganizer>>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                // Do not restore hidden/minimized visibility from the tray session.
                .with_state_flags(
                    tauri_plugin_window_state::StateFlags::SIZE
                        | tauri_plugin_window_state::StateFlags::MAXIMIZED,
                )
                .build(),
        )
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            LOGGER.lock().unwrap().init_window(window);
            tray::setup(app)?;
            Ok(())
        })
        .on_window_event(tray::on_window_event)
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
