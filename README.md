# MacaBell 🌈

<div align="center">

<img src="./assets/screenshots/popup-stack.png" alt="MacaBell preview" width="760" />

<br />

**A soft pastel macOS menu bar reminder app built with Tauri 2, React, TypeScript, and Rust.**

MacaBell helps you remember small things right from your menu bar today — and is gradually evolving toward a local task inbox for AI coding workflows.

<br />

<a href="https://github.com/yixiao-lab/MacaBell/releases/latest">
  <img src="https://img.shields.io/github/v/release/yixiao-lab/MacaBell?style=for-the-badge&label=Download&color=ff8fab" alt="Latest Release" />
</a>
<a href="https://github.com/yixiao-lab/MacaBell/stargazers">
  <img src="https://img.shields.io/github/stars/yixiao-lab/MacaBell?style=for-the-badge&color=fbbf24" alt="GitHub Stars" />
</a>
<a href="https://github.com/yixiao-lab/MacaBell/blob/main/LICENSE">
  <img src="https://img.shields.io/github/license/yixiao-lab/MacaBell?style=for-the-badge&color=8b5cf6" alt="License" />
</a>
<a href="https://github.com/yixiao-lab/MacaBell/releases">
  <img src="https://img.shields.io/github/downloads/yixiao-lab/MacaBell/total?style=for-the-badge&color=38bdf8" alt="Downloads" />
</a>

<br />
<br />

<img src="https://img.shields.io/badge/macOS-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS" />
<img src="https://img.shields.io/badge/Tauri_2-24C8DB?style=flat-square&logo=tauri&logoColor=white" alt="Tauri 2" />
<img src="https://img.shields.io/badge/React-20232A?style=flat-square&logo=react&logoColor=61DAFB" alt="React" />
<img src="https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white" alt="TypeScript" />
<img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" />

<br />
<br />

<a href="https://yixiao-lab.github.io/MacaBell/"><strong>Website</strong></a>
&nbsp;&nbsp;·&nbsp;&nbsp;
<a href="https://github.com/yixiao-lab/MacaBell/releases/latest"><strong>Download</strong></a>
&nbsp;&nbsp;·&nbsp;&nbsp;
<a href="#quick-start"><strong>Quick Start</strong></a>
&nbsp;&nbsp;·&nbsp;&nbsp;
<a href="#roadmap"><strong>Roadmap</strong></a>
&nbsp;&nbsp;·&nbsp;&nbsp;
<a href="#contributing"><strong>Contributing</strong></a>

<br />
<br />

**English** &nbsp;·&nbsp; <a href="./README.zh-CN.md">简体中文</a>

</div>

<br />

## What is MacaBell?

MacaBell is a tiny, local-first macOS menu bar reminder app.

When a reminder fires, a gentle candy-colored popup slides in from the top-right corner of your screen — reminding you to rest, to do something, or to not forget an important day.

No accounts. No cloud sync. No background platform. Your whole configuration is a single local JSON file — simple, direct, and enough.

<br />

## Why MacaBell?

Most reminder apps slowly grow into heavy productivity systems: accounts, cloud sync, collaboration, subscriptions, complex workflows.

MacaBell takes the smaller path.

It focuses on a calm local menu bar experience, pleasant popups, three honest reminder types, and a codebase small enough that any developer can read it, fork it, and turn it into their own Tauri menu bar utility.

<br />

## Preview

When a reminder fires, a candy-colored popup slides in from the top-right corner. When several reminders fire at once, they stack neatly instead of overlapping — and each one can be dismissed on its own.

<table>
  <tr>
    <td width="50%">
      <img src="./assets/screenshots/popup-single.png" alt="MacaBell single reminder" width="100%" />
      <p align="center"><strong>Single Reminder</strong></p>
    </td>
    <td width="50%">
      <img src="./assets/screenshots/popup-stack.png" alt="MacaBell stacked reminders" width="100%" />
      <p align="center"><strong>Stacked Reminders</strong></p>
    </td>
  </tr>
</table>

<br />

## Features

- **Macaron style** — 6 candy color schemes picked at random, easy on the eyes, never anxious, never breaks your flow.
- **Lives in the menu bar** — no main window, just a small icon at the top of your screen.
- **Never steals focus** — popups don't grab the keyboard; click them away whenever you like.
- **Stacked reminders** — multiple reminders at the same time never overlap.
- **Local JSON config** — one single file at `~/.MacaBell/reminders.json`, what you see is what you get.
- **Friendly error hints** — if the JSON is broken, MacaBell tells you gently and recovers once you fix it.
- **Three reminder types** — daily time, fixed interval, yearly anniversary.
- **Soft sound** — with a global toggle you can switch from the tray menu anytime.
- **Launch at login** — one click to keep it running from startup.
- **Universal DMG** — runs on both Apple Silicon and Intel Macs.

<br />

## Installation

Download the latest macOS `.dmg` from the release page:

**https://github.com/yixiao-lab/MacaBell/releases/latest**

Drag MacaBell into your Applications folder and launch it.

### First launch on macOS

MacaBell is distributed as an unsigned open-source build (no paid Apple notarization yet).

If macOS shows *"cannot verify the developer"* or *"the app is damaged"* on first launch:

1. In **Applications**, right-click MacaBell → **Open**, then click **Open** again.
2. If that still fails, run this once in Terminal:

```bash
xattr -dr com.apple.quarantine /Applications/MacaBell.app
```

A signed and notarized build may be explored later if there is enough demand.

<br />

## Quick Start

### 1. Launch MacaBell

After installation, MacaBell appears in your menu bar. The first launch creates a default config at `~/.MacaBell/reminders.json`.

### 2. Open the configuration file

From the tray menu:

```text
MacaBell → Open Configuration File
```

Or edit it directly:

```bash
~/.MacaBell/reminders.json
```

### 3. Edit your reminders

```json
{
  "soundEnabled": true,
  "reminders": [
    {
      "id": "work-report",
      "type": "daily",
      "time": "14:00",
      "title": "Weekly Report",
      "message": "Time to write your weekly report."
    },
    {
      "id": "drink-water",
      "type": "interval",
      "everyMinutes": 60,
      "title": "Hydrate",
      "message": "Drink some water and relax your shoulders."
    },
    {
      "id": "anniversary",
      "type": "anniversary",
      "date": "06-15",
      "time": "09:00",
      "title": "Anniversary",
      "message": "Today is a special day."
    }
  ]
}
```

Save the file — no restart needed. MacaBell reloads the latest config in the background every 20 seconds.

More examples are in [`examples/reminders.json`](examples/reminders.json).

<br />

## Reminder Configuration

MacaBell supports three reminder types.

| Type | Use case | Key fields |
| --- | --- | --- |
| `daily` | Same time every day | `time` (`"14:00"`) |
| `interval` | Repeat every N minutes | `everyMinutes` (`60`) |
| `anniversary` | A date every year | `date` (`"06-15"`), optional `time` |

### Daily

```json
{
  "id": "morning-exercise",
  "type": "daily",
  "time": "08:00",
  "title": "Morning Exercise",
  "message": "Time for a quick workout."
}
```

### Interval

```json
{
  "id": "stand-up",
  "type": "interval",
  "everyMinutes": 90,
  "title": "Stand Up",
  "message": "Get up and walk around for a minute."
}
```

### Anniversary

```json
{
  "id": "birthday",
  "type": "anniversary",
  "date": "12-25",
  "time": "09:00",
  "title": "Birthday",
  "message": "Remember the birthday today."
}
```

<br />

## Tray Menu

Click the macaron icon in the menu bar:

- **Test a reminder** — fire one instantly to check the effect.
- **Open Configuration File** — open `reminders.json` directly.
- **Reveal Config in Finder** — locate the config file.
- **Enable Sound** — sound toggle, written back to the config file.
- **Launch at Login** — keep it running from startup.
- **Quit**

<br />

## Development

### Requirements

Node.js, pnpm, Rust, and the Tauri prerequisites for macOS.

```bash
pnpm install
pnpm tauri dev
pnpm tauri build
```

Build a Universal macOS DMG locally:

```bash
./scripts/build-universal-macos.sh
```

> After changing icons in `src-tauri/icons/`, run `touch src-tauri/build.rs` before rebuilding, otherwise cargo won't re-embed the icons.

<br />

## Release

Push a tag and GitHub Actions builds the Universal macOS DMG and creates a draft release:

```bash
git tag v0.2.0
git push origin v0.2.0
```

See [`docs/RELEASE.md`](docs/RELEASE.md) for details.

<br />

## Project Structure

```text
MacaBell
├── src/                  React popup UI (App.tsx)
├── src-tauri/            Tauri, Rust, tray, scheduler, app config (lib.rs)
├── examples/             Example reminder configurations
├── assets/               Screenshots and visual assets
├── docs/                 Website and release notes
└── .github/workflows/    GitHub Actions release workflow
```

<br />

MacaBell is starting as a lightweight reminder app, and the long-term direction is a local task inbox for developer workflows.

<table>
  <tr>
    <td width="22%"><strong>v0.2</strong></td>
    <td>Universal macOS builds, GitHub Actions release packaging, example configs, friendly error handling, stacked reminders, and easier access to the config file.</td>
  </tr>
  <tr>
    <td><strong>v0.3</strong></td>
    <td>Explore local task notifications through a command-line interface such as <code>macabell notify</code>, letting Shell scripts and developer tools send events into MacaBell.</td>
  </tr>
  <tr>
    <td><strong>v0.4</strong></td>
    <td>Explore a local task inbox for long-running developer workflows — with event history, task states, and simple actions such as opening a project or copying a message.</td>
  </tr>
  <tr>
    <td><strong>Future</strong></td>
    <td>Explore integrations for AI coding workflows such as Codex, Claude Code, Cursor, Shell scripts, build tasks, and deployment tasks — only if real developers find it useful.</td>
  </tr>
</table>

The project will stay local-first, lightweight, privacy-friendly, and open source.

<br />

## Future Direction

MacaBell is useful today as a small reminder app.

The larger idea is to make it a **local task inbox for AI coding workflows** and long-running developer tasks. Future versions may let tools and scripts send messages into one local place:

```bash
macabell notify \
  --source codex \
  --project MacaBell \
  --status done \
  --title "Task completed" \
  --message "The implementation finished successfully."
```

Potential event sources include Codex, Claude Code, Cursor, Shell scripts, test commands, build commands, and deployment scripts.

This direction will only be expanded if real developers find it useful. MacaBell stays local-first throughout: no accounts, no cloud sync, no team collaboration.

<br />

## Built With

| | |
| --- | --- |
| **Tauri 2** | Native desktop shell and macOS integration. |
| **React** | Reminder popup interface. |
| **TypeScript** | Frontend application logic. |
| **Rust** | Tray menu, scheduling, local file operations, native behavior. |

<br />

## Contributing

Contributions, issues, ideas, and feedback are welcome — bug fixes, documentation, packaging improvements, example configs, and small usability tweaks that keep the app lightweight.

Before opening a pull request, please read [`CONTRIBUTING.md`](CONTRIBUTING.md).

<br />

## Support

If MacaBell helps you, or gives you a useful starting point for your own Tauri menu bar app:

- ⭐ **Star** the repo to help more developers find it.
- 🐛 **Open an issue** if you hit a bug or confusing behavior.
- 💡 **Suggest** a focused improvement.
- 🔁 **Share** it with someone building macOS menu bar utilities.

<br />

## Star History

<a href="https://www.star-history.com/#yixiao-lab/MacaBell&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=yixiao-lab/MacaBell&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=yixiao-lab/MacaBell&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=yixiao-lab/MacaBell&type=Date" />
  </picture>
</a>

<br />

## License

Released under the **MIT License**. See [`LICENSE`](LICENSE) for details.

<br />

## Author

Built by **Yixiao Lab**.

- GitHub: https://github.com/yixiao-lab
- Project: https://github.com/yixiao-lab/MacaBell
