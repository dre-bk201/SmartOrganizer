use std::path::PathBuf;

use smartorganizer::ListenerData;
use tauri::State;

use crate::{OrganizerState, IS_RECVING};

#[tauri::command]
pub fn add_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut organizer = state.organizer.lock().unwrap();
    organizer.add_listener(listener);
}

#[tauri::command]
pub fn start_receiver(state: State<OrganizerState>) {
    unsafe {
        if !IS_RECVING {
            dbg!("Starting receiver again");
            state.organizer.lock().unwrap().listen();
            IS_RECVING = true;
        }
    }
}

#[tauri::command]
pub fn update_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut organizer = state.organizer.lock().unwrap();

    organizer.update_listener(listener);
}

#[tauri::command]
pub fn remove_listener(listener: ListenerData, state: State<OrganizerState>) {
    let mut s = state.organizer.lock().unwrap();
    s.remove_listener(&listener);
}

#[tauri::command]
pub fn dir_len(path: PathBuf) -> i32 {
    if path.exists() {
        let paths = std::fs::read_dir(path).unwrap();

        return paths.collect::<Vec<_>>().len() as i32;
    }
    -1
}

// #[tauri::command]
// pub fn undo_action(id: String, from: String, action: Action, handle: tauri::AppHandle) {
//     let window = handle.get_window("main").unwrap();
//     let Action(action, to) = action;
//     let to = PathBuf::from(&to);

//     match action.as_str() {
//         "RENAME" | "MOVE" => {
//             let actions = vec![Action::from("RENAME", &from)];
//             FileOperations::from(&id, &to, &actions, &window, true, false).process(true);
//         }

//         "COPY" => {
//             let actions = vec![Action::from("DELETE", &from)];
//             FileOperations::from(&id, &to, &actions, &window, true, false).process(true);
//         }
//         _ => (),
//     }
// }
