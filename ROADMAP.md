# Roadmap

MacaBell is a small, local-first macOS menu bar reminder app. The roadmap focuses on making it more stable, easier to install, easier to configure, and easier for developers to fork into other menu bar utilities.

## v0.2.0 — Make it easier to install and use

Goal: turn the current working app into a more polished open-source release.

- Add universal macOS builds for both Apple Silicon and Intel Macs.
- Add GitHub Actions workflow for release builds.
- Add a menu item to open `~/.MacaBell/reminders.json` directly.
- Add a menu item to reveal the config folder in Finder.
- Show a friendly error message when the JSON config is invalid.
- Improve reminder queue behavior when multiple reminders trigger at the same time.
- Add a demo GIF and clearer screenshots to the README.
- Add an example `reminders.json` file.

## v0.3.0 — Make configuration friendlier

Goal: reduce the need to manually edit JSON for common reminder changes.

- Add a simple config editor window.
- Support enabling and disabling individual reminders.
- Support duplicating a reminder.
- Support pausing all reminders for a period of time.
- Add basic validation for reminder time, date, and interval fields.
- Improve the first-run onboarding experience.

## v0.4.0 — Make it a better Tauri menubar starter

Goal: make MacaBell useful as a reference project for people building small macOS utilities.

- Document the Tauri tray, window, scheduler, and local config structure.
- Add comments around key Rust and frontend integration points.
- Add a small architecture diagram.
- Extract reusable helper modules where it makes sense.
- Add examples for building other menu bar tools from the same structure.

## Later ideas

These are possible directions, but they should stay secondary unless there is real user demand.

- Custom themes.
- More reminder repeat rules.
- Export and import config.
- Optional signed build.
- Optional automatic updates.
- A separate starter template for Tauri 2 menu bar apps.

## Non-goals

MacaBell should not become a heavy productivity platform.

The project does not plan to add:

- User accounts.
- Cloud sync.
- Team collaboration.
- Complex calendar replacement features.
- A full task management system.

The core value is a calm, local, easy-to-understand menu bar reminder app.
