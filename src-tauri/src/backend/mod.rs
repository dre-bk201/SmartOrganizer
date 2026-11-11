#![allow(unused)]
mod actions;
mod initial_scan;
pub mod logger;
mod rules;

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
    time::{Duration, Instant},
};

use crate::{database::create_tables, *};

pub struct SmartOrganizer {
    pub listeners: Arc<Mutex<Vec<ListenerData>>>,
    watched_paths: HashMap<PathBuf, bool>, // keeps track of all the paths that are being watched
    debouncer: Debouncer<RecommendedWatcher, FileIdMap>, // does the actual monitoring of the paths
    rx: Arc<Mutex<Receiver<Vec<DebouncedEvent>>>>, // receiver of all events
    database: SqliteConnection,
    is_running: bool,
    activated_at: Arc<Mutex<HashMap<String, Instant>>>,
}

impl SmartOrganizer {
    pub fn start(&mut self) {
        if !self.is_running {
            println!("started");
            let rx = self.rx.clone();
            let data = self.listeners.clone();
            let activated_at = self.activated_at.clone();
            self.is_running = true;

            std::thread::Builder::new()
                .name(String::from("fs"))
                .spawn(move || {
                    while let Ok(events) = rx.lock().unwrap().recv() {
                        for event in events {
                            let relevant = matches!(
                                event.kind,
                                EventKind::Create(_)
                                    | EventKind::Modify(
                                        ModifyKind::Any | ModifyKind::Data(_) | ModifyKind::Name(_)
                                    )
                            );
                            if !relevant {
                                continue;
                            }
                            let path = if matches!(
                                event.kind,
                                EventKind::Modify(ModifyKind::Name(RenameMode::Both))
                            ) {
                                event.paths.get(1)
                            } else {
                                event.paths.first()
                            };
                            if let Some(path) = path {
                                // Do not hold the state mutex during slow filesystem operations.
                                let snapshot = data.lock().unwrap().clone();
                                let eligible: Vec<_> = snapshot
                                    .into_iter()
                                    .filter(|listener| {
                                        activated_at
                                            .lock()
                                            .unwrap()
                                            .get(&listener.id)
                                            .is_none_or(|started| event.time >= *started)
                                    })
                                    .collect();
                                Self::parse(path, &eligible);
                            }
                        }
                    }
                })
                .expect("Unable to start filesystem worker");
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

    fn sync_watches(&mut self) {
        let mut desired = HashMap::new();
        for listener in self.listeners.lock().unwrap().iter().filter(|l| l.enabled) {
            for path in &listener.monitors {
                let recursive = desired.entry(path.clone()).or_insert(false);
                *recursive |= listener.deep;
            }
        }
        let obsolete: Vec<_> = self
            .watched_paths
            .iter()
            .filter(|(path, recursive)| desired.get(*path) != Some(*recursive))
            .map(|(path, _)| path.clone())
            .collect();
        for path in obsolete {
            match self.debouncer.watcher().unwatch(&path) {
                Ok(()) => {
                    self.watched_paths.remove(&path);
                }
                Err(error) => eprintln!("Could not stop watching {}: {error}", path.display()),
            }
        }
        for (path, recursive) in desired {
            if self.watched_paths.get(&path) == Some(&recursive) {
                continue;
            }
            let mode = if recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            match self.debouncer.watcher().watch(&path, mode) {
                Ok(()) => {
                    self.watched_paths.insert(path, recursive);
                }
                Err(error) => eprintln!("Could not watch {}: {error}", path.display()),
            }
        }
    }

    pub async fn add_listener(&mut self, listener: &ListenerData) -> Result<String, sqlx::Error> {
        let created = match ListenerQuery::fetch_one(self.database(), &listener.id).await {
            Ok(Some(_)) => false,
            Ok(None) | Err(sqlx::Error::RowNotFound) => {
                ListenerQuery::insert_one(self.database(), listener).await?;
                true
            }
            Err(error) => return Err(error),
        };
        if self
            .listeners
            .lock()
            .unwrap()
            .iter()
            .any(|item| item.id == listener.id)
        {
            return Ok(listener.id.clone());
        }
        // Restoring persisted listeners on startup must not repeat the initial pass.
        if created && listener.enabled {
            Self::initial_pass(listener.clone()).await;
            self.activated_at
                .lock()
                .unwrap()
                .insert(listener.id.clone(), Instant::now());
        }
        self.listeners.lock().unwrap().push(listener.clone());
        self.sync_watches();
        Ok(listener.id.clone())
    }

    pub async fn remove_listener(
        &mut self,
        listener: &ListenerData,
    ) -> Result<Option<ListenerData>, sqlx::Error> {
        let removed = ListenerQuery::remove_one(self.database(), &listener.id).await?;
        if removed.is_some() {
            self.listeners
                .lock()
                .unwrap()
                .retain(|item| item.id != listener.id);
            self.activated_at.lock().unwrap().remove(&listener.id);
            self.sync_watches();
        }
        Ok(removed)
    }

    pub async fn update_listener(
        &mut self,
        listener: &ListenerData,
    ) -> Result<String, sqlx::Error> {
        let previous = self
            .listeners
            .lock()
            .unwrap()
            .iter()
            .find(|item| item.id == listener.id)
            .cloned()
            .ok_or(sqlx::Error::RowNotFound)?;
        ListenerQuery::update_one(self.database(), listener).await?;
        let rescan = initial_scan::needs_initial_pass(Some(&previous), listener);
        if rescan {
            // Pause this listener while scanning; other listeners keep running.
            if let Some(item) = self
                .listeners
                .lock()
                .unwrap()
                .iter_mut()
                .find(|item| item.id == listener.id)
            {
                item.enabled = false;
            }
            self.sync_watches();
            Self::initial_pass(listener.clone()).await;
            self.activated_at
                .lock()
                .unwrap()
                .insert(listener.id.clone(), Instant::now());
        }
        if let Some(item) = self
            .listeners
            .lock()
            .unwrap()
            .iter_mut()
            .find(|item| item.id == listener.id)
        {
            *item = listener.clone();
        }
        self.sync_watches();
        Ok(listener.id.clone())
    }

    async fn initial_pass(listener: ListenerData) {
        if let Err(error) = tokio::task::spawn_blocking(move || {
            // Collect before acting so copies cannot extend this same traversal.
            for path in initial_scan::existing_paths(&listener) {
                Self::parse(path, std::slice::from_ref(&listener));
            }
        })
        .await
        {
            eprintln!("Initial folder scan failed: {error}");
        }
    }
}

/// Associated Methods
impl SmartOrganizer {
    pub async fn new() -> Self {
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

        Self::from_database(database).await
    }

    pub(crate) async fn from_database(mut database: SqliteConnection) -> Self {
        let (tx, rx) = channel();
        let debouncer = new_debouncer(Duration::from_millis(500), None, move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })
        .expect("Failed to initialize 'DebouncedWatcher'");
        ListenerQuery::create(database.borrow_mut())
            .await
            .expect("Cannot initialize listener tables");
        LogQuery::create(database.borrow_mut())
            .await
            .expect("Cannot initialize log table");

        Self {
            debouncer,
            rx: Arc::new(Mutex::new(rx)),
            watched_paths: HashMap::new(),
            listeners: Arc::new(Mutex::new(Vec::new())),
            database,
            is_running: false,
            activated_at: Arc::new(Mutex::new(HashMap::new())),
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

        for listener in listeners {
            let should_recv = listener.enabled && Self::should_recv_evt(path, listener);

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
        // Ignore this listener's output tree to prevent recursive copy/move feedback.
        if listener.actions.iter().any(|action| {
            matches!(action.action, ActionOpts::COPY | ActionOpts::MOVE)
                && !action.path.as_os_str().is_empty()
                && path.starts_with(&action.path)
        }) {
            return false;
        }
        listener.monitors.iter().any(|watched| {
            path != watched
                && path.starts_with(watched)
                && (listener.deep || path.parent() == Some(watched.as_path()))
        })
    }

    fn apply_rule<P: AsRef<Path>>(path: P, listener: &ListenerData) -> bool {
        rules::matches_listener(path.as_ref(), listener)
    }
}

fn diff<T: Eq + std::hash::Hash + Clone>(old: &[T], new: &[T]) -> (Vec<T>, Vec<T>) {
    let old_set: HashSet<_> = old.iter().cloned().collect();
    let new_set: HashSet<_> = new.iter().cloned().collect();

    let added = new_set.difference(&old_set).cloned().collect();
    let removed = old_set.difference(&new_set).cloned().collect();

    (added, removed)
}
