use crate::{logger::LOGGER, Action, ActionOpts, Log, LogLevel};
use std::{fs, io, path::{Path, PathBuf}};

pub struct FSActions;
impl FSActions {
    pub fn operate<P: AsRef<Path>>(id: &str, title: &str, source: P, actions: &[Action]) -> Option<anyhow::Error> {
        let mut current = source.as_ref().to_path_buf();
        for action in actions {
            let result = execute(&current, action);
            let (level, message, destination) = match &result {
                Ok(destination) => (LogLevel::SUCCESS, format!("{} completed", action.action), destination.clone().unwrap_or_default()),
                Err(error) => (LogLevel::ERROR, format!("{} failed: {error}", action.action), action.path.clone()),
            };
            LOGGER.lock().unwrap().broadcast("log".into(), Log { id:nanoid::nanoid!(), name:title.into(), level, message, path:current.clone(), timestamp:chrono::Utc::now().to_rfc3339(), action:action.action.clone(), parent_id:id.into(), destination });
            match result {
                Ok(Some(path)) => { if matches!(action.action, ActionOpts::MOVE | ActionOpts::RENAME) { current=path; } },
                Ok(None) => break,
                Err(error) => return Some(error.into()),
            }
        }
        None
    }
}

fn execute(source: &Path, action: &Action) -> io::Result<Option<PathBuf>> {
    let metadata=fs::symlink_metadata(source)?;
    // Never follow a link into an unrelated directory tree during destructive actions.
    if metadata.file_type().is_symlink() { return Err(io::Error::other("Symbolic links are not supported")); }
    match action.action {
        ActionOpts::DELETE => { trash::delete(source).map_err(io::Error::other)?; Ok(None) },
        ActionOpts::UNLINK => { if metadata.is_dir() { fs::remove_dir_all(source)?; } else { fs::remove_file(source)?; } Ok(None) },
        ActionOpts::MOVE | ActionOpts::COPY | ActionOpts::RENAME => {
            if action.path.as_os_str().is_empty() { return Err(io::Error::other("Choose a destination")); }
            let destination=if matches!(action.action,ActionOpts::RENAME) { action.path.clone() } else {
                if !action.path.is_dir() { return Err(io::Error::other("Destination must be an existing folder")); }
                action.path.join(source.file_name().ok_or_else(||io::Error::other("Cannot operate on a filesystem root"))?)
            };
            if fs::symlink_metadata(&destination).is_ok() { return Err(io::Error::new(io::ErrorKind::AlreadyExists,"Destination already exists; nothing was overwritten")); }
            let parent=destination.parent().ok_or_else(||io::Error::other("Invalid destination"))?.canonicalize()?;
            if parent.starts_with(source.canonicalize()?) { return Err(io::Error::other("Destination cannot be inside the source")); }
            if matches!(action.action,ActionOpts::COPY) { copy_new(source,&destination)?; }
            else if metadata.is_file() {
                // Reserve the new filename atomically; never replace a racing writer.
                fs::hard_link(source,&destination)?;
                fs::remove_file(source)?;
            } else { fs::rename(source,&destination)?; }
            Ok(Some(destination))
        }
    }
}

fn copy_new(source:&Path,destination:&Path)->io::Result<()> {
    let metadata=fs::symlink_metadata(source)?;
    if metadata.file_type().is_symlink() {return Err(io::Error::other("Symbolic links are not supported"));}
    if metadata.is_dir() {
        fs::create_dir(destination)?;
        for entry in fs::read_dir(source)? { let entry=entry?; copy_new(&entry.path(),&destination.join(entry.file_name()))?; }
    } else {
        let mut input=fs::File::open(source)?;
        let mut output=fs::OpenOptions::new().write(true).create_new(true).open(destination)?;
        io::copy(&mut input,&mut output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn copy_does_not_overwrite_and_copies_directories() {
        let dir=assert_fs::TempDir::new().unwrap();let source=dir.path().join("source");let target=dir.path().join("target");fs::create_dir(&source).unwrap();fs::create_dir(&target).unwrap();fs::write(source.join("note"),"original").unwrap();
        let action=Action{action:ActionOpts::COPY,path:target.clone()};execute(&source,&action).unwrap();assert_eq!(fs::read_to_string(target.join("source/note")).unwrap(),"original");
        assert!(execute(&source,&action).is_err());assert!(execute(&source,&Action{action:ActionOpts::MOVE,path:source.clone()}).is_err());assert!(source.join("note").exists());
    }
    #[test] fn action_chain_uses_moved_path() {
        let dir=assert_fs::TempDir::new().unwrap();let source=dir.path().join("note.txt");let moved=dir.path().join("moved");let copied=dir.path().join("copied");fs::create_dir(&moved).unwrap();fs::create_dir(&copied).unwrap();fs::write(&source,"keep me").unwrap();
        assert!(FSActions::operate("test","test",&source,&[Action{action:ActionOpts::MOVE,path:moved.clone()},Action{action:ActionOpts::COPY,path:copied.clone()}]).is_none());
        assert!(!source.exists());assert_eq!(fs::read_to_string(moved.join("note.txt")).unwrap(),"keep me");assert_eq!(fs::read_to_string(copied.join("note.txt")).unwrap(),"keep me");
    }

}
