// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // `macabell notify ...` 走 CLI 分支：写入本地事件文件后直接退出，不启动 GUI
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "notify" {
        std::process::exit(macaron_reminder_lib::cli::run_notify(&args[2..]));
    }

    macaron_reminder_lib::run()
}
