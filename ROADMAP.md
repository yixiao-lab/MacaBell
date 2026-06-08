# Roadmap

MacaBell is starting as a small, local-first macOS menu bar reminder app. The long-term direction is a **local task inbox for AI coding workflows** — one calm local place where scripts, build tasks, and AI coding tools can drop events you actually want to see.

The project stays local-first, lightweight, privacy-friendly, and open source throughout.

## v0.2.0 — Make it easier to install and use

Goal: turn the working app into a polished open-source release.

- Universal macOS builds for both Apple Silicon and Intel Macs.
- GitHub Actions workflow for release builds.
- Menu item to open `~/.MacaBell/reminders.json` directly.
- Menu item to reveal the config folder in Finder.
- Friendly error message when the JSON config is invalid.
- Stacked reminders when multiple fire at the same time.
- Clearer screenshots and an example `reminders.json`.

## v0.3.0 — Local task notifications via CLI

Goal: let developer tools and scripts send events into MacaBell.

- Add a `macabell notify` command-line interface.
- Accept `--source`, `--project`, `--status`, `--title`, `--message` flags.
- Route CLI events into the same gentle popup system.
- Document how to wire it into Shell scripts and build steps.
- Keep everything local — no network, no account.

## v0.4.0 — Local task inbox

Goal: turn one-off notifications into a browsable local inbox.

- A simple task inbox window with event history.
- Task states (pending / done / failed).
- Lightweight actions: open a project, copy a message, dismiss.
- Filter and group events by source and project.
- Still a single local data file, still easy to understand.

## Future — AI coding workflow integrations

Goal: become the local "done" signal for long-running AI and dev tasks.

- Integrations for Codex, Claude Code, and Cursor.
- Hooks for Shell scripts, test runs, build commands, and deploys.
- These expand **only if real developers find them useful.**

## Later ideas

Secondary unless there is real user demand:

- Custom themes.
- More reminder repeat rules.
- Export and import config.
- Optional signed and notarized build.
- Optional automatic updates.
- A separate starter template for Tauri 2 menu bar apps.

## Non-goals

MacaBell should not become a heavy SaaS productivity platform. The project does not plan to add:

- User accounts.
- Cloud sync.
- Team collaboration.
- A full calendar replacement.

The core value stays the same: a calm, local, easy-to-understand tool — first for reminders, then as a local inbox for developer and AI coding workflows.
