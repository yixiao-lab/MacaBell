# Changelog

All notable changes to MacaBell will be documented in this file.

The format is inspired by Keep a Changelog, and this project follows a simple versioning style while it is still early.

## [Unreleased]

### Planned

- Add universal macOS builds for both Apple Silicon and Intel Macs.
- Add GitHub Actions release workflow for building `.dmg` files.
- Add a menu item to open the local reminder config file directly.
- Improve error handling when `reminders.json` is invalid.
- Improve handling when multiple reminders are triggered at the same time.
- Add more screenshots and a short demo GIF to the README.

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
