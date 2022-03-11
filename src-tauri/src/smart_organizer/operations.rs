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
        Self { window }
    }

    pub fn get_window(&self) -> &Window<Q> {
        self.window
    }

    pub fn sort<R: Runtime>(
        &self,
        parent_id: &String,
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
                    "Includes" => {
                        FileOperations::<R>::includes(&pathbuf, &rule.text, &rule.search_type)
                    }
                    "Not Includes" => {
                        !FileOperations::<R>::includes(&pathbuf, &rule.text, &rule.search_type)
                    }
                    "Exact Match" => {
                        FileOperations::<R>::exact_match(&pathbuf, &rule.text, &rule.search_type)
                    }

                    "Is Not" => {
                        !FileOperations::<R>::exact_match(&pathbuf, &rule.text, &rule.search_type)
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let file_ops = FileOperations::from(&parent_id, &pathbuf, &actions, self.window);

            if selection == "All of the following" {
                let is_match = Self::bool_ops(&res, BoolFunction::AND);
                if is_match {
                    file_ops.process(is_match, true);
                }
            } else if selection == "Any of the following" {
                let is_match = Self::bool_ops(&res, BoolFunction::OR);
                if Self::bool_ops(&res, BoolFunction::OR) {
                    file_ops.process(is_match, true);
                }
            }
        }
    }
}

pub struct FileOperations<'a, R: Runtime> {
    parent_id: &'a String,
    path: &'a PathBuf,
    actions: &'a Vec<Action>,
    window: &'a Window<R>,
}
// TODO do not perform action if file already exists

use uuid::Uuid;

impl<'a, Q: Runtime> FileOperations<'a, Q> {
    // pub fn undo

    pub fn send_log_to(&self, to: &PathBuf, kind: &String) {
        let timestamp = Local::now();
        self.window
            .emit(
                "logger",
                Log::from(
                    self.parent_id.to_owned(),
                    Uuid::new_v4().to_string(),
                    self.path.to_str().unwrap().to_owned(),
                    to.to_str().unwrap().to_owned(),
                    kind.to_owned(),
                    timestamp.to_rfc2822(),
                ),
            )
            .expect(&format!("Failed to send log from action: {}", kind));
    }

    pub fn match_against(
        &self,
        to: &PathBuf,
        action: &String,
        dest: Option<&String>,
        enable_logging: bool,
    ) {
        match action.as_str() {
            "COPY" => {
                if self.path.exists() && !to.exists() {
                    match std::fs::copy(self.path, &to) {
                        Ok(_) => {
                            if enable_logging {
                                self.send_log_to(&to, action)
                            }
                        }
                        Err(e) => println!("Error {} {:?}", action, e),
                    };
                }
            }
            "MOVE" => {
                if self.path.exists() && !to.exists() {
                    println!("{} - {}", self.path.display(), to.display());
                    match std::fs::rename(self.path, &to) {
                        Ok(_) => {
                            if enable_logging {
                                self.send_log_to(&to, action)
                            }
                        }
                        Err(e) => println!("Error {} {:?}", action, e),
                    }
                }
            }
            "DELETE" => {
                if self.path.exists() {
                    match trash::delete(&self.path) {
                        Ok(_) => {
                            if enable_logging {
                                self.send_log_to(&to, action)
                            }
                        }
                        Err(e) => println!("Error {} {:?}", action, e),
                    }
                }
            }
            "UNLINK" => {
                if self.path.exists() {
                    if self.path.is_dir() {
                        match std::fs::remove_dir(self.path) {
                            Ok(_) => {
                                if enable_logging {
                                    self.send_log_to(&to, action)
                                }
                            }

                            Err(e) => println!("Error {} {:?}", action, e),
                        }
                    } else if self.path.is_file() {
                        match std::fs::remove_file(self.path) {
                            Ok(_) => {
                                if enable_logging {
                                    self.send_log_to(&to, action)
                                }
                            }
                            Err(e) => println!("Error {} {:?}", action, e),
                        }
                    }
                }
            }

            "RENAME" => {
                let to = PathBuf::from(dest.unwrap());
                if self.path.exists() {
                    match std::fs::rename(self.path, &to) {
                        Ok(_) => {
                            if enable_logging {
                                self.send_log_to(&to, action)
                            }
                        }
                        Err(e) => println!("Error {} {:?}", action, e),
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
    pub fn process(&self, is_match: bool, enable_logging: bool) {
        for Action(action, dest) in self.actions {
            let filename = Path::new(&self.path).file_name().unwrap();
            let to = Path::new(&dest).join(filename);

            if is_match {
                self.match_against(&to, action, Some(dest), enable_logging);
            }
        }
    }
}

impl<'a, Q: Runtime> FileOperations<'a, Q> {
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

    pub fn from(
        parent_id: &'a String,
        path: &'a PathBuf,
        actions: &'a Vec<Action>,
        window: &'a Window<Q>,
    ) -> Self {
        Self {
            parent_id,
            path,
            actions,
            window,
        }
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
                let file = std::fs::File::open(&path).unwrap();
                let mut reader = BufReader::new(file);
                let mut content = Vec::new();

                let mut exists = false;

                if let Some(_) = reader.read_to_end(&mut content).ok() {
                    if let Some(_) = search_sqc.search_in(&content) {
                        exists = true;
                    }
                }

                exists
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
