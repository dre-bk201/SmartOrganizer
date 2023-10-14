use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use std::path::PathBuf;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub search: SearchType,
    pub condition: Condition,
    pub data: Data,
}

#[derive(
    Ord, PartialOrd, PartialEq, Eq, EnumString, Display, Debug, Clone, Serialize, Deserialize,
)]
pub enum ActionOpts {
    MOVE,
    COPY,
    DELETE,
    UNLINK,
    RENAME,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action: ActionOpts,
    pub path: PathBuf,
}

#[derive(
    Ord,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    EnumString,
    Display,
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum Selection {
    #[default]
    Any, // OR Boolean Function
    All, // AND Boolean Function
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub text: Vec<String>,
    pub size: (u64, UnitOpts),
}

#[derive(
    Ord, PartialOrd, PartialEq, Eq, EnumString, Display, Debug, Serialize, Deserialize, Clone,
)]
pub enum UnitOpts {
    B,
    Kb,
    Mb,
    Gb,
    Tb,
}

impl UnitOpts {
    pub fn get_size(&self, size: u64) -> u64 {
        match self {
            Self::B => size,
            Self::Kb => size * 1024,
            Self::Mb => size * 1048576,
            Self::Gb => size * 1073741824,
            Self::Tb => size * 1099511627800,
        }
    }
}

#[derive(
    Ord, PartialOrd, PartialEq, Eq, EnumString, Display, Debug, Serialize, Deserialize, Clone,
)]
pub enum SearchType {
    FolderName,
    FileName,
    FileExtension,
    FileContent,
    FileSize,
    PathName,
}

#[derive(
    Ord, PartialOrd, PartialEq, Eq, EnumString, Display, Debug, Serialize, Deserialize, Clone,
)]
pub enum Condition {
    Includes,
    NotIncludes,
    ExactMatch,
    IsNot,
    Greater,
    IsEqual,
    Less,
}

#[derive(EnumString, Display, Debug, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    WARN,
    INFO,
    SUCCESS,
    ERROR,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub id: String,
    pub name: String,
    pub level: LogLevel,
    pub message: String,
    pub path: PathBuf,
    pub timestamp: String,
    pub action: ActionOpts,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    pub destination: PathBuf,
}

#[derive(Ord, Default, PartialOrd, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenerData {
    pub id: String,
    pub created: String,
    pub deep: bool,
    pub title: String,
    pub enabled: bool,
    pub selection: Selection,
    pub monitors: Vec<PathBuf>,
    pub rules: Vec<Rule>,
    pub actions: Vec<Action>,
}
