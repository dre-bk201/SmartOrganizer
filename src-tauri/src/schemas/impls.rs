use std::path::PathBuf;
use crate::Log;
use super::*;

impl FromRow<'_, SqliteRow> for ListenerRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            created: row.try_get("created")?,
            deep: row.try_get("deep")?,
            title: row.try_get("title")?,
            enabled: row.try_get("enabled")?,
            selection: row.try_get("selection")?,
        })
    }
}

impl FromRow<'_, SqliteRow> for MonitorRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            monitor_path: row.try_get("monitor_path")?,
        })
    }
}

impl FromRow<'_, SqliteRow> for ActionRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            action: row.try_get("action")?,
            action_path: row.try_get("action_path")?,
        })
    }
}

impl FromRow<'_, SqliteRow> for RuleRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            search: row.try_get("search")?,
            condition: row.try_get("condition")?,
            rule_text: row.try_get("rule_text")?,
            rule_size: row.try_get("rule_size")?,
            rule_sizeopt: row.try_get("rule_sizeopt")?,
        })
    }
}

impl FromRow<'_, SqliteRow> for LogRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            action: row.try_get("action")?,
            level: row.try_get("level")?,
            message: row.try_get("message")?,
            path: row.try_get("path")?,
            parent_id: row.try_get("parent_id")?,
            timestamp: row.try_get("timestamp")?,
            destination: row.try_get("destination")?,
        })
    }
}

impl Into<Log> for LogRow {
    fn into(self) -> Log {
        let LogRow {
            id,
            name,
            level,
            message,
            path,
            timestamp,
            action,
            parent_id,
            destination,
        } = self;

        Log {
            id,
            name,
            level: level.parse().unwrap(),
            message,
            path: PathBuf::from(path),
            timestamp,
            action: action.parse().unwrap(),
            parent_id,
            destination: PathBuf::from(destination),
        }
    }
}
