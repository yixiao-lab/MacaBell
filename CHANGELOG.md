# Changelog

All notable changes to MacaBell will be documented in this file.

The format is inspired by Keep a Changelog, and this project follows a simple versioning style while it is still early.

## [Unreleased]

### Planned

- Add a `macabell notify` command-line interface for local task notifications.

## [0.2.0] - 2026-06-11

### Added

- Universal macOS builds for both Apple Silicon and Intel Macs.
- GitHub Actions release workflow that builds and uploads a universal `.dmg`.
- Tray menu item to open `~/.MacaBell/reminders.json` directly.
- Tray menu item to reveal the config folder in Finder.
- Friendly popup message when the JSON config is invalid.
- Stacked reminder popups when multiple reminders fire at the same time.
- Example `reminders.json` under `examples/` and clearer screenshots in the README.

### Notes

- The build is still unsigned, so macOS may show a developer verification warning on first launch.

## [0.1.0] - 2026-06-07

### Added

- Initial public release of MacaBell.
- macOS menu bar reminder app built with Tauri 2, React, TypeScript, and Rust.
- Local JSON based reminder configuration.
- Three reminder types: daily reminders, interval reminders, and yearly anniversary reminders.
- Soft pastel popup window that slides in from the right side of the screen.
- Global sound toggle from the tray menu.
- Auto launch toggle from the tray menu.
- Test reminder action from the tray menu.
- Apple Silicon `.dmg` release package.

### Notes

- The app is currently unsigned, so macOS may show a developer verification warning on first launch.
- The first release focuses on proving the core interaction and the local-first reminder model.
