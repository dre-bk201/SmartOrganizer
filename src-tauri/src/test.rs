
//
// #[async_std::test]
// async fn test_update_listener() -> Result<(), anyhow::Error> {
//     let mut o = SmartOrganizer::new().await;
//     let mut l1 = listeners().first().unwrap().clone();
//
//     l1.title = String::from("This is the new title");
//     l1.monitors.push(PathBuf::from("C:/asd/asd"));
//
//     drop_tables(o.conn()).await?;
//     create_dbs(o.conn()).await?;
//
//     for l in listeners() {
//         o.add_listener(&l).await;
//     }
//
//     o.update_listener(&l1).await;
//
//     let res = DB::fetch(o.conn(), &l1.id).await?;
//     assert!(res == Some(l1));
//
//     Ok(())
// }
//
// #[async_std::test]
// async fn test_remove_listener() -> Result<(), anyhow::Error> {
//     let mut o = SmartOrganizer::new().await;
//
//     drop_tables(o.conn()).await?;
//     create_dbs(o.conn()).await?;
//
//     for l in listeners() {
//         o.add_listener(&l).await;
//     }
//
//     o.remove_listener(listeners().get(0).unwrap()).await;
//     let mut all = DB::fetch_all(o.conn()).await?;
//
//     assert_eq!(all, o.listeners.lock().unwrap().as_slice());
//     Ok(())
// }

#[cfg(test)]
mod organizer_tests {
    use assert_fs::{fixture::ChildPath, prelude::PathChild, TempDir};
    use std::{path::Path, time::Duration};

    use crate::{
        drop_tables, listeners, Action, ActionOpts, ListenerData, Rule, SmartOrganizer,
        UnitOpts, DB, schemas::create_dbs,
    };

    const TIMEOUT: u64 = 2000;

    fn assert_exists(base: &Path, files: &[&str]) {
        for file in files {
            assert!(base.join(file).exists());
        }
    }

    #[tokio::test]
    async fn test_add_listener() -> Result<(), anyhow::Error> {
        // let l1 = listeners().pop().unwrap();
        let l2 = listeners().pop().unwrap();
        let l1 = ListenerData {
            id: String::from("ZIertYt-equRh5DbZt-6e"),
            deep: false,
            title: String::from(""),
            enabled: false,
            selection: crate::Selection::Any,
            monitors: vec![],
            rules: vec![],
            actions: vec![],
        };

        let mut organizer = SmartOrganizer::new().await;

        drop_tables(organizer.conn()).await?;
        create_dbs(organizer.conn()).await?;

        organizer.add_listener(&l1).await;
        let res = DB::fetch(organizer.conn(), &l1.id).await?;
        assert!(Some(l1) == res);

        organizer.add_listener(&l2).await;
        let res = DB::fetch(organizer.conn(), &l2.id).await?;
        assert!(Some(l2) == res);

        Ok(())
    }

    fn create_files(base: &Path, files: &[&str]) {
        for filename in files {
            if filename.ends_with('/') {
                std::fs::create_dir(base.join(filename)).unwrap();
            } else {
                std::fs::File::create(base.join(filename)).unwrap();
            }
        }

        std::thread::sleep(Duration::from_millis(TIMEOUT));
    }

    fn setup_env<P: AsRef<Path>>(persist: bool, path: P) -> (TempDir, ChildPath) {
        let tempdir = TempDir::new().unwrap().into_persistent_if(persist);
        let destination = tempdir.child(path.as_ref());
        std::fs::create_dir_all(&destination).unwrap();
        (tempdir, destination)
    }

    #[async_std::test]
    async fn filename_includes() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "Filename Includes".into(),
                selection: crate::Selection::All,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![
                    Rule {
                        search: crate::SearchType::FileName,
                        condition: crate::Condition::Includes,
                        data: crate::Data {
                            text: vec!["file".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                    Rule {
                        search: crate::SearchType::FileName,
                        condition: crate::Condition::Includes,
                        data: crate::Data {
                            text: vec!["poke".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                ],
                actions: vec![Action {
                    action: ActionOpts::MOVE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &["main.rs", "main.py", "blankfile", "pokefile.rs", "trash/"],
        );

        assert_exists(&tempdir, &["trash/", "blankfile", "main.rs", "main.py"]);
        assert_exists(&destination, &["pokefile.rs"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn filename_not_includes() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "Filename NotIncludes".into(),
                selection: crate::Selection::All,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![
                    Rule {
                        search: crate::SearchType::FileName,
                        condition: crate::Condition::NotIncludes,
                        data: crate::Data {
                            text: vec!["poke".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                    Rule {
                        search: crate::SearchType::FileName,
                        condition: crate::Condition::NotIncludes,
                        data: crate::Data {
                            text: vec!["file".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                ],
                actions: vec![Action {
                    action: ActionOpts::MOVE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &["main.rs", "main.py", "blankfile", "pokemon.go", "trash/"],
        );

        assert_exists(&tempdir, &["blankfile", "trash/", "pokemon.go"]);
        assert_exists(&destination, &["main.rs", "main.py"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn file_extension_not_includes() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "FileExtension NotIncludes".into(),
                selection: crate::Selection::All,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::NotIncludes,
                        data: crate::Data {
                            text: vec!["rs".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::NotIncludes,
                        data: crate::Data {
                            text: vec!["py".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                ],
                actions: vec![Action {
                    action: ActionOpts::MOVE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &["main.rs", "main.py", "blankfile", "pokemon.go", "trash/"],
        );

        assert_exists(&tempdir, &["main.rs", "trash/", "main.py"]);
        assert_exists(&destination, &["blankfile", "pokemon.go"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn file_extension_includes() -> Result<(), anyhow::Error> {
        let tempdir = TempDir::new()?.into_persistent();
        let destination = tempdir.child("playground/");
        std::fs::create_dir_all(&destination)?;

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "FileExtension NotIncludes".into(),
                selection: crate::Selection::Any,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::Includes,
                        data: crate::Data {
                            text: vec!["rs".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::Includes,
                        data: crate::Data {
                            text: vec!["py".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                ],
                actions: vec![Action {
                    action: ActionOpts::COPY,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &["main.rs", "main.py", "blankfile", "pokemon.go", "trash/"],
        );

        std::thread::sleep(Duration::from_secs(3));

        assert_exists(&tempdir, &["pokemon.go", "main.rs", "main.py"]);
        assert_exists(&destination, &["main.rs", "main.py"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn file_ext_exact_match() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "FileName ExactMatch".into(),
                selection: crate::Selection::Any,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::ExactMatch,
                        data: crate::Data {
                            text: vec!["Rs".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                    Rule {
                        search: crate::SearchType::FileExtension,
                        condition: crate::Condition::ExactMatch,
                        data: crate::Data {
                            text: vec!["py".into()],
                            size: (0, UnitOpts::B),
                        },
                    },
                ],
                actions: vec![Action {
                    action: ActionOpts::DELETE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &[
                "main.rs",
                "jinx.Rs",
                "main.py",
                "blankfile",
                "pokemon.go",
                "trash/",
            ],
        );

        std::thread::sleep(Duration::from_millis(TIMEOUT));

        assert_exists(&tempdir, &["pokemon.go", "main.rs", "blankfile", "trash/"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn file_ext_is_not() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "FileName ExactMatch".into(),
                selection: crate::Selection::Any,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![Rule {
                    search: crate::SearchType::FileExtension,
                    condition: crate::Condition::IsNot,
                    data: crate::Data {
                        text: vec!["Rs".into(), "py".into()],
                        size: (0, UnitOpts::B),
                    },
                }],
                actions: vec![Action {
                    action: ActionOpts::DELETE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &[
                "main.rs",
                "jinx.Rs",
                "main.py",
                "blankfile",
                "pokemon.go",
                "trash/",
            ],
        );

        std::thread::sleep(Duration::from_millis(TIMEOUT));

        assert_exists(&tempdir, &["main.py", "jinx.Rs"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn filesize_less() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "Filesize Less".into(),
                selection: crate::Selection::Any,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![Rule {
                    search: crate::SearchType::FileSize,
                    condition: crate::Condition::Less,
                    data: crate::Data {
                        text: vec![],
                        size: (10, UnitOpts::Kb), // Less than 10Kb
                    },
                }],
                actions: vec![Action {
                    action: ActionOpts::DELETE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &[
                "main.rs",
                "jinx.Rs",
                "main.py",
                "blankfile",
                "pokemon.go",
                "trash/",
            ],
        );

        std::thread::sleep(Duration::from_millis(TIMEOUT));

        assert_exists(&tempdir, &["trash/"]);

        tempdir.close()?;
        Ok(())
    }

    #[async_std::test]
    async fn filesize_is_equal() -> Result<(), anyhow::Error> {
        let (tempdir, destination) = setup_env(false, "playground/");

        let mut organizer = SmartOrganizer::new().await;
        organizer.watch_path(tempdir.path(), true);
        organizer
            .add_listener(&ListenerData {
                id: "1".into(),
                deep: false,
                enabled: true,
                title: "Filesize Less".into(),
                selection: crate::Selection::Any,
                monitors: vec![tempdir.to_path_buf()],
                rules: vec![Rule {
                    search: crate::SearchType::FileSize,
                    condition: crate::Condition::IsEqual,
                    data: crate::Data {
                        text: vec![],
                        size: (0, UnitOpts::B), // Less than 10Kb
                    },
                }],
                actions: vec![Action {
                    action: ActionOpts::DELETE,
                    path: destination.to_path_buf(),
                }],
            })
            .await;
        organizer.start();

        create_files(
            &tempdir,
            &[
                "main.rs",
                "jinx.Rs",
                "main.py",
                "blankfile",
                "pokemon.go",
                "trash/",
            ],
        );

        std::thread::sleep(Duration::from_millis(TIMEOUT));

        assert_exists(&tempdir, &["trash/"]);

        tempdir.close()?;
        Ok(())
    }
}

#[cfg(test)]
mod db_tests {
    use std::{borrow::BorrowMut, path::PathBuf};

    use sqlx::{Connection, SqliteConnection};

    use crate::{
        schemas::create_dbs, drop_tables, listeners, refresh_db, ListenerData, Rule, UnitOpts, DB, URI,
    };

    #[async_std::test]
    async fn test_all_sequential() -> Result<(), sqlx::Error> {
        remove_test().await?;
        insert_test().await?;
        update_test().await?;
        fetch_one_test().await?;
        fetch_all_test().await?;
        Ok(())
    }

    async fn remove_test() -> Result<(), sqlx::Error> {
        println!("RUNNING REMOVE TEST");
        let mut conn = SqliteConnection::connect(URI).await?;
        let id = String::from("id_1");

        refresh_db(&mut conn).await?;
        DB::remove(&mut conn, &id).await?;

        let value = DB::fetch(&mut conn, &id).await?;
        assert!(value.is_none());

        // ID not in database
        let id = String::from("id_1");
        DB::remove(&mut conn, &id).await?;
        let value = DB::fetch(&mut conn, &id).await?;
        assert!(value.is_none());

        Ok(())
    }

    async fn update_test() -> Result<(), sqlx::Error> {
        println!("RUNNING UPDATE TEST");
        let data = ListenerData {
            id: "id_1".into(),
            deep: true,
            title: String::from("Aaskdja"),
            enabled: false,
            selection: crate::Selection::All,
            monitors: vec![],
            rules: vec![],
            actions: vec![],
        };
        let mut conn = SqliteConnection::connect(URI).await?;

        DB::update(conn.borrow_mut(), &data).await?;
        let row = DB::fetch(conn.borrow_mut(), &data.id).await?;

        assert!(row.unwrap() == data);

        Ok(())
    }

    async fn fetch_one_test() -> Result<(), sqlx::Error> {
        println!("RUNNING FETCH_ONE TEST");
        let mut conn = SqliteConnection::connect(URI).await?;
        refresh_db(&mut conn).await?;
        let l = DB::fetch(&mut conn, &String::from("id_4")).await?;
        assert!(listeners().get(3) == l.as_ref());

        let l = DB::fetch(&mut conn, &String::from("id_124sa")).await?;
        assert!(l.is_none());

        Ok(())
    }

    async fn fetch_all_test() -> Result<(), sqlx::Error> {
        println!("RUNNING FETCH_ALL TEST");
        // Tests if all the inserted records are retrieved succesfully
        let mut conn = SqliteConnection::connect(URI).await?;
        refresh_db(&mut conn).await?;
        let l = DB::fetch_all(&mut conn).await?;
        assert!(listeners() == l);

        // Drops all tables
        drop_tables(&mut conn).await?;
        create_dbs(&mut conn).await?;
        let l = DB::fetch_all(&mut conn).await?;
        assert!(l.is_empty());

        Ok(())
    }

    async fn insert_test() -> Result<(), sqlx::Error> {
        println!("RUNNING INSERT TEST");
        let mut conn = SqliteConnection::connect(URI).await?;

        let data = ListenerData {
            id: "generated".into(),
            deep: true,
            title: String::from("Aaskdja"),
            enabled: false,
            selection: crate::Selection::All,
            monitors: vec![PathBuf::from("./backend/logger.rs")],
            rules: vec![Rule {
                search: crate::SearchType::FileName,
                condition: crate::Condition::Includes,
                data: crate::Data {
                    text: vec![String::from("rs")],
                    size: (1024, UnitOpts::Mb),
                },
            }],
            actions: vec![
                // Action {
                //     action: crate::ActionOpts::RENAME,
                //     path: PathBuf::from("/etc/X11/xinit/xinitrc.d/50-systemd-user.sh"),
                // }
            ],
        };

        refresh_db(&mut conn).await?;
        DB::insert(&mut conn, &data).await?;
        let sql_data = DB::fetch(&mut conn, &data.id).await?;

        assert!(data == sql_data.unwrap());

        Ok(())
    }
}
