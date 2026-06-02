// 马卡龙提醒 —— Tauri 后端：配置加载 / 定时调度 / 托盘 / 声音 / 窗口控制
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Emitter, Manager, PhysicalPosition,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

// ---------- 数据模型 ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Trigger {
    // 每天固定时间点，如 "14:00"
    Daily { time: String },
    // 固定间隔循环，单位分钟
    Interval {
        #[serde(rename = "everyMinutes")]
        every_minutes: u64,
    },
    // 每年某月某日 + 时间点，date 形如 "06-15"
    Anniversary { date: String, time: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Reminder {
    id: String,
    title: String,
    message: String,
    #[serde(flatten)]
    trigger: Trigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "soundEnabled", default = "default_true")]
    sound_enabled: bool,
    reminders: Vec<Reminder>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Serialize)]
struct ReminderPayload {
    title: String,
    message: String,
}

// ---------- 运行时状态 ----------

struct AppState {
    sound_enabled: Mutex<bool>,
    // daily / anniversary 去重：id -> 最近一次触发的 "YYYY-MM-DD HH:MM"
    fired_at: Mutex<HashMap<String, String>>,
    // interval 计时：id -> 上次触发时刻
    interval_last: Mutex<HashMap<String, Instant>>,
}

// ---------- 配置文件读写 ----------

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".macaron-reminder").join("reminders.json")
}

const DEFAULT_CONFIG: &str = r#"{
  "soundEnabled": true,
  "reminders": [
    {
      "id": "work-report",
      "type": "daily",
      "time": "14:00",
      "title": "工作提醒",
      "message": "该写周报啦"
    },
    {
      "id": "sit-too-long",
      "type": "interval",
      "everyMinutes": 60,
      "title": "久坐提醒",
      "message": "起来动动，喝口水吧"
    },
    {
      "id": "anniversary",
      "type": "anniversary",
      "date": "06-15",
      "time": "09:00",
      "title": "纪念日",
      "message": "今天是个特别的日子"
    }
  ]
}
"#;

fn load_config() -> Config {
    let path = config_path();
    if !path.exists() {
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = fs::write(&path, DEFAULT_CONFIG);
    }
    match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("[macaron] reminders.json 解析失败，使用空配置: {e}");
            Config {
                sound_enabled: true,
                reminders: vec![],
            }
        }),
        Err(_) => Config {
            sound_enabled: true,
            reminders: vec![],
        },
    }
}

// 把当前的声音开关写回配置文件（托盘切换时调用）
fn save_sound_enabled(enabled: bool) {
    let path = config_path();
    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&text) {
            value["soundEnabled"] = serde_json::Value::Bool(enabled);
            if let Ok(pretty) = serde_json::to_string_pretty(&value) {
                let _ = fs::write(&path, pretty);
            }
        }
    }
}

// ---------- 声音 ----------

fn play_sound() {
    // 用 macOS 自带柔和提示音，spawn 不阻塞
    let _ = Command::new("afplay")
        .arg("/System/Library/Sounds/Glass.aiff")
        .spawn();
}

// ---------- 弹窗显示 ----------

fn show_popup(app: &tauri::AppHandle, payload: ReminderPayload, sound_enabled: bool) {
    if let Some(win) = app.get_webview_window("main") {
        // 定位到主屏右上角
        if let Ok(Some(monitor)) = app.primary_monitor() {
            let screen = monitor.size();
            if let Ok(win_size) = win.outer_size() {
                let scale = monitor.scale_factor();
                let margin = (20.0 * scale) as i32;
                let top = (44.0 * scale) as i32;
                let x = screen.width as i32 - win_size.width as i32 - margin;
                let _ = win.set_position(PhysicalPosition::new(x, top));
            }
        }
        let _ = win.show();
        // 推送内容给前端做滑入动画
        let _ = app.emit("reminder", payload);
    }

    // 声音以「调用方传入的当前配置」为准，保证 soundEnabled 改文件也即时生效
    if sound_enabled {
        play_sound();
    }
}

// ---------- 调度 ----------

fn spawn_scheduler(app: tauri::AppHandle) {
    thread::spawn(move || {
        loop {
            let config = load_config();
            let now = Local::now();
            let now_minute = now.format("%Y-%m-%d %H:%M").to_string();
            let now_hm = format!("{:02}:{:02}", now.hour(), now.minute());
            let now_md = format!("{:02}-{:02}", now.month(), now.day());

            let state = app.state::<AppState>();

            for r in &config.reminders {
                let should_fire = match &r.trigger {
                    Trigger::Daily { time } => time == &now_hm,
                    Trigger::Anniversary { date, time } => {
                        date == &now_md && time == &now_hm
                    }
                    Trigger::Interval { every_minutes } => {
                        let mut last = state.interval_last.lock().unwrap();
                        match last.get(&r.id) {
                            Some(t) if t.elapsed() < Duration::from_secs(every_minutes * 60) => {
                                false
                            }
                            _ => {
                                last.insert(r.id.clone(), Instant::now());
                                true
                            }
                        }
                    }
                };

                if !should_fire {
                    continue;
                }

                // daily / anniversary 同一分钟去重
                if !matches!(r.trigger, Trigger::Interval { .. }) {
                    let mut fired = state.fired_at.lock().unwrap();
                    if fired.get(&r.id) == Some(&now_minute) {
                        continue;
                    }
                    fired.insert(r.id.clone(), now_minute.clone());
                }

                show_popup(
                    &app,
                    ReminderPayload {
                        title: r.title.clone(),
                        message: r.message.clone(),
                    },
                    config.sound_enabled,
                );
            }

            thread::sleep(Duration::from_secs(20));
        }
    });
}

// ---------- 前端可调用命令 ----------

#[tauri::command]
fn hide_popup(window: tauri::WebviewWindow) {
    let _ = window.hide();
}

// ---------- 入口 ----------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = load_config();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(AppState {
            sound_enabled: Mutex::new(config.sound_enabled),
            fired_at: Mutex::new(HashMap::new()),
            interval_last: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![hide_popup])
        .setup(|app| {
            let handle = app.handle().clone();

            // 托盘菜单：测试提醒 / 声音开关 / 退出
            let test_item = MenuItemBuilder::with_id("test", "测试提醒一下").build(app)?;
            let initial_sound = *app.state::<AppState>().sound_enabled.lock().unwrap();
            let sound_item = CheckMenuItemBuilder::with_id("sound", "开启提醒声音")
                .checked(initial_sound)
                .build(app)?;
            let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
            let autostart_item = CheckMenuItemBuilder::with_id("autostart", "开机自动启动")
                .checked(autostart_enabled)
                .build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&test_item)
                .item(&sound_item)
                .item(&autostart_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let sound_item_for_event = sound_item.clone();
            let autostart_item_for_event = autostart_item.clone();
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("马卡龙提醒")
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "test" => {
                        // 以文件为准读当前声音设置，保持单一事实源
                        let enabled = load_config().sound_enabled;
                        show_popup(
                            app,
                            ReminderPayload {
                                title: "测试提醒".into(),
                                message: "这是一条测试，弹窗工作正常～".into(),
                            },
                            enabled,
                        );
                    }
                    "sound" => {
                        // 基于文件当前值翻转，文件始终是单一事实源
                        let next = !load_config().sound_enabled;
                        save_sound_enabled(next);
                        *app.state::<AppState>().sound_enabled.lock().unwrap() = next;
                        let _ = sound_item_for_event.set_checked(next);
                    }
                    "autostart" => {
                        let mgr = app.autolaunch();
                        let enabled = mgr.is_enabled().unwrap_or(false);
                        let _ = if enabled { mgr.disable() } else { mgr.enable() };
                        let _ = autostart_item_for_event.set_checked(!enabled);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .build(app)?;

            spawn_scheduler(handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
