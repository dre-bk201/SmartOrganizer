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
use std::fs::metadata;

// TODO: Rename structs and package to something better
pub struct Organizer<'a, R: Runtime> {
    window: &'a Window<R>,
}

fn glob_search<T: AsRef<str>>(s: T, deep: &bool) -> Result<glob::Paths, PatternError> {
    if *deep {
        let c = glob(&format!("{}/**/*", s.as_ref()))?;
        Ok(c)
    } else {
        let c = glob(&format!("{}/*", s.as_ref()))?;
        Ok(c)
    }
}

enum BooleanOP {
    AND,
    OR,
}

impl<'a, Q: Runtime> Organizer<'a, Q> {
    fn bool_ops(items: &Vec<bool>, kind: BooleanOP) -> bool {
        match kind {
            BooleanOP::AND => items.iter().skip(1).fold(items[0], |a, b| a & b),
            BooleanOP::OR => items.iter().skip(1).fold(items[0], |a, b| a | b),
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
        let c = glob_search(path_str, deep)
            .expect("Failed to parse glob pattern")
            .map(|b| b.expect("Failed to map and unwrap glob result"))
            .collect::<Vec<_>>();

        for pathbuf in c.iter() {
            let res = rules
                .iter()
                .map(|rule| match rule.condition.as_str() {
                    "Includes" => ConditionOps::includes(&pathbuf, &rule.text, &rule.search_type),
                    "Not Includes" => {
                        ConditionOps::not_includes(&pathbuf, &rule.text, &rule.search_type)
                    }
                    "Exact Match" => {
                        ConditionOps::exact_match(&pathbuf, &rule.text, &rule.search_type)
                    }
                    "Is Not" => ConditionOps::is_not(&pathbuf, &rule.text, &rule.search_type),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();

            let file_ops = FileOps::from(&pathbuf, &actions);

            if selection == "All of the following" {
                if Self::bool_ops(&res, BooleanOP::AND) {
                    file_ops.process(&res, id, self.get_window());
                }
            } else if selection == "Any of the following" {
                if Self::bool_ops(&res, BooleanOP::OR) {
                    file_ops.process(&res, id, self.get_window());
                }
            }
        }
    }
}

struct ConditionOps;

impl ConditionOps {
    fn parse_pathbuf(path: &PathBuf, kind: &str) -> String {
        if kind == "name" {
            path.file_name().unwrap().to_str().unwrap().to_lowercase()
        } else {
            path.to_str().unwrap().to_lowercase()
        }
    }

    pub fn includes(pathbuf: &PathBuf, to_match: &String, search_type: &String) -> bool {
        match &search_type[..] {
            "File Name" => {
                if pathbuf.is_file() {
                    Self::parse_pathbuf(pathbuf, "name").contains(&to_match.to_lowercase())
                } else {
                    false
                }
            }
            "Folder Name" => {
                if pathbuf.is_dir() {
                    Self::parse_pathbuf(pathbuf, "name").contains(&to_match.to_lowercase())
                } else {
                    false
                }
            }
            "File Extension" => {
                if pathbuf.is_file() {
                    if let Some(f_ext) = pathbuf.extension() {
                        f_ext
                            .to_str()
                            .unwrap()
                            .to_lowercase()
                            .contains(&to_match.to_lowercase())
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            "File Content" => {
                if pathbuf.is_file() {
                    let search_sqc = TwoWaySearcher::new(&to_match.as_bytes());
                    let f = std::fs::File::open(&pathbuf).unwrap();
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
            }
            "Path Name" => pathbuf.to_str().unwrap().contains(to_match),

            // "File Size" => {
            //     if metadata(&pathbuf).unwrap().is_file() {
            //         if to_match.parse::<u64>().unwrap() > metadata(&pathbuf).unwrap().len() {}
            //     }
            // }
            _ => unreachable!(),
        }
    }
    pub fn not_includes(pathbuf: &PathBuf, to_match: &String, operation: &String) -> bool {
        match operation.as_str() {
            "File Name" => {
                if pathbuf.is_file() {
                    !Self::includes(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "Folder Name" => {
                if metadata(&pathbuf).unwrap().is_dir() {
                    !Self::includes(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "File Extension" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    !Self::includes(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "File Content" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    !Self::includes(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "Path Name" => !Self::includes(&pathbuf, &to_match, &operation),
            // "File Size" => {
            //     if metadata(&pathbuf).unwrap().is_file() {
            //         if to_match.parse::<u64>().unwrap() > metadata(&pathbuf).unwrap().len() {}
            //     }
            // }
            _ => unreachable!(),
        }
    }
    pub fn exact_match(pathbuf: &PathBuf, to_match: &String, operation: &String) -> bool {
        match &operation[..] {
            "File Name" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    let fname = pathbuf.file_name().unwrap().to_str().unwrap();

                    fname == to_match
                } else {
                    false
                }
            }
            "Folder Name" => {
                if metadata(&pathbuf).unwrap().is_dir() {
                    let folder_name = pathbuf.file_name().unwrap().to_str().unwrap();

                    folder_name == to_match
                } else {
                    false
                }
            }
            "File Extension" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    let f_ext = pathbuf.extension().unwrap().to_str().unwrap();

                    f_ext == to_match
                } else {
                    false
                }
            }
            "File Content" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    let f = std::fs::File::open(&pathbuf).unwrap();
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
            }
            "Path Name" => pathbuf.to_str().unwrap().contains(to_match),
            // "File Size" => {
            //     if metadata(&pathbuf).unwrap().is_file() {
            //         if to_match.parse::<u64>().unwrap() > metadata(&pathbuf).unwrap().len() {}
            //     }
            // }
            _ => unreachable!(),
        }
    }
    pub fn is_not(pathbuf: &PathBuf, to_match: &String, operation: &String) -> bool {
        match &operation[..] {
            "File Name" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    !Self::exact_match(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "Folder Name" => {
                if metadata(&pathbuf).unwrap().is_dir() {
                    !Self::exact_match(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "File Extension" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    !Self::exact_match(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "File Content" => {
                if metadata(&pathbuf).unwrap().is_file() {
                    !Self::exact_match(&pathbuf, &to_match, &operation)
                } else {
                    false
                }
            }
            "Path Name" => pathbuf.to_str().unwrap().contains(to_match),
            // "File Size" => {
            //     if metadata(&pathbuf).unwrap().is_file() {
            //         if to_match.parse::<u64>().unwrap() > metadata(&pathbuf).unwrap().len() {}
            //     }
            // }
            _ => unreachable!(),
        }
    }
}

struct FileOps<'a> {
    path: &'a PathBuf,
    actions: &'a Vec<Action>,
}
// TODO do not perform action if file already exists

impl<'a> FileOps<'a> {
    pub fn from(path: &'a PathBuf, actions: &'a Vec<Action>) -> Self {
        Self { path, actions }
    }
    pub fn process<R: Runtime>(&self, matches: &Vec<bool>, id: &String, window: &Window<R>) {
        for action in self.actions.iter() {
            println!("Performing Action: {:?}", action);
            println!("Matches: {:?}", matches);
            for &result in matches.iter() {
                if result {
                    // TODO refactor
                    match &action.0[..] {
                        "COPY" => {
                            let filename = Path::new(&self.path).file_name().unwrap();
                            let to = Path::new(&action.1).join(filename);
                            if self.path.exists() && !to.exists() {
                                match std::fs::copy(self.path, &to) {
                                    Ok(_) => {
                                        let timestamp = Local::now();
                                        window
                                            .emit(
                                                "logger",
                                                Log::from(
                                                    id.to_owned(),
                                                    self.path.to_str().unwrap().to_owned(),
                                                    to.to_str().unwrap().to_owned(),
                                                    action.0.to_owned(),
                                                    timestamp.to_rfc2822(),
                                                ),
                                            )
                                            .expect("Failed to send log from action: COPY");
                                    }

                                    Err(e) => panic!("{:?}", e),
                                };
                            }
                        }
                        "MOVE" => {
                            let filename = Path::new(&self.path).file_name().unwrap();
                            let to = Path::new(&action.1).join(filename);
                            if self.path.exists() && !to.exists() {
                                match std::fs::rename(self.path, &to) {
                                    Ok(_) => {
                                        let timestamp = Local::now();
                                        window
                                            .emit(
                                                "logger",
                                                Log::from(
                                                    id.to_owned(),
                                                    self.path.to_str().unwrap().to_owned(),
                                                    to.to_str().unwrap().to_owned(),
                                                    action.0.to_owned(),
                                                    timestamp.to_rfc2822(),
                                                ),
                                            )
                                            .expect("Failed to send log from action: MOVE");
                                    }
                                    Err(e) => panic!("{:?}", e),
                                }
                            }
                        }
                        "DELETE" => {
                            if self.path.exists() {
                                // println!("Deleting: {:?}", matches);
                                // let c = window.clone();
                                match trash::delete(&self.path) {
                                    Ok(_) => {
                                        let timestamp = Local::now();
                                        window
                                            .clone()
                                            .emit(
                                                "logger",
                                                Log::from(
                                                    id.to_owned(),
                                                    self.path.to_str().unwrap().to_owned(),
                                                    "".to_owned(),
                                                    action.0.to_owned(),
                                                    timestamp.to_rfc2822(),
                                                ),
                                            )
                                            .expect("Failed to send log from action: DELETE");
                                    }
                                    Err(e) => panic!("{:?}", e),
                                }
                            }
                            // Send a signal to tauri
                        }
                        "UNLINK" => {
                            if self.path.exists() {
                                if metadata(self.path).unwrap().is_dir() {
                                    match std::fs::remove_dir(self.path) {
                                        Ok(_) => {
                                            let timestamp = Local::now();
                                            window
                                                .emit(
                                                    "logger",
                                                    Log::from(
                                                        id.to_owned(),
                                                        self.path.to_str().unwrap().to_owned(),
                                                        "".to_owned(),
                                                        action.0.to_owned(),
                                                        timestamp.to_rfc2822(),
                                                    ),
                                                )
                                                .expect("Failed to send log from action: UNLINK");
                                        }
                                        Err(e) => println!("{:?}", e),
                                    }
                                } else if metadata(self.path).unwrap().is_file() {
                                    match std::fs::remove_file(self.path) {
                                        Ok(_) => {
                                            let timestamp = Local::now();
                                            window
                                                .emit(
                                                    "logger",
                                                    Log::from(
                                                        id.to_owned(),
                                                        self.path.to_str().unwrap().to_owned(),
                                                        "".to_owned(),
                                                        action.0.to_owned(),
                                                        timestamp.to_rfc2822(),
                                                    ),
                                                )
                                                .expect("Failed to send log from action: UNLINK");
                                        }
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
                        _ => unreachable!(),
                    }
                    // }
                }
            }
        }
    }
}
