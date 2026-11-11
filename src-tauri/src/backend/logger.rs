use tauri::Emitter;
use lazy_mut::lazy_mut;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    rc::Rc,
    sync::{LazyLock, Mutex},
};

#[derive(Default)]
pub struct Channel {
    window: Option<tauri::WebviewWindow>, // tauri::WebviewWindow
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn window(&self) -> &Option<tauri::WebviewWindow> {
        self.window.borrow()
    }

    pub fn init_window(&mut self, window: tauri::WebviewWindow) {
        self.window = Some(window);
    }

    pub fn broadcast<T>(&self, event: String, payload: T)
    where
        T: Serialize + Clone,
    {
        if let Some(window) = &self.window { let _ = window.emit(&event, payload); }
    }
}

pub static LOGGER: LazyLock<Mutex<Channel>> = LazyLock::new(|| Mutex::new(Channel::new()));

// lazy_mut! {
//     pub static mut LOGGER: Channel = Channel::new();
// }
