#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_store::PluginBuilder;

mod commands;
use commands::{add_listener, delete_listener, dir_len, organize, undo_action, update_listener};

use org::smart_organizer::organizer::{ListenerData, SmartOrganizer};
use std::sync::Mutex;

pub struct OrganizerState {
    pub organizer: Mutex<SmartOrganizer>,
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .manage(OrganizerState {
            organizer: Mutex::new(SmartOrganizer::new()),
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
