use std::{borrow::BorrowMut, path::PathBuf};

use sqlx::{Connection, SqliteConnection};

use super::*;

pub async fn drop_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query("DROP TABLE IF EXISTS Action")
        .execute(conn.borrow_mut())
        .await?;
    sqlx::query("DROP TABLE IF EXISTS Monitor")
        .execute(conn.borrow_mut())
        .await?;
    sqlx::query("DROP TABLE IF EXISTS Rule")
        .execute(conn.borrow_mut())
        .await?;
    sqlx::query("DROP TABLE IF EXISTS ListenerData")
        .execute(conn.borrow_mut())
        .await?;
    Ok(())
}

pub async fn refresh_db(conn: &mut SqliteConnection) -> Result<Vec<ListenerData>, sqlx::Error> {
    drop_tables(conn).await?;
    create_dbs(conn).await?;
    insert_into_db(conn, listeners()).await?;
    DB::fetch_all(conn).await
}

#[tokio::test]
async fn refresh() {
    let mut conn = SqliteConnection::connect(URI).await.unwrap();
    refresh_db(&mut conn).await;
}

pub async fn insert_into_db(
    conn: &mut SqliteConnection,
    listeners: Vec<ListenerData>,
) -> Result<(), sqlx::Error> {
    // let listeners = listeners();

    for l in listeners.iter() {
        sqlx::query(
            "
            INSERT INTO ListenerData (id, deep, title, enabled, selection) 
            VALUES (?, ?, ?, ?, ?)
        ",
        )
        .bind(l.id.clone())
        .bind(l.deep)
        .bind(l.title.clone())
        .bind(l.enabled)
        .bind(l.selection.to_string())
        .execute(conn.borrow_mut())
        .await?
        .last_insert_rowid();

        for monitor in l.monitors.iter() {
            sqlx::query(
                "
            INSERT INTO Monitor (id, monitor_path)
            VALUES (?, ?)
        ",
            )
            .bind(l.id.clone())
            .bind(monitor.to_str().unwrap())
            .execute(conn.borrow_mut())
            .await?;
        }

        for rule in l.rules.iter() {
            sqlx::query(
                "
                INSERT INTO Rule (id, search, condition, rule_text, rule_size, rule_sizeopt)
                VALUES(?, ?, ?, ?, ?, ?)
            ",
            )
            .bind(l.id.clone())
            .bind(rule.search.to_string())
            .bind(rule.condition.to_string())
            .bind(rule.data.text.join(" "))
            .bind(rule.data.size.0 as i64)
            .bind(rule.data.size.1.to_string())
            .execute(conn.borrow_mut())
            .await?;
        }

        for Action { action, path } in l.actions.iter() {
            sqlx::query(
                "
                INSERT INTO Action (id, action, action_path)
                VALUES (?, ?, ?)
            ",
            )
            .bind(l.id.clone())
            .bind(action.to_string())
            .bind(path.clone().to_str().unwrap())
            .execute(conn.borrow_mut())
            .await?;
        }
    }

    Ok(())
}

pub async fn create_dbs(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
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

    Ok(())
}

pub fn listeners() -> Vec<ListenerData> {
    // Create some dummy paths
    let path1 = std::path::PathBuf::from("/path/to/dir1");
    let path2 = std::path::PathBuf::from("/path/to/dir2");

    // Create a dummy rule
    let rule = Rule {
        search: SearchType::FileName,
        condition: Condition::Includes,
        data: Data {
            text: vec!["example".to_string()],
            size: (1024, UnitOpts::Kb),
        },
    };

    // Create dummy actions
    let action1 = Action {
        action: ActionOpts::MOVE,
        path: PathBuf::from("/path/to/destination1"),
    };

    let action2 = Action {
        action: ActionOpts::DELETE,
        path: PathBuf::from(""),
    };
    let listeners: Vec<ListenerData> = vec![
        ListenerData {
            id: "id_1".to_string(),
            deep: true,
            title: "Dummy Listener 1".to_string(),
            enabled: true,
            selection: Selection::Any,
            monitors: vec![path1.clone(), path2.clone()],
            rules: vec![rule.clone()],
            actions: vec![action1.clone(), action2.clone()],
        },
        ListenerData {
            id: "id_2".to_string(),
            deep: false,
            title: "Dummy Listener 2".to_string(),
            enabled: false,
            selection: Selection::All,
            monitors: vec![path1.clone()],
            rules: vec![rule.clone()],
            actions: vec![action1.clone()],
        },
        ListenerData {
            id: "id_3".to_string(),
            deep: true,
            title: "Dummy Listener 3".to_string(),
            enabled: true,
            selection: Selection::Any,
            monitors: vec![path2.clone()],
            rules: vec![rule.clone()],
            actions: vec![action2.clone()],
        },
        ListenerData {
            id: "id_4".to_string(),
            deep: false,
            title: "Dummy Listener 4".to_string(),
            enabled: true,
            selection: Selection::All,
            monitors: vec![path2],
            rules: vec![rule.clone()],
            actions: vec![action2.clone()],
        },
        ListenerData {
            id: "id_5".to_string(),
            deep: true,
            title: "Dummy Listener 5".to_string(),
            enabled: false,
            selection: Selection::Any,
            monitors: vec![path1.clone()],
            rules: vec![rule.clone()],
            actions: vec![action1.clone()],
        },
    ];

    listeners
}
