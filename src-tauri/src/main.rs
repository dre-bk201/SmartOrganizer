#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(unused)]

use tauri::Manager;
use tauri_plugin_store::PluginBuilder;
use window_shadows::set_shadow;

mod commands;
use commands::{add_listener, dir_len, remove_listener, start_receiver, update_listener};

// use org::smart_organizer::organizer::{ListenerData, SmartOrganizer};
use std::sync::{Arc, Mutex};

static mut IS_RECVING: bool = false;

pub struct OrganizerState {
    pub organizer: Arc<Mutex<smartorganizer::SmartOrganizer>>,
}

fn main() {
    let mut organizer = OrganizerState {
        organizer: Arc::new(Mutex::new(smartorganizer::SmartOrganizer::new())),
    };

    let organizer_arc = organizer.organizer.clone();

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();

            #[cfg(any(target_os = "windows", target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            organizer_arc.lock().unwrap().set_window(window);

            Ok(())
        })
        .plugin(PluginBuilder::default().build())
        .manage(organizer)
        .invoke_handler(tauri::generate_handler![
            add_listener,
            start_receiver,
            dir_len,
            update_listener,
            remove_listener,
            // undo_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
