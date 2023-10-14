#![feature(lazy_cell)]
use lazy_mut::lazy_mut;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, rc::Rc, sync::{LazyLock, Mutex}};

#[derive(Default)]
pub struct Channel {
    window: Option<tauri::Window>, // tauri::Window
}

impl Channel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn window(&self) -> &Option<tauri::Window> {
        self.window.borrow()
    }

    pub fn init_window(&mut self, window: tauri::Window) {
        self.window = Some(window);
    }

    pub fn broadcast<T>(&self, event: String, payload: T)
    where
        T: Serialize + Clone,
    {
        self.window.as_ref().unwrap().emit(&event, payload);
    }
}

pub static LOGGER: LazyLock<Mutex<Channel>> = LazyLock::new(|| Mutex::new(Channel::new()));

// lazy_mut! {
//     pub static mut LOGGER: Channel = Channel::new();
// }
