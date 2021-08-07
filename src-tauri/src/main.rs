#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use directories::ProjectDirs;
use directory_tree::{filetree, FileNode};
use organizer::modules::{organizer::SmartOrganizer, shared::ListenerData, shared::Logs};
use std::sync::Mutex;
use tauri::{command, State};

// "devPath": "./src",

#[derive(Default)]
struct OrganizerState<'a> {
  organizer: Mutex<SmartOrganizer<'a>>,
}

#[command]
fn add_listener<'a>(listener: ListenerData, state: State<OrganizerState<'a>>) {
  let mut state = state.organizer.lock().unwrap();
  (*state).push(listener);
}

#[command]
fn update_listener<'a>(listener: ListenerData, state: State<OrganizerState<'a>>) {
  let mut state = state.organizer.lock().unwrap();
  println!("{:#?}", state);

  (*state).replace(listener);
}

#[command]
fn remove_listener<'a>(index: usize, state: State<OrganizerState<'a>>) {
  let mut state = state.organizer.lock().unwrap();
  (*state).remove_at(index);
}

#[command]
fn run_organizer<'a>(state: State<OrganizerState<'a>>) -> Vec<Vec<Logs>> {
  let mut s = state.organizer.lock().unwrap();
  (*s).organize()
}

#[command]
fn save_state<'a>(data: Vec<ListenerData>) {
  if let Some(app_dir) = ProjectDirs::from("com", "h4ck3r", "SmartOrganizer") {
    println!("Saving state");

    let db_path = app_dir.config_dir().join("db.json");
    if !db_path.exists() {
      std::fs::create_dir(&app_dir.config_dir()).unwrap();
    }
    let contents = serde_json::to_string(&data).unwrap();
    println!("Saving file: {:?}", contents);
    std::fs::write(db_path.to_str().unwrap(), &contents).unwrap();
  }
}

#[command]
fn load_state<'a>() -> Vec<ListenerData> {
  let mut return_val = Vec::new();
  if let Some(app_dir) = ProjectDirs::from("com", "h4ck3r", "SmartOrganizer") {
    let db_path = app_dir.config_dir().join("db.json");

    if db_path.exists() {
      let file = std::fs::File::open(db_path).unwrap();
      let reader = std::io::BufReader::new(file);
      match serde_json::from_reader(reader) {
        Ok(val) => return_val = val,
        Err(e) => panic!("Error parsing with serde_json: {:?}", e),
      }
    }
  }
  return_val
}

#[command(async)]
fn walk_dir(path: String) -> Option<FileNode> {
  std::thread::spawn(move || {
    let path = &std::path::Path::new(&path);
    if path.exists() {
      Some(filetree(path).unwrap())
    } else {
      None
    }
  })
  .join()
  .unwrap()
}

fn main() {
  tauri::Builder::default()
    .manage(OrganizerState {
      organizer: Mutex::new(SmartOrganizer::new()),
    })
    .invoke_handler(tauri::generate_handler![
      add_listener,
      remove_listener,
      run_organizer,
      update_listener,
      walk_dir,
      load_state,
      save_state
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
