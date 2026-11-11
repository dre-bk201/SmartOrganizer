mod backend;
mod database;

mod models;
mod schemas;


use std::sync::LazyLock;

use directories::ProjectDirs;

pub static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    let dir = directories::ProjectDirs::from("io", "smartorganizer", "smartorganizer").unwrap();
    std::fs::create_dir_all(dir.data_local_dir()).unwrap();
    dir
});

pub use backend::*;
pub use database::*;

pub use models::*;
pub use schemas::*;

#[cfg(test)]
mod test;
