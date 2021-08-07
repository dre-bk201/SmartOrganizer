#![allow(dead_code)]

use super::models::{FileExtension, FileName};
use super::shared::{ListenerData, Logs};

use maplit::hashmap;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;

#[derive(Debug, Default)]
pub struct SmartOrganizer<'a> {
  pub options: Vec<ListenerData>,
  default_options: HashMap<&'a str, Vec<&'a str>>,
  presets: Vec<String>,
  pub combinations: Vec<HashMap<&'a str, &'a str>>,
}

impl<'a> SmartOrganizer<'a> {
  pub fn organize(&mut self) -> Vec<Vec<Logs>> {
    // println!("self.options: {:#?}", self.options);
    let mut logs: Vec<Vec<Logs>> = Vec::new();
    for option in self.options.iter_mut() {
      for combination in &self.combinations {
        for (_, actions) in &option.action_paths.clone() {
          for action in actions {
            let option_map = hashmap! {
              "type".to_owned() => option.type_.clone(),
              "rule".to_owned() => option.rule.clone(),
              "action".to_owned() => action.to_owned(),
            };
            if SmartOrganizer::compare_options(combination, &option_map) {
              match &SmartOrganizer::determine_preset(combination)[..] {
                "111" | "112" | "113" | "114" | "121" | "122" | "123" | "124" => {
                  // let
                  logs.push(FileExtension::new(option).sort());
                }
                "211" | "212" | "213" | "214" | "221" | "222" | "223" | "224" => {
                  logs.push(FileName::new(option).sort());
                }
                _ => {
                  println!("The other thingy")
                }
              }
            }
          }
        }
      }
    }
    logs
  }

  pub fn push(&mut self, listener: ListenerData) {
    self.options.push(listener);
  }

  pub fn replace(&mut self, listener: ListenerData) {
    let index = listener.index;
    self.options[index] = listener;
  }

  pub fn remove_at(&mut self, index: usize) {
    self.options.remove(index);

    for (idx, option) in self.options.iter_mut().enumerate() {
      option.index = idx;
    }
  }
}

impl<'a> SmartOrganizer<'a> {
  pub fn new() -> Self {
    let default_options: HashMap<&'a str, Vec<&'a str>> = hashmap! {
        "type" => vec!["FileExtension-1", "FileName-2"],
        "rule" => vec!["Includes-1", "Not Includes-2"],
        "action" => vec!["COPY-1", "MOVE-2", "DELETE-3", "UNLINK-4"]
    };

    let mut combinations: Vec<HashMap<&'a str, &'a str>> = Vec::new();
    println!("In the new Function");

    for &_type in default_options.get("type").unwrap() {
      for &rule in default_options.get("rule").unwrap() {
        for &action in default_options.get("action").unwrap() {
          let map: HashMap<&'a str, &'a str> = hashmap! {
              "type" => _type,
              "rule" => rule,
              "action" => action
          };
          combinations.push(map);
        }
      }
    }

    Self {
      options: Vec::new(),
      presets: Vec::new(),
      default_options,
      combinations,
    }
  }
}

impl<'a> SmartOrganizer<'a> {
  fn split(object: &HashMap<&'a str, &'a str>, key: &'a str) -> Vec<&'a str> {
    object.get(key).unwrap().split("-").collect()
  }

  fn compare_options(
    default_option: &HashMap<&'a str, &'a str>,
    option: &HashMap<String, String>,
  ) -> bool {
    let default_type = SmartOrganizer::split(default_option, "type")[0];
    let default_rule = SmartOrganizer::split(default_option, "rule")[0];
    let default_action = SmartOrganizer::split(default_option, "action")[0];

    let option_type = option.get("type").unwrap();
    let option_rule = option.get("rule").unwrap();
    let option_action = option.get("action").unwrap();

    if default_type == option_type {
      if default_rule == option_rule {
        if default_action == option_action {
          return true;
        }
      }
    }

    false
  }

  fn determine_preset(combination: &'a HashMap<&'a str, &'a str>) -> String {
    let preset = format!(
      "{}{}{}",
      SmartOrganizer::split(&combination, "type")[1],
      SmartOrganizer::split(&combination, "rule")[1],
      SmartOrganizer::split(&combination, "action")[1]
    );
    preset
  }

  fn dir(path: &'a str) -> Vec<String> {
    let directory = read_dir(&Path::new(path));
    let mut paths: Vec<String> = vec![];

    for dir in directory.unwrap() {
      if let Ok(res) = dir {
        if let Some(x) = res.path().as_path().to_str() {
          paths.push(x.to_owned());
        }
      }
    }
    paths
  }
}

//      Locations to Monitor
// ["/home/h4ck3r/Downloads", "/home/h4ck3r/Desktop"]

//      Search Types
// ["Filename", "FileExtension"]

//      Conditions
// ["Includes", "Not Includes"]

//      Data
// A text to split for whitespace for different types

//      Actions to take
// ["MOVE", "UMLINK", "COPY", "DELETE", "NOTIFY"]

//      SmartOrganizing is enabled
// enable_smart_organizer: false
