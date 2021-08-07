use chrono::prelude::*;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use trash;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Logs {
  pub type_: String,
  pub index: usize,
  pub monitor: String,
  pub destination: String,
  pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListenerData {
  pub search: String,
  pub index: usize,
  pub title: String,
  pub type_: String,
  pub rule: String,
  pub logs: Vec<Logs>,
  pub deep: bool,
  pub enable_smart_organizer: bool,
  pub monitor_paths: Vec<String>,
  pub action_paths: HashMap<String, Vec<String>>,
}

// -----------------------------------------------------
pub struct FileOperations;

impl FileOperations {
  pub fn copy(filename: String, basedir: &Path, destination: &Path) -> bool {
    let from = basedir.join(&filename);
    let to = destination.join(&filename);
    let mut exists = true;

    if !to.exists() {
      match std::fs::copy(from, to) {
        Err(e) => panic!("Error with copying {:?} {:?}", basedir.join(&filename), e),
        Ok(_) => exists = false,
      };
    }
    exists
  }
  pub fn rename(filename: String, basedir: &Path, destination: &Path) -> bool {
    let from = basedir.join(&filename);
    let to = destination.join(&filename);
    let mut exists = true;

    if !to.exists() {
      match std::fs::rename(from, to) {
        Err(e) => panic!("Error with moving {:?} {:?}", basedir.join(&filename), e),
        Ok(_) => {
          exists = false;
        }
      };
    }
    exists
  }
  pub fn unlink(basedir: &Path) -> bool {
    let mut exists = true;

    if basedir.is_dir() {
      std::fs::remove_dir(basedir).unwrap_or_else(|_| exists = false);
    } else if basedir.is_file() {
      std::fs::remove_file(basedir).unwrap_or_else(|_| exists = false);
    }
    exists
  }
  // pub fn notify() {}

  pub fn walk(path: &str, deep: bool) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    if deep {
    } else {
      for entry in glob(path).expect("Failure reading path") {
        match entry {
          Ok(path) => paths.push(path),
          Err(err) => panic!("{:?}", err),
        }
      }
    }
    paths
  }

  pub fn sort(
    action: String,
    filename: String,
    basepath: String,
    destination: String,
    index: usize,
  ) -> Logs {
    let mut log: Logs = Logs::default();

    if action == "COPY" {
      let save_log = FileOperations::copy(
        filename.clone(),
        &Path::new(&basepath),
        &Path::new(&destination),
      );
      let timestamp = Local::now();
      if !save_log {
        log = Logs {
          index,
          monitor: format!("{}{}", basepath, filename),
          destination,
          timestamp: timestamp.to_rfc2822(),
          type_: action,
        };
      }
    } else if action == "MOVE" {
      let save_log = FileOperations::rename(
        filename.clone(),
        &Path::new(&basepath),
        &Path::new(&destination),
      );
      let timestamp = Local::now();
      if !save_log {
        log = Logs {
          index,
          monitor: format!("{}{}", basepath, filename),
          destination,
          timestamp: timestamp.to_rfc2822(),
          type_: action,
        };
      }
    } else if action == "DELETE" {
      trash::delete(&Path::new(&basepath).join(filename.clone())).unwrap();
      let timestamp = Local::now();

      log = Logs {
        index,
        monitor: format!("{}{}", basepath, filename),
        destination,
        timestamp: timestamp.to_rfc2822(),
        type_: action,
      };
    } else if action == "UNLINK" {
      let save_log = FileOperations::unlink(&Path::new(&basepath).join(&filename));
      let timestamp = Local::now();

      if !save_log {
        log = Logs {
          index,
          monitor: format!("{}{}", basepath, filename),
          destination,
          timestamp: timestamp.to_rfc2822(),
          type_: action,
        };
      }
    }

    log
  }
}
