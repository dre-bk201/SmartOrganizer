use super::shared::{FileOperations, ListenerData, Logs};

use std::collections::HashMap;
// TODO: Fix copying so that it doesn't rewrite when copying

pub struct FileName<'a> {
  monitor_paths: &'a Vec<String>,
  search: String,
  deep: bool,
  index: usize,
  rule: String,
  destinations: &'a HashMap<String, Vec<String>>,
}

impl<'a> FileName<'a> {
  pub fn new(data: &'a mut ListenerData) -> Self {
    Self {
      monitor_paths: &data.monitor_paths,
      deep: data.deep,
      search: data.search.clone(),
      index: data.index,
      rule: data.rule.clone(),
      destinations: &data.action_paths,
    }
  }

  pub fn sort(&mut self) -> Vec<Logs> {
    let mut log: Vec<Logs> = Vec::new();

    for basepath in self.monitor_paths {
      for path in FileOperations::walk(&format!("{}*", basepath), self.deep) {
        if let Some(file) = path.file_name() {
          let filename = file.to_str().unwrap().to_owned();
          let searches: Vec<&str> = self.search.split_whitespace().collect();
          let mut contains = false;

          for search in searches {
            if filename.contains(search) {
              contains = true;
              break;
            }
            contains = false;
          }

          for (destination, value) in self.destinations {
            for action in value {
              if self.rule == "Not Includes".to_owned() {
                // ignore if extension is ignore list
                if contains {
                  continue;
                }
              } else {
                if !contains {
                  // ignores if extension is not in ignore list
                  continue;
                }
              }

              log.push(FileOperations::sort(
                action.to_owned(),
                filename.to_owned(),
                basepath.to_owned(),
                destination.to_owned(),
                self.index,
              ));

              // (self.logs) = v;
              // self.logs = v.clone();
            }
          }
        }
      }
    }
    log
  }
}

#[derive(Debug)]
pub struct FileExtension<'a> {
  extensions: Vec<String>,
  monitor_paths: &'a Vec<String>,
  deep: bool,
  index: usize,
  rule: String,
  destinations: &'a HashMap<String, Vec<String>>,
}

impl<'a> FileExtension<'a> {
  pub fn new(data: &'a mut ListenerData) -> Self {
    let extensions: Vec<String> = data.search.split(" ").map(|s| s.to_owned()).collect();

    Self {
      extensions,
      index: data.index,
      monitor_paths: &data.monitor_paths,
      deep: data.deep,
      rule: data.rule.clone(),
      destinations: &data.action_paths,
    }
  }

  // test if all the closures works on folders and not only files [N.B] -> except unlink

  pub fn sort(&mut self) -> Vec<Logs> {
    let mut log: Vec<Logs> = Vec::new();
    for basepath in self.monitor_paths {
      // Walks the basepath for all file and folder names
      for path in FileOperations::walk(&format!("{}*", basepath), false) {
        if let Some(extension) = path.as_path().extension() {
          let contains = self
            .extensions
            .contains(&extension.to_str().unwrap().to_owned());

          let filename = match path.file_name() {
            Some(name) => name.to_str().unwrap(),
            None => "Empty",
          };

          for (destination, value) in self.destinations {
            for action in value {
              if self.rule == "Not Includes" {
                // ignore if extension is ignore list
                if contains {
                  continue;
                }
              } else {
                if !contains {
                  // ignores if extension is not in ignore list
                  continue;
                }
              }

              log.push(FileOperations::sort(
                action.to_owned(),
                filename.to_owned(),
                basepath.to_owned(),
                destination.to_owned(),
                self.index,
              ));
            }
          }
        }
      }
    }
    log
  }
}
