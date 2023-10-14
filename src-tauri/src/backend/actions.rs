use chrono::Local;

use crate::{logger::LOGGER, Log, LogLevel};

use super::{Action, ActionOpts};
use std::path::{Path, PathBuf};

pub struct FSActions;

impl FSActions {
    pub fn operate<P: AsRef<Path>>(
        id: &str,
        title: &str,
        path_found: P,
        actions: &[Action],
    ) -> Option<anyhow::Error> {
        let path_found = path_found.as_ref();

        println!("performing action on: {}", path_found.display());

        // returns if LOGGER is none
        if LOGGER.lock().unwrap().window().is_none() {
            println!("Crashing here");
            return None;
        }

        if !path_found.exists() {
            println!(
                "WARN: {} no longer exists, as such no action will take place",
                path_found.display()
            );
            LOGGER.lock().unwrap().broadcast("log".into(), Log {
                id: nanoid::nanoid!(),
                name: title.into(),
                level: LogLevel::WARN,
                message: format!("{} no longer exists, as such no action will take place", path_found.display()),
                path: path_found.to_path_buf(),
                timestamp: Local::now().to_rfc2822(),
                action: ActionOpts::DELETE,
                parent_id: id.into(),
                destination: PathBuf::from("N/A"),
            });

            return None;
        }

        let name = path_found
            .file_name()
            .expect("Unable to get filename of file")
            .to_str()
            .unwrap();

        let file_kind = if path_found.is_file() {
            "file"
        } else {
            "folder"
        };

        for Action { action, path } in actions.iter() {
            match action {
                ActionOpts::MOVE => {
                    let to = path.join(path_found.file_name()?);

                    if !to.exists() && path != path_found {
                        match std::fs::rename(path_found, &to) {
                            Ok(v) => {
                                println!(
                                    "{name} {file_kind} was successfully moved to {}",
                                    to.display()
                                );

                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::SUCCESS,
                                    message: format!("{name} {file_kind} was successfully moved to {}", to.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: to.to_path_buf()
                                });
                            }
                            Err(e) => {
                                println!("Failed to copy {name} to {}", to.display());
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::ERROR,
                                    message: format!("Failed to copy {name} to {}", to.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: to.to_path_buf()
                                });
                            }
                        }
                    }
                }
                ActionOpts::COPY => {
                    // issue with copying; There either needs to be an path_found:[action.1]
                    if path.is_file() {
                        match std::fs::copy(path_found, path) {
                            Ok(_) => {
                                println!(
                                    "{name} {file_kind} was successfully copied to {}",
                                    path.display()
                                );
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::SUCCESS,
                                    message: format!("{name} {file_kind} was successfully copied to {}", path.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: path.to_path_buf()
                                })
                            }
                            Err(_) => {
                                println!("Failed to copy {name} to {}", path.display());
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::ERROR,
                                    message: format!("Failed to copy {name} to {}", path.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: path.to_path_buf()
                                });
                            }
                        }
                    } else {
                        let to = path.join(path_found.file_name()?);
                        match std::fs::copy(path_found, &to) {
                            Ok(_) => {
                                println!(
                                    "{name} {file_kind} was successfully copied to {}",
                                    to.display()
                                );
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::SUCCESS,
                                    message: format!("{name} {file_kind} was successfully copied to {}", to.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: to.to_path_buf()
                                })
                            }
                            Err(_) => {
                                println!("Failed to copy folder {name} to {}", to.display());
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::ERROR,
                                    message: format!("Failed to copy folder {name} to {}", to.display()),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: to.to_path_buf()
                                });
                            }
                        }
                    }
                }
                ActionOpts::DELETE => match trash::delete(path_found) {
                    Ok(_) => {
                        println!("{name} {file_kind} was successfully deleted");
                        let log = Log {
                            id: nanoid::nanoid!(),
                            name: title.into(),
                            level: LogLevel::SUCCESS,
                            message: format!("{name} {file_kind} was successfully deleted"),
                            path: path_found.to_path_buf(),
                            timestamp: Local::now().to_rfc2822(),
                            action: action.clone(),
                            parent_id: id.into(),
                            destination: PathBuf::from("N/A"),
                        };

                        LOGGER.lock().unwrap().broadcast(String::from("log"), log);
                    }
                    Err(e) => {
                        println!("Failed to delete {name} {file_kind}");
                        LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                            id: nanoid::nanoid!(),
                            name: title.into(),
                            level: LogLevel::ERROR,
                            message: format!("Failed to delete {name} {file_kind}"),
                            path: path_found.to_path_buf(),
                            timestamp: Local::now().to_rfc2822(),
                            action: action.clone(),
                            parent_id: id.into(),
                            destination: PathBuf::from("N/A"),
                        });
                    }
                },
                ActionOpts::UNLINK => {
                    if path_found.is_file() {
                        match std::fs::remove_file(path_found) {
                            Ok(_) => {
                                println!("{name} {file_kind} was successfully unlinked");
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::SUCCESS,
                                    message: format!("{name} {file_kind} was successfully unlinked"),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: PathBuf::from("N/A"),
                                });
                            }
                            Err(e) => {
                                println!("Failed to unlink {name} {file_kind}");
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::ERROR,
                                    message: format!("Failed to unlink {name} {file_kind}"),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: PathBuf::from("N/A"),
                                });
                            }
                        }
                    } else {
                        match std::fs::remove_dir_all(path_found) {
                            Ok(_) => {
                                println!("{name} {file_kind} was successfully unlinked");
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::SUCCESS,
                                    message: format!("{name} {file_kind} was successfully unlinked"),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: PathBuf::from("N/A"),
                                });
                            }
                            Err(e) => {
                                println!("Failed to unlink {name} {file_kind}");
                                LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                                    id: nanoid::nanoid!(),
                                    name: title.into(),
                                    level: LogLevel::ERROR,
                                    message: format!("Failed to unlink {name} {file_kind}"),
                                    path: path_found.to_path_buf(),
                                    timestamp: Local::now().to_rfc2822(),
                                    action: action.clone(),
                                    parent_id: id.into(),
                                    destination: PathBuf::from("N/A"),
                                });
                            }
                        }
                    }
                }
                ActionOpts::RENAME => match std::fs::rename(path_found, path) {
                    Ok(_) => {
                        println!(
                            "{name} {file_kind} was successfully renamed to {}",
                            path.display()
                        );
                        LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                            id: nanoid::nanoid!(),
                            name: title.into(),
                            level: LogLevel::SUCCESS,
                            message: format!("{name} {file_kind} was successfully renamed to {}", path.display()),
                            path: path_found.to_path_buf(),
                            timestamp: Local::now().to_rfc2822(),
                            action: action.clone(),
                            parent_id: id.into(),
                            destination: path.to_path_buf(),
                        })
                    }
                    Err(e) => {
                        println!("Failed to rename {file_kind} {name} to {}", path.display());
                        LOGGER.lock().unwrap().broadcast(String::from("log"), Log {
                            id: nanoid::nanoid!(),
                            name: title.into(),
                            level: LogLevel::ERROR,
                            message: format!("Failed to rename {file_kind} {name} to {}", path.display()),
                            path: path_found.to_path_buf(),
                            timestamp: Local::now().to_rfc2822(),
                            action: action.clone(),
                            parent_id: id.into(),
                            destination: path.to_path_buf(),
                        });
                    }
                },
            }
        }

        None
    }
}
