#![allow(unused)]
mod actions;
pub mod logger;

use actions::*;
use logger::*;

use sqlx::{Connection, SqliteConnection};

use super::ListenerData;

use notify_debouncer_full::{
    new_debouncer,
    notify::{
        self,
        event::{DataChange, ModifyKind, RenameMode},
        Config, Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
    },
    DebouncedEvent, Debouncer, FileIdMap,
};
use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
    time::Duration,
};

use crate::{database::create_tables, *};

pub struct SmartOrganizer {
    pub listeners: Arc<Mutex<Vec<ListenerData>>>,
    watched_paths: HashMap<PathBuf, u16>, // keeps track of all the paths that are being watched
    debouncer: Debouncer<RecommendedWatcher, FileIdMap>, // does the actual monitoring of the paths
    rx: Arc<Mutex<Receiver<Vec<DebouncedEvent>>>>, // receiver of all events
    database: SqliteConnection,
    is_running: bool,
}

impl SmartOrganizer {
    pub fn start(&mut self) {
        if !self.is_running {
            println!("started");
            let rx = self.rx.clone();
            let data = self.listeners.clone();
            self.is_running = true;

            std::thread::Builder::new()
                .name(String::from("fs"))
                .spawn(move || loop {
                    for event in rx.lock().unwrap().recv().unwrap() {
                        match event.kind {
                            EventKind::Create(_) => {
                                Self::parse(event.paths[0].clone(), &data.lock().unwrap());
                            }

                            EventKind::Modify(kind) => match kind {
                                ModifyKind::Any => {
                                    Self::parse(event.paths[0].clone(), &data.lock().unwrap());
                                }
                                ModifyKind::Name(RenameMode::To) => {
                                    Self::parse(event.paths[0].clone(), &data.lock().unwrap());
                                }
                                ModifyKind::Name(RenameMode::Both) => {
                                    Self::parse(event.paths[1].clone(), &data.lock().unwrap());
                                }
                                ModifyKind::Name(RenameMode::Any) => {
                                    Self::parse(event.paths[0].clone(), &data.lock().unwrap());
                                }
                                ModifyKind::Data(DataChange::Any) => {
                                    Self::parse(event.paths[0].clone(), &data.lock().unwrap());
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                });
        }
    }

    pub fn database(&mut self) -> &mut SqliteConnection {
        self.database.borrow_mut()
    }

    pub fn watch_path<P: AsRef<Path>>(&mut self, path: P, recursive: bool) {
        let recursive_mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        if self
            .debouncer
            .watcher()
            .watch(path.as_ref(), recursive_mode)
            .is_err()
        {
            // code to report to frontend
            dbg!("Not able to add listener path: {path:?}");
        }
    }

    pub async fn add_listener(&mut self, listener: &ListenerData) -> Result<String, sqlx::Error> {
        // needs working on
        let found = ListenerQuery::fetch_one(self.database(), &listener.id).await;

        match found {
            Ok(_) => {
                for path in &listener.monitors {
                    *self.watched_paths.entry(path.to_owned()).or_default() += 1; // increments how many watchers on path
                    self.watch_path(path, listener.deep);
                }
            }
            Err(e) => {
                ListenerQuery::insert_one(self.database(), listener).await?;
            }
        }

        self.listeners
            .lock()
            .expect("Unable to lock mutex")
            .push(listener.clone());

        Ok(listener.id.clone())
    }

    pub async fn remove_listener(&mut self, listener: &ListenerData) -> Option<ListenerData> {
        for rmpath in listener.monitors.iter() {
            if let Some(attached) = self.watched_paths.get_mut(rmpath) {
                *attached -= 1;

                if *attached == 0 {
                    self.watched_paths.remove(rmpath);
                }
            }
        }

        match ListenerQuery::remove_one(self.database(), &listener.id).await {
            Ok(data) => match data {
                Some(data) => {
                    self.listeners.lock().unwrap().retain(|v| v.id != data.id);
                    Some(data)
                }
                None => None,
            },
            Err(e) => {
                println!("ERROR: {e:?}");
                None
            }
        }
    }

    pub async fn update_listener(&mut self, listener: &ListenerData) -> String {
        ListenerQuery::update_one(self.database(), listener).await;

        let updated = ListenerQuery::fetch_one(self.database(), &listener.id)
            .await
            .expect("`update_listener`: failed to `fetch_one`")
            .unwrap();

        let mut listeners = self.listeners.clone();
        let mut listeners = listeners.lock().unwrap();

        let idx = listeners
            .iter()
            .position(|item| item.id == updated.id)
            .unwrap();

        let (added, removed) = diff(&listeners[idx].monitors, &listener.monitors);

        for path in added {
            *self.watched_paths.entry(path.to_owned()).or_default() += 1; // increments how many watches on this path
            self.watch_path(path, listener.deep);
        }

        for path in removed {
            if let Some(attached) = self.watched_paths.get_mut(&path) {
                *attached -= 1;

                if *attached == 0 {
                    self.watched_paths.remove(&path);
                }
            }
        }

        listeners[idx] = updated;

        listeners[idx].id.clone()
    }
}

/// Associated Methods
impl SmartOrganizer {
    pub async fn new() -> Self {
        let (tx, rx) = channel();
        let debouncer = new_debouncer(Duration::from_millis(500), None, move |res| {
            if let Ok(event) = res {
                tx.send(event).unwrap();
            }
        })
        .expect("Failed to initialize 'DebouncedWatcher'");
        // sqlite://smartorganizer.db?mode=rwc

        // pub const URI: String = format!("sqlite://{}/data.db?mode=rwc", DIRS.data_local_dir().display());
        let uri = format!(
            "sqlite://{}/data.db?mode=rwc",
            DIRS.data_local_dir().display()
        );

        dbg!(&uri);

        let mut database = SqliteConnection::connect(&uri)
            .await
            .expect("Failed to establish connection");

        ListenerQuery::create(database.borrow_mut()).await;
        LogQuery::create(database.borrow_mut()).await;

        Self {
            debouncer,
            rx: Arc::new(Mutex::new(rx)),
            watched_paths: HashMap::new(),
            listeners: Arc::new(Mutex::new(Vec::new())),
            database,
            is_running: false,
        }
    }

    fn parse<F>(path_found: F, listeners: &[ListenerData])
    where
        F: AsRef<Path>,
    {
        let path = path_found.as_ref();

        if !path.exists() {
            return;
        }
        dbg!(listeners);

        for listener in listeners {
            let should_recv = listener.enabled && Self::should_recv_evt(path, listener);
            dbg!(should_recv);
            if should_recv {
                let is_match = Self::apply_rule(path, listener);
                println!("{} is a match: {is_match}", path.display());
                if is_match {
                    FSActions::operate(&listener.id, &listener.title, path, &listener.actions);
                }
            }
        }
    }

    pub fn should_recv_evt<P: AsRef<Path>>(path: P, listener: &ListenerData) -> bool {
        let path = path.as_ref();
        let mut should_recv = false;

        for watched_path in listener.monitors.iter() {
            // if watched_path is a sub path
            if path.starts_with(watched_path) {
                if listener.deep {
                    return true;
                }

                let path_len = path.components().collect::<Vec<_>>().len();
                let watched_path_len = watched_path.components().collect::<Vec<_>>().len();
                let diff = path_len.abs_diff(watched_path_len);

                if (diff == 1) {
                    return true;
                }
            }
        }

        false
    }

    /// Applies all rules to listener to determine if it is a match
    fn apply_rule<P: AsRef<Path>>(path: P, listener: &ListenerData) -> bool {
        use crate::Condition::*;

        let matches = listener
            .rules
            .iter()
            .map(|rule| match rule.condition {
                Includes => Self::includes(&path, &rule.search, &rule.data.text),
                NotIncludes => match rule.search {
                    SearchType::FileExtension | SearchType::FileContent | SearchType::FileName => {
                        path.as_ref().is_file()
                            & !Self::includes(&path, &rule.search, &rule.data.text)
                    }

                    SearchType::FolderName => {
                        path.as_ref().is_dir()
                            & !Self::includes(&path, &rule.search, &rule.data.text)
                    }
                    _ => !Self::includes(&path, &rule.search, &rule.data.text),
                },
                ExactMatch => Self::exact_match(&path, &rule.search, &rule.data.text),
                IsNot => match rule.search {
                    SearchType::FileExtension | SearchType::FileContent | SearchType::FileName => {
                        path.as_ref().is_file()
                            & !Self::exact_match(&path, &rule.search, &rule.data.text)
                    }

                    SearchType::FolderName => {
                        path.as_ref().is_dir()
                            & !Self::exact_match(&path, &rule.search, &rule.data.text)
                    }
                    _ => !Self::exact_match(&path, &rule.search, &rule.data.text),
                },
                Greater => Self::greater(&path, &rule.search, &rule.data.size),
                Less => {
                    path.as_ref().is_file() & !Self::greater(&path, &rule.search, &rule.data.size)
                }

                IsEqual => Self::is_equal(&path, &rule.search, &rule.data.size),
                _ => false,
            })
            .collect::<Vec<_>>();

        Self::match_and_or(&listener.selection, &matches)
    }

    fn match_and_or(selection: &Selection, matches: &[bool]) -> bool {
        match selection {
            Selection::Any => matches.iter().skip(1).fold(matches[0], |a, b| a | b),
            Selection::All => matches.iter().skip(1).fold(matches[0], |a, b| a & b),
        }
    }

    fn is_equal<P: AsRef<Path>>(path: P, search_type: &SearchType, size: &(u64, UnitOpts)) -> bool {
        let path = path.as_ref();

        if path.is_file() {
            if let SearchType::FileSize = search_type {
                return path.metadata().unwrap().len() == size.1.get_size(size.0);
            };
        }
        false
    }

    fn greater<P: AsRef<Path>>(path: P, search_type: &SearchType, size: &(u64, UnitOpts)) -> bool {
        let path = path.as_ref();

        if path.is_file() {
            if let SearchType::FileSize = search_type {
                return path.metadata().unwrap().len() > size.1.get_size(size.0);
            };
        }

        false
    }

    fn exact_match<P: AsRef<Path>>(path: P, search_type: &SearchType, matches: &[String]) -> bool {
        let path = path.as_ref();
        let mut is_match = false;

        match search_type {
            SearchType::FileName => {
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        let filename = filename.to_str().unwrap();
                        is_match = matches.iter().fold(false, |a, b| a | filename.eq(b));
                    }
                }
            }

            SearchType::FileExtension => match path.extension() {
                Some(ext) => {
                    let ext: String = ext.to_str().unwrap().into();
                    is_match = matches.iter().fold(false, |a, b| a | ext.eq(b));
                    println!("ext:{ext}; matches:{matches:?}; is_match:{is_match}");
                }
                None => {
                    // is_match = matches.contains(&"".into());
                    println!("File has no extension");
                }
            },

            SearchType::FileContent => {
                use sliceslice::x86::DynamicAvx2Searcher;
                if let Ok(file_content) = std::fs::read_to_string(path) {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let searcher = unsafe { DynamicAvx2Searcher::new(to_match.as_bytes()) };
                        unsafe { searcher.search_in(file_content.as_bytes()) }
                    });
                }
            }

            SearchType::PathName => {
                if let Ok(abs_path) = path.canonicalize() {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let as_str = abs_path.to_string_lossy();
                        let normalized = as_str.to_lowercase().replace('\\', "/");
                        normalized.eq(&to_match.replace('\\', "/"))
                    });
                }
            }

            SearchType::FolderName => {
                if path.is_dir() {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let foldername = path.file_name().unwrap().to_str().unwrap();
                        foldername.eq(to_match)
                    });
                }
            }

            _ => {}
        }

        is_match
    }

    fn includes<P: AsRef<Path>>(path: P, search_type: &SearchType, matches: &[String]) -> bool {
        let path = path.as_ref();
        let matches = matches.iter().map(|v| v.to_lowercase()).collect::<Vec<_>>();
        let mut is_match = false;

        match search_type {
            SearchType::FileName => {
                if path.is_file() {
                    if let Some(filename) = path.file_name() {
                        let filename = filename.to_str().unwrap();
                        is_match = matches.iter().fold(false, |a, b| a | filename.contains(b));
                    }
                }
            }

            SearchType::FileExtension => match path.extension() {
                Some(ext) => {
                    let ext: String = ext.to_str().unwrap().into();
                    is_match = matches
                        .iter()
                        .fold(false, |a, b| a | ext.to_lowercase().contains(b))
                }
                None => {
                    // is_match = matches.contains(&"".into());
                    println!("File has no extension");
                }
            },

            SearchType::FileContent => {
                use sliceslice::x86::DynamicAvx2Searcher;
                if let Ok(file_content) = std::fs::read_to_string(path) {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let searcher = unsafe { DynamicAvx2Searcher::new(to_match.as_bytes()) };
                        let content = file_content.to_lowercase();
                        unsafe { searcher.search_in(content.as_bytes()) }
                    });
                }
            }

            SearchType::PathName => {
                if let Ok(abs_path) = path.canonicalize() {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let as_str = abs_path.to_string_lossy();
                        let normalized = as_str.to_lowercase().replace('\\', "/");
                        normalized.contains(&to_match.to_lowercase().replace('\\', "/"))
                    });
                }
            }

            SearchType::FolderName => {
                if path.is_dir() {
                    is_match = matches.iter().fold(false, |a, to_match| {
                        let foldername = path.file_name().unwrap().to_str().unwrap();
                        foldername.to_lowercase().contains(&to_match.to_lowercase())
                    });
                }
            }

            _ => {}
        }

        is_match
    }
}

fn diff<T: Eq + std::hash::Hash + Clone>(old: &[T], new: &[T]) -> (Vec<T>, Vec<T>) {
    let old_set: HashSet<_> = old.iter().cloned().collect();
    let new_set: HashSet<_> = new.iter().cloned().collect();

    let added = new_set.difference(&old_set).cloned().collect();
    let removed = old_set.difference(&new_set).cloned().collect();

    (added, removed)
}
