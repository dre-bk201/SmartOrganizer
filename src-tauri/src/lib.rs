#![allow(unused)]

use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::{mpsc::Receiver, Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Local;
use glob::glob;
use notify::{DebouncedEvent, Result, Watcher};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoolFunc {
    Any, // OR Boolean Function
    All, // AND Boolean Function
}

// Instance of logger that is instanciated at runtime
static mut LOGGER: Logger = Logger::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenerData {
    pub id: String,
    pub deep: bool,
    pub enabled: bool,
    pub selection: BoolFunc,
    pub paths: Vec<PathBuf>,
    pub rules: Vec<Rule>,
    pub actions: Vec<Action>,
}

pub struct SmartOrganizer {
    data: Arc<Mutex<Vec<ListenerData>>>,
    watched_paths: HashMap<PathBuf, bool>,
    watcher: notify::RecommendedWatcher,
    recv: Arc<Mutex<Receiver<DebouncedEvent>>>,
}

impl std::fmt::Debug for SmartOrganizer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SmartOrganizer")
            .field("data", &self.data)
            .field("watched_paths", &self.watched_paths)
            .field("recv", &self.recv)
            .finish()
    }
}

impl SmartOrganizer {
    /// Insantiates a new `SmartOrganizer`
    pub fn new() -> SmartOrganizer {
        let (send, recv) = std::sync::mpsc::channel();
        let watcher = notify::Watcher::new(send, std::time::Duration::from_secs(1))
            .expect("Failed to start watcher");

        SmartOrganizer {
            data: Arc::new(Mutex::new(Vec::new())),
            watcher,
            watched_paths: HashMap::new(),
            recv: Arc::new(Mutex::new(recv)),
        }
    }

    /// Instantiates window at runtime
    pub fn set_window(&mut self, window: tauri::Window) {
        unsafe {
            dbg!("Instantiating Logger");
            LOGGER.set_window(window);
        }
    }

    /// Checks if any a listener should receive the path based on its `deep` field
    fn recv_event<P: AsRef<Path>>(data: &ListenerData, path: P) -> bool {
        // Checks "Should this listener receive the event?"
        if data.deep {
            return true;
        }

        let mut returnv = false;

        data.paths.iter().for_each(|watched_path| {
            // checks if the event_path is a subdir of the watched_path
            let found = path.as_ref().starts_with(watched_path);

            let w = watched_path.components().collect::<Vec<_>>();
            let p = path.as_ref().components().collect::<Vec<_>>();

            if found {
                let diff = w.len().abs_diff(p.len());

                // Watches events in immediate directories
                if diff == 2 {
                    returnv = true;
                }

                // Watches immediate changes in the watched_path
                if diff > 0 && diff <= 1 {
                    returnv = true;
                }
            }
        });

        returnv
    }

    /// Organizes the path supplied if it matches rules
    fn organize_path<P: AsRef<Path>>(path: P, data: &ListenerData) {
        if !path.as_ref().exists() {
            println!("{:?} does not exist", path.as_ref().display());
            return;
        }

        // Checks if listener should process this event if it is deep
        let recv_event = Self::recv_event(&data, path.as_ref());

        if data.enabled && recv_event {
            let is_match = Self::apply_rule(path.as_ref(), &data.selection, &data.rules);

            if is_match {
                unsafe {
                    // Unsafe because of `LOGGER`
                    FileOperations::new(
                        &data.id,
                        path.as_ref().to_path_buf(),
                        &data.actions,
                        &LOGGER,
                    )
                    .perform_ops();
                }
            }
        }
    }

    pub fn organize<P: AsRef<Path>>(path: P, listener_data: &Vec<ListenerData>) {
        for data in listener_data.iter() {
            Self::organize_path(&path, &data);
        }
    }

    /// Starts the fs receiver channel on another thread
    pub fn listen(&mut self) {
        let data = self.data.clone();
        let recv = self.recv.clone();

        dbg!("Listening");

        std::thread::Builder::new()
            .name("signal_recv".to_owned())
            .spawn(move || loop {
                match recv.lock().unwrap().recv() {
                    Ok(event) => {
                        match event {
                            // Parses on path created
                            DebouncedEvent::Create(path) => {
                                Self::organize(&path, &data.lock().unwrap());
                            }

                            // Parses on path rename or move
                            DebouncedEvent::Rename(from, to) => {
                                Self::organize(&to, &data.lock().unwrap());
                            }

                            // Parses on file changes and any write action
                            DebouncedEvent::Write(path) => {
                                Self::organize(&path, &data.lock().unwrap());
                            }

                            DebouncedEvent::Remove(path) => {
                                Self::organize(&path, &data.lock().unwrap());
                            }

                            _ => {}
                        }
                    }
                    Err(e) => (),
                }
            });
    }

    /// Adds a new path to watch with the option of recursively watching
    pub fn watch_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        mode: notify::RecursiveMode,
    ) -> notify::Result<()> {
        match mode {
            notify::RecursiveMode::Recursive => {
                self.watcher.watch(path, mode)?;
            }

            notify::RecursiveMode::NonRecursive => {
                self.watcher.watch(path, mode)?;
            }
        };

        Ok(())
    }

    /// Unwatches path if it exists
    pub fn unwatch_path<P: AsRef<Path>>(&mut self, path: P) -> notify::Result<()> {
        self.watcher.unwatch(&path.as_ref())?;
        Ok(())
    }

    /// Modifies the whether to watch path recursively or non-recursively
    pub fn change_mode<P: AsRef<Path>>(
        &mut self,
        path: P,
        mode: notify::RecursiveMode,
    ) -> notify::Result<()> {
        self.unwatch_path(&path)?;
        self.watch_path(&path, mode);
        Ok(())
    }

    // issue: Adding watchers as deep or non-deep will conflict
    // seeing that the current architecture allows for only one watcher per path

    // possible workaround: add all watchers as deep by default and if listener is not deep,
    // then ignore file events on that path

    /// Updates listener
    pub fn update_listener(&mut self, listener_data: ListenerData) {
        println!("Updating Listener: {listener_data:?}");
        self.remove_listener(&listener_data);
        self.add_listener(listener_data.clone())
    }

    /// Adds a listener to SmartOrganizer if listener does not already exists
    pub fn add_listener(&mut self, listener: ListenerData) {
        let exists = self
            .data
            .lock()
            .unwrap()
            .iter()
            .any(|l| l.id == listener.id);

        if !exists {
            // Watches deep by default
            let mode = notify::RecursiveMode::Recursive;

            for path in &listener.paths {
                // Prevents having duplicate paths in the watcher
                // Essentially only one path is subscribed to
                let inserted = self.watched_paths.insert(path.clone(), listener.enabled);

                if let None = inserted {
                    self.watch_path(path, mode);
                }
            }

            self.data.lock().unwrap().push(listener);
        }

        dbg!("Listener not added");
    }

    pub fn remove_listener(&mut self, listener: &ListenerData) {
        // Unwatches path(s) for the listener to be removed, if no other listener is subscribed to that path
        for rmpath in listener.paths.iter() {
            let keep = self
                .data
                .lock()
                .unwrap()
                .iter()
                .fold(false, |a, b| a | b.paths.contains(rmpath));

            if !keep {
                self.watched_paths.remove(rmpath);
                self.unwatch_path(rmpath);
            }

            self.data.lock().unwrap().retain(|d| d.id != listener.id)
        }
    }

    fn glob_search(path: &PathBuf, deep: bool) -> Vec<PathBuf> {
        let path_as_str = path.to_str().unwrap();
        if deep {
            let r = glob(&format!("{}/**/*", path_as_str))
                .expect("Failed to parse glob")
                .map(|b| b.unwrap())
                .collect::<Vec<_>>();
            return r;
        } else {
            let r = glob(&format!("{}/*", path_as_str))
                .expect("Failed to parse glob")
                .map(|b| b.unwrap())
                .collect::<Vec<_>>();
            return r;
        }
    }

    /// Checks if path follows all the provided `Rules`
    fn apply_rule<P: AsRef<Path>>(path: P, selection: &BoolFunc, rules: &Vec<Rule>) -> bool {
        use Condition::*;

        let matches = rules
            .iter()
            .map(|rule| match rule.condition {
                Includes => Self::includes(&path, &rule.search_type, &rule.text),
                NotIncludes => !Self::includes(&path, &rule.search_type, &rule.text),
                IsNot => Self::exact_match(&path, &rule.search_type, &rule.text),
                ExactMatch => Self::exact_match(&path, &rule.search_type, &rule.text),
                Greater => Self::greater(&path, &rule.search_type, &rule.text),
                Less => !Self::greater(&path, &rule.search_type, &rule.text),
            })
            .collect::<Vec<_>>();

        Self::match_bool_func(&selection, &matches)
    }

    /// Matches
    fn match_bool_func(selection: &BoolFunc, matches: &Vec<bool>) -> bool {
        match selection {
            BoolFunc::Any => matches.iter().skip(1).fold(matches[0], |a, b| a | b),
            BoolFunc::All => matches.iter().skip(1).fold(matches[0], |a, b| a & b),
        }
    }

    /// Parses if `SearchType::FileSize` is greater than the value passed
    fn greater<P: AsRef<Path>>(p: P, search_type: &SearchType, text: &String) -> bool {
        match search_type {
            SearchType::FileSize => {
                if let Some(parse_int) = text.parse::<usize>().ok() {
                    let path = p.as_ref();
                    if let Some(metadata) = path.metadata().ok() {
                        return metadata.len() as usize > parse_int;
                    }
                }
                false
            }
            _ => false,
        }
    }

    /// Parses path according to search_type and checks if text is contained
    pub fn includes<P: AsRef<Path>>(p: P, search_type: &SearchType, text: &String) -> bool {
        use SearchType::*;
        let path = p.as_ref();
        let is_dir = path.is_dir();

        let to_match = text.to_lowercase();

        match search_type {
            FileName => {
                // Only if path is a file
                if !is_dir {
                    if let Some(filename) = path.file_stem() {
                        let filename = filename.to_str().unwrap().to_lowercase();
                        return filename.contains(&to_match);
                    }
                }
            }

            FileContent => {
                use sliceslice::x86::DynamicAvx2Searcher;
                if !is_dir {
                    if let Some(file_content) = std::fs::read_to_string(p).ok() {
                        let searcher = unsafe { DynamicAvx2Searcher::new(to_match.as_bytes()) };
                        let lowercase = file_content.to_lowercase();

                        return unsafe { searcher.search_in(lowercase.as_bytes()) };
                    }
                }
            }

            FileExtension => {
                if let Some(extension) = path.extension() {
                    if let Some(as_str) = extension.to_str() {
                        return as_str == to_match;
                    }
                }
            }

            // Add support on front end and possibly some other enums to match for
            // Like Greater than or Less than
            FileSize => {
                if !is_dir {
                    if let Ok(metadata) = path.metadata() {
                        return metadata.len() > 12;
                    }
                }
            }
            FolderName => {
                if is_dir {
                    let foldername = path.file_name().unwrap().to_str().unwrap();
                    return foldername.to_lowercase().contains(&to_match);
                }
            }

            PathName => {
                if let Ok(abs_path) = path.canonicalize() {
                    let as_str = abs_path.to_string_lossy();
                    let normalized = as_str.to_lowercase().replace("\\", "/");

                    return normalized.contains(&to_match.replace("\\", "/"));
                }
            }
        };
        false
    }

    pub fn exact_match<P: AsRef<Path>>(p: P, search_type: &SearchType, text: &String) -> bool {
        use SearchType::*;
        let path = p.as_ref();
        let is_dir = path.is_dir();

        match search_type {
            FileName => {
                if !is_dir {
                    let filename = path.file_stem().unwrap().to_str().unwrap();
                    return filename == text;
                }
                false
            }

            FileContent => {
                if !is_dir {
                    use sliceslice::x86::DynamicAvx2Searcher;

                    if let Ok(file) = std::fs::File::open(path) {
                        let mut reader = BufReader::new(file);
                        let mut buffer = Vec::new();

                        if let Some(_) = reader.read_to_end(&mut buffer).ok() {
                            let searcher = unsafe { DynamicAvx2Searcher::new(text.as_bytes()) };
                            return unsafe { searcher.search_in(&buffer) };
                        }
                    }
                }
                false
            }

            FolderName => {
                if is_dir {
                    let foldername = path.file_name().unwrap().to_str().unwrap();
                    return foldername == text;
                }
                false
            }

            // Directories can have extensions in filename
            FileExtension => {
                if let Some(extension) = path.extension() {
                    if let Some(as_str) = extension.to_str() {
                        return as_str == text;
                    }
                }
                false
            }

            FileSize => false,

            PathName => {
                if let Ok(abs_path) = path.canonicalize() {
                    if let Some(as_str) = abs_path.to_str() {
                        let normalized = as_str.replace("\\", "/");
                        return normalized == text.replace("\\", "/");
                    }
                }
                false
            }
        }
    }
}

struct Logger {
    window: Option<tauri::Window>,
}

impl Logger {
    pub const fn new() -> Self {
        Self { window: None }
    }

    pub fn set_window(&mut self, window: tauri::Window) {
        self.window = Some(window);
    }

    // Sends event `logger` to frontend
    pub fn log(&self, log: Log) {
        if let Some(window) = &self.window {
            window.emit("logger", log).expect("Failed to emit log");
        }
    }
}
struct FileOperations<'a> {
    id: &'a String,
    path: PathBuf,
    actions: &'a Vec<Action>,
    logger: &'a Logger,
}

#[derive(Serialize)]

struct User<'a> {
    pub name: &'a String,
}

impl<'a> FileOperations<'a> {
    pub fn new(
        id: &'a String,
        path: PathBuf,
        actions: &'a Vec<Action>,
        logger: &'a Logger,
    ) -> Self {
        FileOperations {
            id,
            path,
            actions,
            logger,
        }
    }

    pub fn perform_ops(&self) {
        use ActionType::*;
        for Action(action, to) in self.actions.iter() {
            // let log = Log {

            // }
            match action {
                MOVE => {
                    self.logger.log(Log {
                        parent_id: &self.id,
                        action: &action,
                        path: &self.path,
                        destination: &to,
                        id: Uuid::new_v4().to_string(),
                        timestamp: Local::now().to_rfc2822(),
                    });
                    let to = to.join(self.path.file_name().unwrap());

                    if self.path.exists() && !to.exists() {
                        match std::fs::rename(&self.path, to) {
                            Ok(_) => println!("File successfully moved"),
                            Err(e) => println!("Failed to move file: {}", e),
                        }
                    }
                }
                COPY => {
                    // self.logger.log("Copying File");
                    if self.path.exists() && to.exists() {
                        match std::fs::copy(&self.path, to) {
                            Ok(_) => println!("Copying is a success"),
                            Err(e) => println!("Copying is a failure: {}", e),
                        }
                    }
                }

                DELETE => {
                    if self.path.exists() {
                        match trash::delete(&self.path) {
                            Ok(_) => println!("File successfully deleted"),
                            Err(e) => println!("Failed to delete file: {}", e),
                        }
                    }
                }

                RENAME => {
                    if self.path.exists() && to.exists() {
                        match std::fs::rename(&self.path, to) {
                            Ok(_) => println!("File successfully renamed"),
                            Err(e) => println!("Failed to rename file: {}", e),
                        }
                    }
                }

                UNLINK => {
                    if self.path.exists() {
                        match std::fs::remove_file(&self.path) {
                            Ok(_) => println!("File successfully unlinked"),
                            Err(e) => println!("Failed to unlink file: {}", e),
                        }
                    }
                }
            };
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SearchType {
    FolderName,
    FileName,
    FileExtension,
    FileContent,
    FileSize,
    PathName,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Condition {
    Includes,
    NotIncludes,
    ExactMatch,
    IsNot,
    Greater,
    Less,
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub enum ActionType {
    MOVE,
    COPY,
    DELETE,
    UNLINK,
    RENAME,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub search_type: SearchType,
    pub condition: Condition,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action(ActionType, PathBuf);

#[derive(Debug, Serialize, Clone)]
pub struct Log<'a> {
    pub parent_id: &'a String,
    pub id: String,
    pub action: &'a ActionType,
    pub timestamp: String,
    pub path: &'a PathBuf,
    pub destination: &'a PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const PATH: &str = "smorg";
    const FILES: [&str; 8] = [
        "Naruto Episode 1.mp4",
        "Naruto Episode 2.mkv",
        "main.rs",
        "argparser.py",
        "README.md",
        "Todo.txt",
        "index.html",
        "data",
    ];

    const FOLDERS: [&str; 5] = ["Audio", "Documents", "Images", "Videos", "Other"];
    fn create_test_env() {
        let basepath = std::env::temp_dir().join(&PATH);
        let file_content = [
            "Naruto Episode 1 Content",
            "Naruto Episode 2 Content",
            "fn main() {\n println!(\"hello world\")}",
            "import random\nrandom.randint(1,10)",
            "This is the readme for smartorganizer",
            "[x] Buy Eggs\n[ ] Buy Milk",
            "
            <!DOCTYPE html>
            <html lang='en'>
            <head>
                <meta charset='UTF-8' />
                <link rel='icon' href='/favicon.ico' />
                <meta name='viewport; content='width=device-width, initial-scale=1.0' />
                <title>Svelte + TS + Vite App</title>
            </head>
            <body>
            </body>
            </html>
           ",
            "This is the data file that doesn't have an extension",
        ];

        // Create directories
        for &path in FOLDERS.iter() {
            std::fs::create_dir_all(basepath.join(&path)).unwrap();
        }

        // Create files
        for (idx, &fname) in FILES.iter().enumerate() {
            let filepath = basepath.join(fname);
            std::fs::write(&filepath, file_content[idx]).expect("Failed to write file");
        }
    }

    fn clean_test_env() {
        let basepath = std::env::temp_dir().join(&PATH);

        std::fs::remove_dir_all(&basepath).unwrap();
    }

    struct OrganizerState {
        pub organizer: Arc<Mutex<SmartOrganizer>>,
    }

    struct Includes(String, SearchType, String);

    const FILEEXTENSION: [&str; 3] = ["Fifa 18.tar.gz", "Notes.txt", "bin"];
    const FILENAME: [&str; 3] = ["Fifa 18.tar.gz", "", "bin"];

    fn run_test(test: Includes, result_case: bool) {
        let test_case = SmartOrganizer::includes(&test.0, &test.1, &test.2);

        println!(
            "Test Case: {}    {test_case} => {result_case} =====> {}",
            test.0,
            test_case == result_case
        );
        assert_eq!(test_case, result_case);
    }

    #[test]
    fn test_recv_event() {
        let mut data = ListenerData {
            id: String::from("test"),
            actions: vec![Action(ActionType::MOVE, PathBuf::from("/home/test"))],
            deep: false,
            rules: vec![Rule {
                search_type: SearchType::FileName,
                condition: Condition::Includes,
                text: String::from("Fifa 18"),
            }],
            selection: BoolFunc::Any,
            enabled: true,
            paths: vec![PathBuf::from("C:/Users/user/Desktop")],
        };

        // Tests for events
        let path = PathBuf::from("C:/Users/user/Desktop/fifa/11/filename.txt");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, false);

        let path = PathBuf::from("C:/Users/user/Desktop/filename.txt");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, true);

        let path = PathBuf::from("C:/Users/user/Desktop/fifa/Fifa 18.tar.gz");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, true);

        let path = PathBuf::from("C:/Users/user/Desktop");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, false);

        data.deep = true;
        let path = PathBuf::from("C:/Users/user/Desktop/fifa/11/filename.txt");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, true);

        let path = PathBuf::from("C:/Users/user/Desktop/fifa/11/9900/filename.txt");
        let result = SmartOrganizer::recv_event(&data, &path);

        assert_eq!(result, true);
    }

    #[test]
    fn test_create_env() {
        create_test_env()
    }

    // Includes Testing
    #[test]
    fn test_includes_file_ext() {
        // ["Fifa 18.tar.gz", "Notes.txt", "bin"]
        let resultcases = [true, true, false];
        let testcases = ["Gz", "txt", ""];

        FILEEXTENSION
            .iter()
            .zip(testcases)
            .zip(resultcases)
            .for_each(|((&test, case), pass)| {
                run_test(
                    Includes(test.to_owned(), SearchType::FileExtension, case.to_owned()),
                    pass,
                )
            });
    }

    #[test]
    fn test_includes_file_name() {
        // ["Fifa 18.tar.gz", "", "bin"]

        let basepath = std::env::temp_dir().join(&PATH);
        let resultcases = [true, false, true];
        let testcases = ["FIFA 18.tar", "", "bin"];

        // Test for file extension
        let path = "Fifa 18.tar.gz";
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::FileName, &"FIFA 18.tar".to_owned()),
            true
        );

        // Test
        let path = "";
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::FileName, &"".to_owned()),
            false
        );

        // File without extension
        let path = "bin";
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::FileName, &"bin".to_owned()),
            true
        );

        // FILENAME
        //     .iter()
        //     .zip(testcases)
        //     .zip(resultcases)
        //     .for_each(|((&test, case), pass)| {
        //         println!("{test}, {case}");
        //         run_test(
        //             Includes(test.to_owned(), SearchType::FileName, case.to_owned()),
        //             pass,
        //         )
        //     });
    }

    #[test]
    fn test_includes_path_name() {
        // Dirs have to exist
        let basepath = std::env::temp_dir().join(&PATH);

        // Tests for case-sensitive paths
        let path = &basepath.join("audio");
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::PathName, &"audio".to_owned()),
            true
        );

        // Tests for path type strings
        let path = &basepath;
        assert_eq!(
            SmartOrganizer::includes(
                &path,
                &SearchType::PathName,
                &format!("{}", basepath.display())
            ),
            true
        );

        // Tests if path does not exist
        let path = &basepath.join("Pokemons");
        assert_eq!(
            SmartOrganizer::includes(
                &path,
                &SearchType::PathName,
                &format!("{}", basepath.join("Pokemons").display())
            ),
            false
        );
    }

    #[test]
    fn test_includes_file_content() {
        let basepath = std::env::temp_dir().join(&PATH);

        let path = &basepath.join("main.rs");
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::FileContent, &"fn main()".to_owned()),
            true
        );

        // Tests if content is case-sensitive
        let path = &basepath.join("argparser.py");
        assert_eq!(
            SmartOrganizer::includes(&path, &SearchType::FileContent, &"IMPORT random".to_owned()),
            true
        );
    }

    #[test]
    fn test_includes_folder_name() {}

    // Not Includes

    #[test]
    fn test_not_includes_file_ext() {}

    #[test]
    fn test_not_includes_file_name() {}

    #[test]
    fn test_not_includes_path_name() {}

    #[test]
    fn test_not_includes_file_content() {}

    #[test]
    fn test_not_includes_folder_name() {}

    // Exact Match Testing
    #[test]
    fn test_exact_match_file_ext() {}

    #[test]
    fn test_exact_match_file_name() {}

    #[test]
    fn test_exact_match_path_name() {}

    #[test]
    fn test_exact_match_file_content() {}

    #[test]
    fn test_exact_match_folder_name() {}

    // Is Not Testing
    #[test]
    fn test_is_not_file_ext() {}

    #[test]
    fn test_is_not_file_name() {}

    #[test]
    fn test_is_not_path_name() {}

    #[test]
    fn test_is_not_file_content() {}

    #[test]
    fn test_is_not_folder_name() {}
}
//     #[test]
//     fn test_includes() {
//         use super::SearchType::*;

//         let basepath = std::env::temp_dir().join(&PATH);

//         let run_test = |tests: Vec<Include>, pass: [bool; 3]| {
//             for (idx, test) in tests.iter().enumerate() {
//                 println!("Testing {:?} against {}", test, pass[idx]);
//                 let includes =
//                     SmartOrganizer::includes(test.path.clone(), &test.search_type, &test.text);
//                 println!("{includes} == {}", pass[idx]);
//                 assert_eq!(
//                     SmartOrganizer::includes(test.path.clone(), &test.search_type, &test.text),
//                     pass[idx]
//                 )
//             }
//         };

//         // FileExtension
//         let mut tests: Vec<Include> = vec![];
//         let mut testcases = vec!["GZ", "txt", ""];
//         let pass = [true, true, false];

//         for (idx, &ext) in FILEEXT.iter().enumerate() {
//             tests.push(Include::new(
//                 ext.to_owned(),
//                 FileExtension,
//                 testcases[idx].to_owned(),
//             ));
//         }

//         run_test(tests, pass);

//         // FileSize
//         // FileContent
//         // FolderName

//         // FileName
//         let mut tests: Vec<Include> = vec![];
//         let mut testcases = vec!["Fifa 18.tar", "Notes", "bin"];
//         let pass = [true, true, true];

//         for (idx, &name) in FILENAME.iter().enumerate() {
//             tests.push(Include::new(
//                 name.to_owned(),
//                 FileName,
//                 testcases[idx].to_owned(),
//             ));
//         }

//         run_test(tests, pass);

//         // PathName
//         let mut tests: Vec<Include> = vec![];
//         let mut testcases = vec!["Fifa 18.tar", "Notes", "bin"];
//         let pass = [true, true, true];

//         for (idx, &name) in FILENAME.iter().enumerate() {
//             tests.push(Include::new(
//                 name.to_owned(),
//                 FileName,
//                 testcases[idx].to_owned(),
//             ));
//         }

//         run_test(tests, pass);
//     }
// }
