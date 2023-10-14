#![feature(lazy_cell)]
mod backend;
mod database;
#[cfg(test)]
mod fakedata;
mod models;
mod schemas;
#[cfg(test)]
mod test;

use std::sync::LazyLock;

use directories::ProjectDirs;

pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    let dir = directories::ProjectDirs::from("io", "smartorganizer", "smartorganizer").unwrap();
    std::fs::create_dir_all(dir.data_local_dir()).unwrap();
    dir
});

pub use backend::*;
pub use database::*;
#[cfg(test)]
pub use fakedata::*;
pub use models::*;
pub use schemas::*;
