# Demo Assets

Recommended assets for the README:

```text
assets/screenshots/menu.png
assets/screenshots/popup-single.png
assets/screenshots/popup-stack.png
assets/demo.gif
```

Suggested recording flow:

1. Start MacaBell locally with `pnpm tauri dev`.
2. Click the menu bar icon.
3. Trigger “测试提醒一下”.
4. Add two reminders with the same time in `~/.MacaBell/reminders.json`.
5. Wait for the stacked reminders to appear.
6. Record a short GIF and place it at `assets/demo.gif`.

The app-side implementation for stacked reminders is already included in v0.2.0. The actual images need to be captured on a running macOS environment.
