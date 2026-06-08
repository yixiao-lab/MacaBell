# Contributing to MacaBell

Thanks for taking an interest in MacaBell.

MacaBell is a small macOS menu bar reminder app built with Tauri 2, React, TypeScript, and Rust. The project is intentionally simple: no account, no cloud sync, no backend, and no heavy productivity system. The goal is to keep it soft, lightweight, local-first, and easy to modify.

## What kind of contributions are welcome?

Good contributions usually fall into one of these areas:

- Bug fixes that make the app more stable.
- Small usability improvements that reduce friction.
- Better documentation, screenshots, examples, or install notes.
- macOS packaging improvements, such as universal builds, signing notes, and release automation.
- Reminder improvements that keep the app simple and local-first.
- Refactors that make the Tauri menubar app structure easier to understand.

Please avoid large feature additions before opening an issue first. MacaBell should stay small and calm rather than becoming a full task management system.

## Local development

Make sure you have Node.js, pnpm, Rust, and the Tauri prerequisites installed.

```bash
pnpm install
pnpm tauri dev
```

To build the app locally:

```bash
pnpm tauri build
```

## Project structure

```text
src/                 React popup UI
src-tauri/           Tauri, Rust, tray, scheduler, window control
public/              Static assets
assets/              App preview and visual materials
docs/                Project documentation
```

The main app logic currently lives in:

```text
src-tauri/src/lib.rs
src/App.tsx
```

## Before opening a pull request

Please check the following before submitting:

- The app can run with `pnpm tauri dev`.
- The changed behavior is described clearly.
- UI changes include screenshots or a short screen recording when possible.
- The contribution keeps the app local-first and lightweight.
- The pull request focuses on one clear change.

## Opening an issue

When reporting a bug, please include:

- macOS version.
- Mac model, especially Intel or Apple Silicon.
- MacaBell version.
- What you expected to happen.
- What actually happened.
- Steps to reproduce the issue.
- Screenshots or logs if available.

## Design principles

MacaBell should feel:

- Small.
- Gentle.
- Local-first.
- Easy to understand.
- Easy to fork into another macOS menu bar utility.

If a feature makes the app powerful but harder to trust, configure, or maintain, it probably needs more discussion before implementation.

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.
