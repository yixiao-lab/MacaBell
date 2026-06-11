// macabell notify 子命令：把本地任务事件写入 ~/.MacaBell/events/，由运行中的 app 消费弹窗

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub project: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub created_at: String,
}

pub fn events_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".MacaBell").join("events")
}

const USAGE: &str = "\
macabell notify —— 向 MacaBell 发送一条本地任务通知

用法:
  macabell notify --message \"构建完成\" [选项]

选项:
  --source <名称>    事件来源，如 claude-code、codex、ci
  --project <名称>   所属项目
  --status <状态>    done / failed / pending（影响弹窗图标）
  --title <标题>     弹窗标题，缺省时由 source 和 project 拼出
  --message <内容>   弹窗正文（必填）
  --help             显示本帮助

事件只写入本地 ~/.MacaBell/events/，不经过任何网络。
MacaBell 在运行时会立即弹出通知。";

// 解析 `notify` 之后的参数并落盘事件文件。返回进程退出码。
pub fn run_notify(args: &[String]) -> i32 {
    let mut event = TaskEvent {
        source: String::new(),
        project: String::new(),
        status: String::new(),
        title: String::new(),
        message: String::new(),
        created_at: String::new(),
    };

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let value = |iter: &mut std::slice::Iter<String>| -> Result<String, String> {
            iter.next()
                .map(|v| v.to_string())
                .ok_or_else(|| format!("{arg} 后面缺少值"))
        };

        let result = match arg.as_str() {
            "--source" => value(&mut iter).map(|v| event.source = v),
            "--project" => value(&mut iter).map(|v| event.project = v),
            "--status" => value(&mut iter).map(|v| event.status = v),
            "--title" => value(&mut iter).map(|v| event.title = v),
            "--message" => value(&mut iter).map(|v| event.message = v),
            "--help" | "-h" => {
                println!("{USAGE}");
                return 0;
            }
            other => Err(format!("不认识的参数：{other}")),
        };

        if let Err(e) = result {
            eprintln!("macabell notify: {e}\n\n{USAGE}");
            return 2;
        }
    }

    if event.message.is_empty() && event.title.is_empty() {
        eprintln!("macabell notify: 至少需要 --message 或 --title\n\n{USAGE}");
        return 2;
    }

    event.created_at = chrono::Local::now().to_rfc3339();

    match write_event(&event) {
        Ok(path) => {
            println!("已写入事件：{}", path.display());
            0
        }
        Err(e) => {
            eprintln!("macabell notify: {e}");
            1
        }
    }
}

// 先写临时文件再 rename，保证 app 轮询时不会读到半个 JSON
fn write_event(event: &TaskEvent) -> Result<PathBuf, String> {
    let dir = events_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("无法创建事件目录 {}：{e}", dir.display()))?;

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let filename = format!("{}-{}.json", nanos, std::process::id());

    let json = serde_json::to_string_pretty(event).map_err(|e| format!("序列化事件失败：{e}"))?;

    let tmp_path = dir.join(format!(".{filename}.tmp"));
    let final_path = dir.join(filename);

    fs::write(&tmp_path, json).map_err(|e| format!("写入事件文件失败：{e}"))?;
    fs::rename(&tmp_path, &final_path).map_err(|e| format!("移动事件文件失败：{e}"))?;

    Ok(final_path)
}
