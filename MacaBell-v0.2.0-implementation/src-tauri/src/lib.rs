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
    #[serde(default)]
    reminders: Vec<Reminder>,
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Serialize)]
struct ReminderPayload {
    id: String,
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

    // 配置错误提示去重：避免 JSON 写错后每 20 秒弹一次
    config_error_last: Mutex<Option<String>>,
}

// ---------- 配置文件读写 ----------

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".MacaBell")
        .join("reminders.json")
}

fn config_dir() -> PathBuf {
    config_path()
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
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

fn ensure_config_exists() -> Result<PathBuf, String> {
    let path = config_path();

    if path.exists() {
        return Ok(path);
    }

    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("无法创建配置目录：{e}"))?;
    }

    fs::write(&path, DEFAULT_CONFIG).map_err(|e| format!("无法写入默认配置：{e}"))?;
    Ok(path)
}

fn load_config_result() -> Result<Config, String> {
    let path = ensure_config_exists()?;
    let text = fs::read_to_string(&path).map_err(|e| format!("无法读取配置文件：{e}"))?;

    serde_json::from_str::<Config>(&text).map_err(|e| {
        format!(
            "配置文件格式不正确：{e}\n\n请检查 {}，修正 JSON 后会自动恢复。",
            path.display()
        )
    })
}

fn load_config() -> Config {
    load_config_result().unwrap_or_else(|e| {
        eprintln!("[macaron] reminders.json 解析失败，使用空配置: {e}");
        Config {
            sound_enabled: true,
            reminders: vec![],
        }
    })
}

// 把当前的声音开关写回配置文件（托盘切换时调用）
fn save_sound_enabled(enabled: bool) {
    let path = match ensure_config_exists() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("[macaron] 初始化配置失败，无法写入声音开关: {e}");
            return;
        }
    };

    if let Ok(text) = fs::read_to_string(&path) {
        if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&text) {
            value["soundEnabled"] = serde_json::Value::Bool(enabled);
            if let Ok(pretty) = serde_json::to_string_pretty(&value) {
                let _ = fs::write(&path, pretty);
            }
        }
    }
}

// ---------- macOS 打开配置 ----------

fn open_config_file() {
    match ensure_config_exists() {
        Ok(path) => {
            let _ = Command::new("open").arg(path).spawn();
        }
        Err(e) => eprintln!("[macaron] 打开配置文件失败: {e}"),
    }
}

fn reveal_config_in_finder() {
    match ensure_config_exists() {
        Ok(path) => {
            let _ = Command::new("open").arg("-R").arg(path).spawn();
        }
        Err(e) => {
            eprintln!("[macaron] 定位配置文件失败: {e}");
            let _ = Command::new("open").arg(config_dir()).spawn();
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

fn prepare_popup_window(app: &tauri::AppHandle) {
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
    }
}

fn emit_popup(app: &tauri::AppHandle, payload: ReminderPayload) {
    prepare_popup_window(app);
    let _ = app.emit("reminder", payload);
}

fn show_popup(app: &tauri::AppHandle, payload: ReminderPayload, sound_enabled: bool) {
    emit_popup(app, payload);

    // 声音以「调用方传入的当前配置」为准，保证 soundEnabled 改文件也即时生效
    if sound_enabled {
        play_sound();
    }
}

// ---------- 调度 ----------

fn should_fire_reminder(
    state: &AppState,
    reminder: &Reminder,
    now_minute: &str,
    now_hm: &str,
    now_md: &str,
) -> bool {
    let should_fire = match &reminder.trigger {
        Trigger::Daily { time } => time == now_hm,
        Trigger::Anniversary { date, time } => date == now_md && time == now_hm,
        Trigger::Interval { every_minutes } => {
            let interval_minutes = (*every_minutes).max(1);
            let mut last = state.interval_last.lock().unwrap();

            match last.get(&reminder.id) {
                Some(t) if t.elapsed() < Duration::from_secs(interval_minutes * 60) => false,
                _ => {
                    last.insert(reminder.id.clone(), Instant::now());
                    true
                }
            }
        }
    };

    if !should_fire {
        return false;
    }

    // daily / anniversary 同一分钟去重
    if !matches!(&reminder.trigger, Trigger::Interval { .. }) {
        let mut fired = state.fired_at.lock().unwrap();
        if fired
            .get(&reminder.id)
            .map(|last_minute| last_minute == now_minute)
            .unwrap_or(false)
        {
            return false;
        }
        fired.insert(reminder.id.clone(), now_minute.to_string());
    }

    true
}

fn spawn_scheduler(app: tauri::AppHandle) {
    thread::spawn(move || loop {
        let state = app.state::<AppState>();

        match load_config_result() {
            Ok(config) => {
                *state.sound_enabled.lock().unwrap() = config.sound_enabled;
                *state.config_error_last.lock().unwrap() = None;

                let now = Local::now();
                let now_minute = now.format("%Y-%m-%d %H:%M").to_string();
                let now_hm = format!("{:02}:{:02}", now.hour(), now.minute());
                let now_md = format!("{:02}-{:02}", now.month(), now.day());

                let mut due_payloads = Vec::new();

                for reminder in &config.reminders {
                    if should_fire_reminder(&state, reminder, &now_minute, &now_hm, &now_md) {
                        due_payloads.push(ReminderPayload {
                            id: reminder.id.clone(),
                            title: reminder.title.clone(),
                            message: reminder.message.clone(),
                        });
                    }
                }

                if !due_payloads.is_empty() {
                    for payload in due_payloads {
                        emit_popup(&app, payload);
                    }

                    if config.sound_enabled {
                        play_sound();
                    }
                }
            }
            Err(message) => {
                let mut last_error = state.config_error_last.lock().unwrap();
                let should_notify = last_error.as_ref() != Some(&message);

                if should_notify {
                    *last_error = Some(message.clone());
                    drop(last_error);

                    show_popup(
                        &app,
                        ReminderPayload {
                            id: "config-error".into(),
                            title: "配置文件有点问题".into(),
                            message,
                        },
                        true,
                    );
                }
            }
        }

        thread::sleep(Duration::from_secs(20));
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
            config_error_last: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![hide_popup])
        .setup(|app| {
            let handle = app.handle().clone();

            // 托盘菜单：测试提醒 / 配置文件 / 声音开关 / 开机自启 / 退出
            let test_item = MenuItemBuilder::with_id("test", "测试提醒一下").build(app)?;
            let open_config_item = MenuItemBuilder::with_id("open-config", "打开配置文件").build(app)?;
            let reveal_config_item =
                MenuItemBuilder::with_id("reveal-config", "在 Finder 中显示配置").build(app)?;

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
                .separator()
                .item(&open_config_item)
                .item(&reveal_config_item)
                .separator()
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
                                id: "test".into(),
                                title: "测试提醒".into(),
                                message: "这是一条测试，弹窗工作正常～".into(),
                            },
                            enabled,
                        );
                    }
                    "open-config" => open_config_file(),
                    "reveal-config" => reveal_config_in_finder(),
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
