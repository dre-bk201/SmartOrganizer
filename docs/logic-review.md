# Logic review and corrections

## Corrected
- Rule evaluation extracted to a portable module: no AVX2 assumption, empty rule sets never match, strict size comparisons, all search terms considered, full-content exact comparisons, case-insensitive substring matching and safe missing-file handling.
- Fixed tebibyte conversion and saturated oversized size calculations.
- Event handling tolerates empty path lists and disconnected channels; worker snapshots state without holding its mutex throughout disk operations. Monitor roots and each listener's output tree are excluded.
- Watch registrations reconcile enabled listener state, shared recursive requirements, removed paths and recursion changes. Repeated loads do not duplicate in-memory listeners. Failed watch registrations remain retryable and are logged to stderr.
- Listener insert/update/delete operations use transactions; explicit child cleanup works with older schemas. Multiword rule values round-trip as JSON with legacy whitespace fallback. Database read/write failures propagate rather than being ignored or causing enum parse panics. Fixed malformed table creation SQL and LogQuery::drop incorrectly dropping listener data.
- File actions share one execution/error path. Folder copies work; file copies reserve targets with create_new. Moves/renames of files reserve a target using hard links before removing the source, preventing overwrite races. Existing targets are rejected, nested destinations and symlinks rejected, action chains follow moved paths, and execution no longer requires a UI window.
- IPC mutations report errors; frontend store updates require confirmed saves, deletion handles absent results, editor saves a cloned snapshot, and theme changes update DOM only after persistence succeeds.
- UNLINK no longer requires a destination; rename uses a save-path dialog.

## Verification and limits

The active Rust regression suite uses only temporary directories and an in-memory database. The obsolete test implementation was replaced; unused historical fakedata.rs is no longer linked into tests. Tests exercise matching, event scope, size units, database rollback/round trips, folder-copy collisions and move/copy chains. No user folders or app database are used by the suite.

Native filesystem watcher delivery and trash integration still require manual OS-level verification. Cross-volume moves return an error rather than risking a partial copy/delete. Failed directory copies can leave a partial destination, logged as failure; no automatic deletion is attempted. Symlink automation is deliberately rejected. Paths should be absolute as supplied by the desktop picker. Native watcher registration errors currently go to stderr, not a UI error indicator. Journal persistence still depends on the frontend's log event handler. This review does not certify all possible filesystem races or malicious concurrent changes.
