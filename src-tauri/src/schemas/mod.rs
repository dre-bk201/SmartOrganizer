use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Debug, Clone)]
pub struct ListenerRow {
    pub id: String,
    pub created: String,
    pub deep: bool,
    pub title: String,
    pub enabled: bool,
    pub selection: String,
}

#[derive(Debug, Clone)]
pub struct LogRow {
    pub id: String,
    pub name: String,
    pub level: String,
    pub message: String,
    pub path: String,
    pub timestamp: String,
    pub action: String,
    pub parent_id: String,
    pub destination: String,
}

#[derive(Debug, Clone)]
pub struct MonitorRow {
    pub monitor_path: String,
}

#[derive(Debug, Clone)]
pub struct RuleRow {
    pub search: String,
    pub condition: String,
    pub rule_text: String,
    pub rule_size: i64,
    pub rule_sizeopt: String,
}

#[derive(Debug, Clone)]
pub struct ActionRow {
    pub action: String,
    pub action_path: String,
}

mod impls;
