use super::*;

#[test]
fn insert_test() {}

impl LogQuery {
    pub async fn create(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS LOG (
            id TEXT,
            name TEXT,
            level CHAR(10),
            message TEXT,
            path TEXT,
            timestamp TEXT,
            action TEXT,
            parent_id TEXT,
            destination TEXT
        )",
        )
        .execute(conn)
        .await
        .expect("Failed to create table `Log`");

        Ok(())
    }

    pub async fn drop(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        sqlx::query("DROP TABLE IF EXISTS ListenerData")
            .execute(conn.borrow_mut())
            .await
            .expect("Cannot drop table `ListenerData`");
        sqlx::query("DROP TABLE IF EXISTS Log")
            .execute(conn)
            .await
            .expect("Cannot drop table `Log`");
        Ok(())
    }

    pub async fn fetch_one(conn: &mut SqliteConnection, id: &String) -> Option<Log> {
        let row = sqlx::query_as::<_, LogRow>("SELECT * FROM LOG WHERE id=?")
            .bind(id)
            .fetch_one(conn)
            .await
            .ok()?;

        Some(row.into())
    }

    pub async fn fetch_all(conn: &mut SqliteConnection) -> Result<Option<Vec<Log>>, sqlx::Error> {
        let rowres = match sqlx::query_as::<_, LogRow>("SELECT * FROM LOG")
            .fetch_all(conn)
            .await
        {
            Ok(v) => Some(v.into_iter().map(|a| a.into()).collect::<Vec<Log>>()),
            Err(_) => None,
        };

        Ok(rowres)
    }

    pub async fn insert_one(conn: &mut SqliteConnection, log: &Log) -> bool {
        sqlx::query(
            "INSERT INTO LOG VALUES 
                    (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&log.id)
        .bind(&log.name)
        .bind(log.level.to_string())
        .bind(&log.message)
        .bind(log.path.to_str().unwrap())
        .bind(&log.timestamp)
        .bind(&log.action.to_string())
        .bind(&log.parent_id)
        .bind(log.destination.to_str().unwrap())
        .execute(conn)
        .await
        .is_ok()
    }
}

impl ListenerQuery {
    pub async fn create(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
        CREATE TABLE IF NOT EXISTS ListenerData (
            id CHAR(50) PRIMARY KEY,
            created TEXT,
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
                ON DELETE CASCADE
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
                ON DELETE CASCADE
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
                ON DELETE CASCADE
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
        )
        ",
        )
        .execute(conn.borrow_mut())
        .await?;

        Ok(())
    }

    pub async fn remove_one(
        conn: &mut SqliteConnection,
        id: &String,
    ) -> Result<Option<ListenerData>, sqlx::Error> {
        let removed = Self::fetch_one(conn.borrow_mut(), id).await?;

        match sqlx::query(&format!("DELETE FROM {} WHERE id=?", "ListenerData"))
            .bind(id)
            .execute(conn.borrow_mut())
            .await
        {
            Ok(_) => println!("table `ListenerData` successfully cleared"),
            Err(e) => println!("failed to clear table: {e}"),
        };

        println!("removed from db: {removed:?}");

        Ok(removed)
    }

    pub async fn update_one(
        conn: &mut SqliteConnection,
        data: &ListenerData,
    ) -> Result<(), sqlx::Error> {
        Self::remove_one(conn.borrow_mut(), &data.id).await?;
        Self::insert_one(conn.borrow_mut(), data).await?.unwrap();
        Ok(())
    }

    pub async fn insert_one(
        conn: &mut SqliteConnection,
        l: &ListenerData,
    ) -> Result<Option<ListenerData>, sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO ListenerData (id, created, deep, title, enabled, selection) 
            VALUES (?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&l.id)
        .bind(&l.created)
        .bind(l.deep)
        .bind(&l.title)
        .bind(l.enabled)
        .bind(l.selection.to_string())
        .execute(conn.borrow_mut())
        .await?;

        for monitor in l.monitors.iter() {
            sqlx::query(
                "
            INSERT INTO Monitor (id, monitor_path)
            VALUES (?, ?)
        ",
            )
            .bind(&l.id)
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
            .bind(&l.id)
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
            .bind(&l.id)
            .bind(action.to_string())
            .bind(path.to_str().unwrap())
            .execute(conn.borrow_mut())
            .await?;
        }

        let found = Self::fetch_one(conn.borrow_mut(), &l.id)
            .await
            .expect("Unable to fetch");

        dbg!(&found);

        Ok(found)
    }

    pub async fn fetch_one(
        conn: &mut SqliteConnection,
        id: &String,
    ) -> Result<Option<ListenerData>, sqlx::Error> {
        let lrow = sqlx::query_as::<_, ListenerRow>(
            "
        SELECT * FROM ListenerData
        WHERE id=?
    ",
        )
        .bind(id)
        .fetch_one(conn.borrow_mut())
        .await?;

        let mrow = sqlx::query_as::<_, MonitorRow>(
            "
        SELECT * FROM Monitor
        WHERE id=?
    ",
        )
        .bind(id)
        .fetch_all(conn.borrow_mut())
        .await?;

        let rrow = sqlx::query_as::<_, RuleRow>(
            "
        SELECT * FROM Rule
        WHERE id=?
    ",
        )
        .bind(id)
        .fetch_all(conn.borrow_mut())
        .await?;

        let arow = sqlx::query_as::<_, ActionRow>(
            "
        SELECT * FROM Action
        WHERE id=?
    ",
        )
        .bind(id)
        .fetch_all(conn.borrow_mut())
        .await?;

        Ok(Some(ListenerData {
            id: lrow.id,
            created: lrow.created,
            title: lrow.title,
            enabled: lrow.enabled,
            deep: lrow.deep,
            selection: lrow.selection.parse().unwrap(),
            monitors: mrow
                .into_iter()
                .map(|s| PathBuf::from(s.monitor_path))
                .collect(),
            rules: rrow
                .into_iter()
                .map(|s| Rule {
                    search: s.search.parse().unwrap(),
                    condition: s.condition.parse().unwrap(),
                    data: crate::Data {
                        text: s
                            .rule_text
                            .split_whitespace()
                            .map(|v| v.to_owned())
                            .collect::<Vec<String>>(),
                        size: (s.rule_size as u64, s.rule_sizeopt.parse().unwrap()),
                    },
                })
                .collect(),
            actions: arow
                .into_iter()
                .map(|a| Action {
                    action: a.action.parse().unwrap(),
                    path: PathBuf::from(a.action_path),
                })
                .collect(),
        }))
    }

    pub async fn fetch_all(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Option<ListenerData>>, sqlx::Error> {
        let ls = sqlx::query_as::<_, ListenerRow>("SELECT * FROM ListenerData")
            .fetch_all(conn.borrow_mut())
            .await?;

        let mut res = Vec::with_capacity(ls.len());

        for l in ls {
            if let Ok(data) = Self::fetch_one(conn.borrow_mut(), &l.id).await {
                res.push(data);
            }
        }

        Ok(res)
    }
}
