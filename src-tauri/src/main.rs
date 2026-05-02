// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// 在 Windows 的 release 构建中防止出现额外的命令行窗口，请勿移除此行！！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 启动 hitomi_downloader_lib 中定义的 Tauri 应用程序
    hitomi_downloader_lib::run()
}
