use std::{borrow::BorrowMut, path::PathBuf};

use sqlx::SqliteConnection;

use crate::{Action, ActionRow, ListenerData, ListenerRow, Log, LogRow, MonitorRow, Rule, RuleRow};

pub struct LogQuery;
pub struct ListenerQuery;

pub async fn create_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS ListenerData (
            id CHAR(50) PRIMARY KEY,
            deep BOOLEAN NOT NULL,
            title TEXT NOT NULL,
            enabled BOOLEAN NOT NULL,
            selection CHAR(50) NOT NULL
        )",
    )
    .execute(conn.borrow_mut())
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS Monitor (
            id CHAR(50),
            monitor_path TEXT,
            FOREIGN KEY(id)
                REFERENCES ListenerData(id)
        )
    ",
    )
    .execute(conn.borrow_mut())
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS Rule (
            id CHAR(50),
            search CHAR(20),
            condition CHAR(20),
            rule_text TEXT,
            rule_size INTEGER,
            rule_sizeopt CHAR(2),
            FOREIGN KEY(id)
                REFERENCES ListenerData(id)
        )
    ",
    )
    .execute(conn.borrow_mut())
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS Action (
            id CHAR(50),
            action CHAR(20),
            action_path TEXT,
            FOREIGN KEY(id)
                REFERENCES ListenerData(id)
        )
    ",
    )
    .execute(conn.borrow_mut())
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS Log (
            id TEXT,
            name TEXT,
            level CHAR(10),
            message TEXT,
            path TEXT,
            timestamp TEXT,
            action TEXT,
            parent_id TEXT,
            destination TEXT,
            FOREIGN KEY(parent_id)
                REFERENCES ListenerData(id)
        )
        ",
    )
    .execute(conn.borrow_mut())
    .await?;

    Ok(())
}

mod impls;
