use std::path::PathBuf;

use crate::{ListenerData, OrganizerState};
use org::smart_organizer::{operations::FileOperations, organizer::Action};
use tauri::{Manager, Runtime, State, Window};

#[tauri::command]
pub fn add_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    s.push(listener)
}

#[tauri::command]
pub fn update_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    s.replace(listener);
}

#[tauri::command]
pub fn delete_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    s.delete(listener)
}

#[tauri::command(async)]
pub fn organize<R: Runtime>(state: State<OrganizerState>, window: Window<R>) -> () {
    let mut s = state.organizer.lock().unwrap();
    s.organize(&window)
}

#[tauri::command]
pub fn dir_len(path: PathBuf) -> i32 {
    if path.exists() {
        let paths = std::fs::read_dir(path).unwrap();

        return paths.collect::<Vec<_>>().len() as i32;
    }
    -1
}

#[tauri::command]
pub fn undo_action(id: String, from: String, action: Action, handle: tauri::AppHandle) {
    let window = handle.get_window("main").unwrap();
    let Action(action, to) = action;
    let to = PathBuf::from(&to);

    match action.as_str() {
        "RENAME" | "MOVE" => {
            let actions = vec![Action::from("RENAME", &from)];
            FileOperations::from(&id, &to, &actions, &window).process(true, false)
        }

        "COPY" => {
            let actions = vec![Action::from("DELETE", &from)];
            FileOperations::from(&id, &to, &actions, &window).process(true, false)
        }
        _ => (),
    }
}
