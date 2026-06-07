# MacaBell · 马卡龙提醒

一个很小很可爱的 **Mac 菜单栏提醒小工具**。到点后，从屏幕右侧滑入一个温和的糖果色小弹窗，提醒你该休息、该做事，或者别忘了某个重要日子。

没有账号，没有云同步，没有复杂后台——配置就是一个本地 JSON 文件，简单、直接、够用。

![预览](assets/preview.png)

## 下载与安装

- 🍬 **官网**：https://yixiao-lab.github.io/MacaBell/
- ⬇️ **下载**：[最新版 .dmg](https://github.com/yixiao-lab/MacaBell/releases/latest)（仅 Apple Silicon / M 系列）

> 应用未做 Apple 付费签名，首次打开若提示「无法验证开发者」或「已损坏」：在「应用程序」里**右键点 MacaBell → 打开**，再点一次「打开」即可；仍不行就在终端执行一次 `xattr -dr com.apple.quarantine /Applications/MacaBell.app`。

## 特点

- 🍬 **马卡龙风格** — 6 套糖果配色每次随机，看着舒服、不焦虑、不打扰心流
- 🧭 **常驻菜单栏** — 没有主窗口，只在顶部菜单栏放一个小图标
- 🔕 **不抢焦点** — 弹窗不夺取键盘焦点、不自动消失，随手点掉即可
- 📂 **本地 JSON 配置** — 单一文件 `~/.macaron-reminder/reminders.json`，所见即所得
- 🔔 **三类提醒** — 每日定时 / 固定间隔 / 每年纪念日
- 🔊 **柔和提示音** — 带全局开关，可在托盘菜单里随时切换
- 🚀 **开机自启** — 一键开启，登录即常驻

## 三类提醒

| 类型 | 用途 | 例子 |
|------|------|------|
| `daily` | 每日固定时间点 | 每天 14:00 写周报 |
| `interval` | 固定间隔循环 | 每 60 分钟起身动动 |
| `anniversary` | 每年某月某日 | 每年 06-15 的纪念日 |

## 配置

首次启动会在 `~/.macaron-reminder/reminders.json` 生成默认配置，直接编辑即可：

```json
{
  "soundEnabled": true,
  "reminders": [
    { "id": "work-report", "type": "daily", "time": "14:00",
      "title": "工作提醒", "message": "该写周报啦" },
    { "id": "sit-too-long", "type": "interval", "everyMinutes": 60,
      "title": "久坐提醒", "message": "起来动动，喝口水吧" },
    { "id": "anniversary", "type": "anniversary", "date": "06-15", "time": "09:00",
      "title": "纪念日", "message": "今天是个特别的日子" }
  ]
}
```

改完无需重启，后台每 20 秒读取一次最新配置。

## 菜单栏

点击菜单栏的马卡龙图标：

- **测试提醒一下** — 立刻弹一条，确认效果
- **开启提醒声音** — 声音开关（写回配置文件）
- **开机自动启动** — 登录时自动常驻
- **退出**

## 技术栈

Tauri 2 + React + TypeScript。后端调度 / 托盘 / 窗口控制在 `src-tauri/src/lib.rs`，前端弹窗在 `src/App.tsx`。

对我来说，它同时也是一个 Tauri 菜单栏应用的小脚手架——以后想做 AI 快捷助手、定时通知工具、剪贴板工具，都可以基于它继续改。

## 开发

```bash
pnpm install
pnpm tauri dev      # 本地开发（热重载）
pnpm tauri build    # 打包成 .app / .dmg
```

> 改了 `src-tauri/icons/` 里的图标后，记得 `touch src-tauri/build.rs` 再重新编译，否则 cargo 不会重新嵌入图标。

## 初衷

我自己平时写代码、做项目，经常会忘记站起来活动，也会漏掉一些固定的小任务，所以想做一个看着舒服、不打扰心流的小工具。

小工具不一定要很大，能解决一个自己真实的问题，就已经有价值了。

## 反馈

这个工具还很小，正在靠反馈长大。用得不顺手、想要某类提醒、或发现 bug，欢迎到 [GitHub Issues](https://github.com/yixiao-lab/MacaBell/issues) 提，看到都会回。

## License

MIT
