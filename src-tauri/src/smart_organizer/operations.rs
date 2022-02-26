use std::{
    io::{BufReader, Read},
    path::Path,
    path::PathBuf,
};

use chrono::Local;

use super::organizer::{Action, Log, Rule};

use memmem::{Searcher, TwoWaySearcher};
use tauri::{Runtime, Window};
use trash;

use glob::{glob, PatternError};

// TODO: Rename structs and package to something better
pub struct Organizer<'a, R: Runtime> {
    window: &'a Window<R>,
}

fn glob_search<T: AsRef<str>>(s: T, deep: &bool) -> Result<glob::Paths, PatternError> {
    if *deep {
        Ok(glob(&format!("{}/**/*", s.as_ref()))?)
    } else {
        Ok(glob(&format!("{}/*", s.as_ref()))?)
    }
}

enum BoolFunction {
    AND,
    OR,
}

impl<'a, Q: Runtime> Organizer<'a, Q> {
    fn bool_ops(items: &Vec<bool>, kind: BoolFunction) -> bool {
        match kind {
            BoolFunction::AND => items.iter().skip(1).fold(items[0], |a, b| a & b),
            BoolFunction::OR => items.iter().skip(1).fold(items[0], |a, b| a | b),
        }
    }

    pub fn from(window: &'a Window<Q>) -> Self {
        Self { window: window }
    }

    pub fn get_window(&self) -> &Window<Q> {
        self.window
    }

    pub fn sort<R: Runtime>(
        &self,
        id: &String,
        deep: &bool,
        path_str: &String,
        rules: &Vec<Rule>,
        actions: &Vec<Action>,
        selection: &String,
    ) {
        let search_result = glob_search(path_str, deep)
            .expect("Failed to parse glob pattern")
            .map(|b| b.expect("Failed to map and unwrap glob result"))
            .collect::<Vec<_>>();

        for pathbuf in search_result.iter() {
            let res = rules
                .iter()
                .map(|rule| match rule.condition.as_str() {
                    "Includes" => FileOperations::includes(&pathbuf, &rule.text, &rule.search_type),
                    "Not Includes" => {
                        !FileOperations::includes(&pathbuf, &rule.text, &rule.search_type)
                    }
                    "Exact Match" => {
                        FileOperations::exact_match(&pathbuf, &rule.text, &rule.search_type)
                    }

                    "Is Not" => {
                        !FileOperations::exact_match(&pathbuf, &rule.text, &rule.search_type)
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let file_ops = FileOperations::from(&pathbuf, &actions);

            if selection == "All of the following" {
                let is_match = Self::bool_ops(&res, BoolFunction::AND);
                if is_match {
                    file_ops.process(&is_match, id, self.get_window());
                }
            } else if selection == "Any of the following" {
                let is_match = Self::bool_ops(&res, BoolFunction::OR);
                if Self::bool_ops(&res, BoolFunction::OR) {
                    file_ops.process(&is_match, id, self.get_window());
                }
            }
        }
    }
}

struct FileOperations<'a> {
    path: &'a PathBuf,
    actions: &'a Vec<Action>,
}
// TODO do not perform action if file already exists

impl<'a> FileOperations<'a> {
    pub fn process<R: Runtime>(&self, is_match: &bool, id: &String, window: &Window<R>) {
        for Action(action, dest) in self.actions {
            let filename = Path::new(&self.path).file_name().unwrap();
            let to = Path::new(&dest).join(filename);
            let send_log_to = |to: &PathBuf, kind| {
                let timestamp = Local::now();
                window
                    .emit(
                        "logger",
                        Log::from(
                            id.to_owned(),
                            self.path.to_str().unwrap().to_owned(),
                            to.to_str().unwrap().to_owned(),
                            action.to_owned(),
                            timestamp.to_rfc2822(),
                        ),
                    )
                    .expect(&format!("Failed to send log from action: {}", kind));
            };

            if *is_match {
                match action.as_str() {
                    "COPY" => {
                        if self.path.exists() && !to.exists() {
                            match std::fs::copy(self.path, &to) {
                                Ok(_) => send_log_to(&to, action),
                                Err(e) => panic!("{:?}", e),
                            };
                        }
                    }
                    "MOVE" => {
                        if self.path.exists() && !to.exists() {
                            match std::fs::rename(self.path, &to) {
                                Ok(_) => send_log_to(&to, action),
                                Err(e) => panic!("{:?}", e),
                            }
                        }
                    }
                    "DELETE" => {
                        if self.path.exists() {
                            match trash::delete(&self.path) {
                                Ok(_) => send_log_to(&to, action),
                                Err(e) => panic!("{:?}", e),
                            }
                        }
                        // Send a signal to tauri
                    }
                    "UNLINK" => {
                        if self.path.exists() {
                            if self.path.is_dir() {
                                match std::fs::remove_dir(self.path) {
                                    Ok(_) => send_log_to(&to, action),
                                    Err(e) => println!("{:?}", e),
                                }
                            } else if self.path.is_file() {
                                match std::fs::remove_file(self.path) {
                                    Ok(_) => send_log_to(&to, action),
                                    Err(e) => println!("{:?}", e),
                                }
                            }
                        }
                    }

                    // "RENAME" => {
                    //     // Based on if rename is
                    //     let filename = Path::new(&self.path).file_name().unwrap();

                    //     std::fs::rename(self.path, to)
                    // }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
    }
}

impl<'a> FileOperations<'a> {
    fn parse_pathbuf(path: &PathBuf, kind: &str) -> String {
        if kind == "name" {
            path.file_name().unwrap().to_str().unwrap().to_lowercase()
        } else if kind == "ext" {
            if let Some(ext) = path.extension() {
                ext.to_str().unwrap().to_lowercase()
            } else {
                String::new()
            }
        } else {
            unreachable!()
        }
    }

    pub fn from(path: &'a PathBuf, actions: &'a Vec<Action>) -> Self {
        Self { path, actions }
    }

    pub fn includes(path: &PathBuf, to_match: &String, search_type: &String) -> bool {
        if search_type.contains("Name") {
            Self::parse_pathbuf(path, "name").contains(to_match)
        } else if search_type.contains("Extension") {
            Self::parse_pathbuf(path, "ext").contains(to_match)
        } else if search_type.contains("Path") {
            path.to_str().unwrap().contains(to_match)
        } else if search_type.contains("Content") {
            if path.is_file() {
                let search_sqc = TwoWaySearcher::new(&to_match.as_bytes());
                let f = std::fs::File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let mut content = Vec::new();

                reader.read_to_end(&mut content).unwrap();
                if let Some(_) = search_sqc.search_in(&content) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            unreachable!()
        }
    }
    pub fn exact_match(path: &PathBuf, to_match: &String, search_type: &String) -> bool {
        if search_type.contains("Name") {
            Self::parse_pathbuf(path, "name") == *to_match
        } else if search_type.contains("Extension") {
            Self::parse_pathbuf(path, "ext") == *to_match
        } else if search_type.contains("Path") {
            path.to_str().unwrap() == *to_match
        } else if search_type.contains("Content") {
            if path.is_file() {
                let f = std::fs::File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let mut content = Vec::new();

                reader.read_to_end(&mut content).unwrap();

                if content.len() != to_match.as_bytes().len() {
                    false
                } else {
                    content == to_match.as_bytes()
                }
            } else {
                false
            }
        } else {
            unreachable!()
        }
    }
}
