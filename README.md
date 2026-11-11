# SmartOrganizer

SmartOrganizer is a powerful application designed to enhance your desktop/workspace tidiness by allowing you to define custom rules and actions for automated organization tasks. Whether you need to declutter your desktop, sort files into specific folders, or clean up unused directories, SmartOrganizer has you covered.

# Demo

https://github.com/dre-bk201/SmartOrganizer/assets/48803238/2f11a4d0-ca4b-4b2c-8f44-ef84fb9292a6



## Personal UseCases

- **Book Sorting:** Organize your light novel books, coding books, or any other category with ease.
- **Document Management:** Efficiently categorize and move school documents into appropriate folders.
- **Directory Cleanup:** Automatically clean up unused environment directories or project folders, such as `python venv` directories, `node_modules/`, and `src/target/`.

## How It Works

SmartOrganizer simplifies the task of maintaining a tidy workspace. You can define rules and actions using a simple configuration, and the application will take care of the rest.


## Development

Requires Node.js 22.12+ (Node 24 LTS recommended), pnpm 11.25, stable Rust 1.97+, and the [Tauri 2 platform prerequisites](https://v2.tauri.app/start/prerequisites/).

```sh
pnpm install --frozen-lockfile
pnpm tauri dev
pnpm build
cargo check --manifest-path src-tauri/Cargo.toml
```

`pnpm dev` serves the frontend only; filesystem, window and updater APIs require the desktop runtime.

## Redesign concepts

Open `docs/redesign-mockups.html` to compare Workspace, Focus and Flow. They use illustrative data and do not modify files. Flow is now implemented in the production interface: crimson workspace, combined Watch / Match / Act editor, updated journal and statistics, and light/dark appearances.

## Dependency upgrade

Migrated to Tauri 2 with explicit desktop capabilities and dialog, OS, clipboard, process and updater plugins, following the [official migration guide](https://v2.tauri.app/start/migrate/from-tauri-1/). Updated frontend tooling and compatible Rust lockfile dependencies. PrimeVue 3, Tailwind 3, Anime.js 3, tailwind-merge 1, TypeScript 5, SQLx 0.7 and notify-debouncer-full 0.3 remain on compatible major lines; adopting their newer APIs is a separate migration. Release builds now use stable Rust, Node 24 and WebKitGTK 4.1 on Linux. Existing signing secrets are mapped to Tauri 2 environment variable names.

## Validation

- `pnpm install --frozen-lockfile`, `pnpm build`, and `cargo check --manifest-path src-tauri/Cargo.toml` pass.
- Mockups inspected at desktop and narrow widths using HTML preview.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib` runs the isolated regression suite; legacy database-touching tests were replaced. See docs/logic-review.md.
- Desktop file operations, signed updater installation, and platform release bundles still need runtime verification.

## System tray

Closing the main window hides it to the system tray; folder listeners continue running. Left-click the tray icon to restore the window (including when minimized). Right-click for Show SmartOrganizer, Hide window, and Quit SmartOrganizer. Quit stops the app and folder automation. Linux desktop environments may expose only the tray menu; use Show SmartOrganizer there. Minimize retains its normal taskbar behavior.

Tray setup runs in Rust once per app process, so frontend hot reload does not duplicate icons. The effect uses Tauri's built-in system tray integration: https://v2.tauri.app/learn/system-tray/.

## Existing files when enabling a listener

Creating an enabled listener, enabling a paused listener, or changing an active listener's rules, watched folders, recursion, matching mode, or actions first applies that listener to existing entries, then enables live watching. The initial pass respects Include subfolders, skips symbolic links and output folders, and uses the same matching and file actions as live events. It runs off the async thread; saving waits for the pass to finish. Unchanged saves, name-only edits, disabled drafts and startup restoration do not trigger a sweep. Failed file actions use the normal journal logging path; inaccessible folders are reported to stderr and watching still starts.

## Remembered window size

Window dimensions and maximized state are saved on normal app exit and restored on the next launch using Tauri's window-state plugin. Hiding to the tray preserves the current window. The next launch always shows the window, even if the previous session exited while hidden. Window position, fullscreen mode, and titlebar decoration preferences are not restored by this plugin.
