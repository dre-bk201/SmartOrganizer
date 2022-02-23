use serde::{Deserialize, Serialize};
use tauri::{Runtime, Window};

use super::operations::Organizer;

#[derive(Debug, Default)]

pub struct SmartOrganizer {
    data: Vec<ListenerData>,
}

impl SmartOrganizer {
    pub fn get_data(&self) -> &Vec<ListenerData> {
        &self.data
    }

    pub fn from(data: Vec<ListenerData>) -> Self {
        Self { data }
    }

    pub fn organize<R: Runtime>(&mut self, window: &Window<R>) {
        for data in self.data.iter() {
            for path_str in data.paths.iter() {
                if data.enabled {
                    Organizer::from(window).sort::<R>(
                        &data.id,
                        &data.deep,
                        &path_str,
                        &data.rules,
                        &data.actions,
                        &data.selection,
                    );
                }
            }
            // Self::parse(&data);
        }
        // Emit a signal back to frontend
    }

    pub fn delete(&mut self, item: ListenerData) {
        let idx = self.data.iter().position(|val| *val.id == item.id).unwrap();
        self.data.remove(idx);
    }

    pub fn push(&mut self, listener: ListenerData) {
        self.data.push(listener);
    }

    pub fn replace(&mut self, listener: ListenerData) {
        self.data.iter_mut().for_each(|item| {
            if item.id == listener.id {
                *item = listener.clone();
            }
        })
    }
}

impl SmartOrganizer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct Action(pub String, pub String);

impl Action {
    pub fn from<T: Into<String>>(x: T, y: T) -> Self {
        Self(x.into(), y.into())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListenerData {
    id: String,
    deep: bool,
    title: String,
    enabled: bool,
    selection: String,
    paths: Vec<String>,
    rules: Vec<Rule>,
    actions: Vec<Action>,
    logs: Vec<Log>,
}

impl ListenerData {
    pub fn from(
        id: String,
        deep: bool,
        title: String,
        enabled: bool,
        paths: Vec<String>,
        selection: String,
        rules: Vec<Rule>,
        actions: Vec<Action>,
        logs: Vec<Log>,
    ) -> Self {
        Self {
            id,
            deep,
            title,
            enabled,
            paths,
            selection,
            actions,
            rules,
            logs,
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
pub enum SearchType {
    FileName(String),
    FolderName(String),
    FileExtension(String),
    FileContent(String),
    PathName(String),
}
#[derive(Debug, Deserialize, Clone)]
pub enum Condition {
    Includes(String),
    NotIncludes(String),
    ExactMatch(String),
    IsNot(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub search_type: SearchType,
    pub condition: Condition,
    pub text: String,
}

// impl Rule {
//     pub fn from<T: Into<String>>(search_type: T, condition: T, text: T) -> Self {
//         Self {
//             search_type: SearchType(),
//             condition: condition.into(),
//             text: text.into(),
//         }
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    id: String,
    path: String,
    destination: String,
    action: String,
    timestamp: String,
}

impl Log {
    pub fn from(
        id: String,
        path: String,
        destination: String,
        action: String,
        timestamp: String,
    ) -> Self {
        Self {
            id,
            path,
            destination,
            action,
            timestamp,
        }
    }
}
