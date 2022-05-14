#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_store::PluginBuilder;
use window_shadows::set_shadow;

mod commands;
use commands::{add_listener, delete_listener, dir_len, organize, undo_action, update_listener};

use org::smart_organizer::organizer::{ListenerData, SmartOrganizer};
use std::sync::{Arc, Mutex};

pub struct OrganizerState {
    pub organizer: Arc<Mutex<SmartOrganizer>>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(any(target_os = "windows", target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .plugin(PluginBuilder::default().build())
        .manage(OrganizerState {
            organizer: Arc::new(Mutex::new(SmartOrganizer::new())),
        })
        .invoke_handler(tauri::generate_handler![
            add_listener,
            organize,
            dir_len,
            update_listener,
            delete_listener,
            undo_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
