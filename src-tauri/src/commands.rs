use std::{cell::RefCell, path::Path, rc::Rc};

use appconfig::AppConfigManager;
use organizer::{ListenerData, ListenerQuery, Log, LogQuery, DIRS};
use tauri::command;

use crate::State;

#[command]
pub async fn start_listener(state: tauri::State<'_, State>) -> Result<(), ()> {
    let mut o = state.inner().inner.lock().await;
    o.start();
    Ok(())
}

#[command]
pub async fn add_listener(
    listener: ListenerData,
    state: tauri::State<'_, State>,
) -> Result<String, ()> {
    let mut o = state.inner().inner.lock().await;
    Ok(o.add_listener(&listener).await.unwrap())
}

#[derive(serde::Serialize)]
pub struct LoadedState {
    listeners: Vec<ListenerData>,
    logs: Vec<Log>,
}

#[command]
pub async fn load_from_database(state: tauri::State<'_, State>) -> Result<LoadedState, ()> {
    let mut o = state.inner().inner.lock().await;
    let listeners = ListenerQuery::fetch_all(o.database())
        .await
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let logs = LogQuery::fetch_all(o.database()).await.unwrap().unwrap();

    for d in listeners.iter() {
        o.add_listener(d).await.unwrap();
    }

    Ok(LoadedState { listeners, logs })
}

#[command]
pub async fn remove_listener(
    listener: ListenerData,
    state: tauri::State<'_, State>,
) -> Result<Option<ListenerData>, ()> {
    let mut o = state.inner().inner.lock().await;
    Ok(o.remove_listener(&listener).await)
}

#[command]
pub async fn save_log(log: Log, state: tauri::State<'_, State>) -> Result<bool, ()> {
    let mut o = state.inner().inner.lock().await;
    Ok(LogQuery::insert_one(o.database(), &log).await)
}

#[command]
pub async fn update_listener(
    listener: ListenerData,
    state: tauri::State<'_, State>,
) -> Result<String, ()> {
    let mut o = state.inner().inner.lock().await;
    Ok(o.update_listener(&listener).await)
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AppConfig {
    theme: String,
    #[serde(rename = "titlebarStyle")]
    titlebarstyle: String,
    rounded: u32,
    pin: bool,
}

#[command]
pub fn load_settings() -> Option<AppConfig> {
    let path = DIRS.data_local_dir();
    println!("{path:?}");

    let manager = AppConfigManager::new(
        Rc::new(RefCell::from(AppConfig {
            pin: true,
            rounded: 15,
            theme: String::from("dark"),
            titlebarstyle: String::from("macos"),
        })),
        "smartorganizer",
        "smartorganizer",
    );

    match manager.load() {
        Ok(_) => Some(manager.data().borrow().clone()),
        Err(_) => None,
    }
}

#[command]
pub fn save_settings(settings: AppConfig) -> Option<i32> {
    let manager = AppConfigManager::new(
        Rc::new(RefCell::from(settings)),
        "smartorganizer",
        "smartorganizer",
    );

    match manager.save() {
        Ok(_) => Some(1),
        Err(_) => None,
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DirCount {
    files: i32,
    folders: i32,
}

#[command]
pub fn get_dir_count(path: &Path) -> DirCount {
    let mut dir_count = DirCount {
        files: 0,
        folders: 0,
    };

    match std::fs::read_dir(path) {
        Ok(dirs) => {
            for dir in dirs.flatten() {
                if dir.path().is_file() {
                    dir_count.files += 1;
                } else if dir.path().is_dir() {
                    dir_count.folders += 1;
                }
            }
        }
        Err(_) => {
            dir_count.files = -1;
            dir_count.folders = -1;
        }
    };

    dir_count
}
