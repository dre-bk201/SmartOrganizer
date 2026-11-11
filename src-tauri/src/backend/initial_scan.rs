use crate::{ListenerData, SmartOrganizer};
use std::{collections::HashSet, path::PathBuf};

pub(super) fn needs_initial_pass(previous: Option<&ListenerData>, next: &ListenerData) -> bool {
    next.enabled
        && previous.is_none_or(|old| {
            !old.enabled
                || old.monitors != next.monitors
                || old.rules != next.rules
                || old.actions != next.actions
                || old.deep != next.deep
                || old.selection != next.selection
        })
}

pub(super) fn existing_paths(listener: &ListenerData) -> Vec<PathBuf> {
    let mut pending = listener.monitors.clone();
    let mut directories = HashSet::new();
    let mut entries = HashSet::new();
    while let Some(directory) = pending.pop() {
        // Do not follow symbolic links (including a monitored root).
        if std::fs::symlink_metadata(&directory).map_or(true, |m| m.file_type().is_symlink()) {
            continue;
        }
        if !directories.insert(directory.clone()) {
            continue;
        }
        let children = match std::fs::read_dir(&directory) {
            Ok(children) => children,
            Err(error) => {
                eprintln!("Could not scan {}: {error}", directory.display());
                continue;
            }
        };
        for entry in children {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    eprintln!("Could not read directory entry: {error}");
                    continue;
                }
            };
            let path = entry.path();
            let kind = match entry.file_type() {
                Ok(kind) => kind,
                Err(_) => continue,
            };
            if kind.is_symlink() || !SmartOrganizer::should_recv_evt(&path, listener) {
                continue;
            }
            entries.insert(path.clone());
            if listener.deep && kind.is_dir() {
                pending.push(path);
            }
        }
    }
    let mut paths: Vec<_> = entries.into_iter().collect();
    // Descendants first: moving a matched folder must not invalidate unprocessed children.
    paths.sort_by(|a, b| {
        b.components()
            .count()
            .cmp(&a.components().count())
            .then_with(|| a.cmp(b))
    });
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, ActionOpts, Condition, Data, Rule, SearchType, UnitOpts};
    use sqlx::{Connection, SqliteConnection};
    fn listener(root: PathBuf, output: PathBuf) -> ListenerData {
        ListenerData {
            id: "initial-scan".into(),
            title: "Documents".into(),
            enabled: true,
            monitors: vec![root],
            rules: vec![Rule {
                search: SearchType::FileExtension,
                condition: Condition::ExactMatch,
                data: Data {
                    text: vec!["txt".into()],
                    size: (0, UnitOpts::B),
                },
            }],
            actions: vec![Action {
                action: ActionOpts::MOVE,
                path: output,
            }],
            ..Default::default()
        }
    }
    #[tokio::test]
    async fn creation_and_enable_process_existing_files_without_rescanning_on_restore() {
        let dir = assert_fs::TempDir::new().unwrap();
        let root = dir.path().join("input");
        let output = dir.path().join("output");
        std::fs::create_dir(&root).unwrap();
        std::fs::create_dir(&output).unwrap();
        let mut app =
            SmartOrganizer::from_database(SqliteConnection::connect(":memory:").await.unwrap())
                .await;
        let mut config = listener(root.clone(), output.clone());
        std::fs::write(root.join("existing.txt"), "existing").unwrap();
        std::fs::write(root.join("leave.pdf"), "leave").unwrap();
        app.add_listener(&config).await.unwrap();
        assert!(output.join("existing.txt").exists());
        assert!(!root.join("existing.txt").exists());
        assert!(root.join("leave.pdf").exists());
        config.enabled = false;
        app.update_listener(&config).await.unwrap();
        std::fs::write(root.join("paused.txt"), "paused").unwrap();
        assert!(root.join("paused.txt").exists());
        config.enabled = true;
        app.update_listener(&config).await.unwrap();
        assert!(output.join("paused.txt").exists());
        std::fs::write(root.join("unchanged.txt"), "unchanged").unwrap();
        app.update_listener(&config).await.unwrap();
        assert!(root.join("unchanged.txt").exists());
        app.listeners.lock().unwrap().clear();
        app.add_listener(&config).await.unwrap();
        assert!(root.join("unchanged.txt").exists());
        config.rules[0].data.text.push("pdf".into());
        app.update_listener(&config).await.unwrap();
        assert!(output.join("leave.pdf").exists());
        assert!(output.join("unchanged.txt").exists());
        app.start();
        std::fs::write(root.join("after-enable.txt"), "new event").unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(6), async {
            while !output.join("after-enable.txt").exists() {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
        .await
        .expect("Live watcher should process files created after the initial pass");
    }
    #[test]
    fn traversal_respects_depth_outputs_and_overlapping_roots() {
        let dir = assert_fs::TempDir::new().unwrap();
        let root = dir.path().join("input");
        let sub = root.join("sub");
        let output = root.join("output");
        std::fs::create_dir_all(&sub).unwrap();
        std::fs::create_dir(&output).unwrap();
        std::fs::write(root.join("top.txt"), "top").unwrap();
        std::fs::write(sub.join("nested.txt"), "nested").unwrap();
        std::fs::write(output.join("excluded.txt"), "excluded").unwrap();
        let mut config = listener(root.clone(), output.clone());
        let paths = existing_paths(&config);
        assert!(paths.contains(&root.join("top.txt")));
        assert!(!paths.contains(&sub.join("nested.txt")));
        assert!(!paths.iter().any(|p| p.starts_with(&output)));
        assert!(!paths.contains(&root));
        config.deep = true;
        config.monitors.push(sub.clone());
        let paths = existing_paths(&config);
        assert_eq!(
            paths
                .iter()
                .filter(|p| **p == sub.join("nested.txt"))
                .count(),
            1
        );
        assert!(!paths.iter().any(|p| p.starts_with(&output)));
        assert!(!needs_initial_pass(Some(&config), &config));
        let mut renamed = config.clone();
        renamed.title = "New name".into();
        assert!(!needs_initial_pass(Some(&config), &renamed));
        renamed.enabled = false;
        assert!(!needs_initial_pass(Some(&config), &renamed));
        assert!(needs_initial_pass(Some(&renamed), &config));
    }
}
