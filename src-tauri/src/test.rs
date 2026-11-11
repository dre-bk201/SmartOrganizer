// All regression tests use an in-memory database or temporary filesystem.
use crate::*;
use sqlx::{Connection, SqliteConnection};
#[tokio::test]
async fn listener_roundtrip_update_remove_and_rollback() {
    let mut db=SqliteConnection::connect(":memory:").await.unwrap();
    ListenerQuery::create(&mut db).await.unwrap();
    LogQuery::create(&mut db).await.unwrap();
    let mut listener=ListenerData {id:"test".into(),title:"Documents".into(),rules:vec![Rule{search:SearchType::FileName,condition:Condition::Includes,data:Data{text:vec!["two words".into(),"other".into()],size:(0,UnitOpts::B)}}],monitors:vec!["folder".into()],..Default::default()};
    ListenerQuery::insert_one(&mut db,&listener).await.unwrap();
    assert_eq!(ListenerQuery::fetch_one(&mut db,&listener.id).await.unwrap(),Some(listener.clone()));
    listener.title="Updated".into();ListenerQuery::update_one(&mut db,&listener).await.unwrap();
    // A failure halfway through replacing children must preserve the original record.
    sqlx::query("CREATE TRIGGER reject_action BEFORE INSERT ON Action BEGIN SELECT RAISE(ABORT, 'test failure'); END").execute(&mut db).await.unwrap();
    let mut invalid=listener.clone();invalid.title="Must rollback".into();invalid.actions.push(Action{action:ActionOpts::COPY,path:"destination".into()});
    assert!(ListenerQuery::update_one(&mut db,&invalid).await.is_err());
    assert_eq!(ListenerQuery::fetch_one(&mut db,&listener.id).await.unwrap(),Some(listener.clone()));
    ListenerQuery::remove_one(&mut db,&listener.id).await.unwrap();
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM Rule").fetch_one(&mut db).await.unwrap();assert_eq!(count,0);
}
#[test]
fn size_units_and_event_scope() {
    assert_eq!(UnitOpts::Tb.get_size(1),1u64<<40);
    assert_eq!(UnitOpts::Tb.get_size(u64::MAX),u64::MAX);
    let mut listener=ListenerData{monitors:vec!["root".into()],..Default::default()};
    assert!(SmartOrganizer::should_recv_evt("root/file",&listener));
    assert!(!SmartOrganizer::should_recv_evt("root/sub/file",&listener));
    assert!(!SmartOrganizer::should_recv_evt("root-other/file",&listener));
    listener.actions.push(Action{action:ActionOpts::COPY,path:"root/output".into()});
    assert!(!SmartOrganizer::should_recv_evt("root/output/file",&listener));
    listener.deep=true;assert!(!SmartOrganizer::should_recv_evt("root",&listener));assert!(SmartOrganizer::should_recv_evt("root/sub/file",&listener));
}
