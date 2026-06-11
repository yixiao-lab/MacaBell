# MacaBell CLI

`macabell notify` lets scripts, build steps, and AI coding tools drop a local task notification into MacaBell. While the app is running, a pastel popup appears within a second.

Everything stays on your machine: each event is a small JSON file written to `~/.MacaBell/events/`, which the running app consumes and deletes. No network, no account, no daemon beyond the app itself.

## Setup

The CLI lives inside the app binary. Link it once:

```bash
sudo ln -sf /Applications/MacaBell.app/Contents/MacOS/MacaBell /usr/local/bin/macabell
```

Or, if you prefer not to touch `/usr/local/bin`, add an alias to your shell profile:

```bash
alias macabell="/Applications/MacaBell.app/Contents/MacOS/MacaBell"
```

## Usage

```bash
macabell notify --message "Build finished" [options]
```

| Flag | Description |
| --- | --- |
| `--message <text>` | Popup body text. Required unless `--title` is given. |
| `--title <text>` | Popup title. Defaults to `source · project`, or `任务通知` if both are empty. |
| `--source <name>` | Where the event came from, e.g. `claude-code`, `codex`, `ci`. |
| `--project <name>` | Project the event belongs to. |
| `--status <state>` | `done` / `failed` / `pending` — prefixes the message with ✅ / ❌ / ⏳. |
| `--help` | Show usage. |

Exit codes: `0` event written, `1` write failed, `2` bad arguments.

## Recipes

### Shell script / build step

```bash
pnpm build \
  && macabell notify --source ci --project my-app --status done --message "Build passed" \
  || macabell notify --source ci --project my-app --status failed --message "Build failed"
```

### Long-running command

```bash
cargo test; macabell notify --source shell --project my-app \
  --status $([ $? -eq 0 ] && echo done || echo failed) --message "cargo test finished"
```

### Claude Code Stop hook

In `.claude/settings.json`:

```json
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "macabell notify --source claude-code --project my-app --status done --message 'Claude Code finished a task'"
          }
        ]
      }
    ]
  }
}
```

## How it works

1. `macabell notify` writes the event as JSON into `~/.MacaBell/events/` (written to a temp file first, then renamed, so the app never reads a half-written file).
2. The running MacaBell app polls that folder every second.
3. Each event becomes a popup card; the file is deleted after it is consumed.

If MacaBell is not running when an event is sent, the event waits in the folder and pops up the next time the app starts.

The event format is plain JSON, so anything that can write a file can integrate:

```json
{
  "source": "codex",
  "project": "my-app",
  "status": "done",
  "title": "",
  "message": "Build finished",
  "createdAt": "2026-06-11T20:38:33+08:00"
}
```
