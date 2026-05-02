//! 声明项目内部的模块
mod commands;
mod config;
mod download_manager;
mod errors;
mod events;
mod export;
mod extensions;
mod hitomi;
mod hitomi_client;
mod logger;
mod types;
mod utils;

use anyhow::Context;
use config::Config;
use download_manager::DownloadManager;
use events::{DownloadSpeedEvent, DownloadTaskEvent, ExportCbzEvent, ExportPdfEvent, LogEvent};
use hitomi_client::HitomiClient;
use parking_lot::RwLock;
use tauri::{Manager, Wry};

use crate::commands::*;

// 生成 Tauri 的上下文环境
fn generate_context() -> tauri::Context<Wry> {
    tauri::generate_context!()
}

// 定义 Tauri 应用程序的入口函数
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化 tauri_specta，用于前后端类型安全的通信
    let builder = tauri_specta::Builder::<Wry>::new()
        // 注册所有的命令，前端可以调用这些函数
        .commands(tauri_specta::collect_commands![
            greet,
            get_config,
            save_config,
            search,
            get_page,
            get_comic,
            create_download_task,
            pause_download_task,
            resume_download_task,
            cancel_download_task,
            get_downloaded_comics,
            export_pdf,
            export_cbz,
            get_search_suggestions,
            get_logs_dir_size,
            show_path_in_file_manager,
            get_cover_data,
            get_synced_comic,
            get_image_data,
        ])
        // 注册所有的事件，前端可以监听这些事件
        .events(tauri_specta::collect_events![
            LogEvent,
            DownloadTaskEvent,
            DownloadSpeedEvent,
            ExportPdfEvent,
            ExportCbzEvent,
        ]);

    // 在开发（调试）模式下，自动导出 TypeScript 绑定文件
    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                // 将 Rust 的 BigInt 映射为 TS 的 Number
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                // 使用 prettier 格式化输出代码
                .formatter(specta_typescript::formatter::prettier)
                .header("// @ts-nocheck"), // disable typescript checks
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    // 构建并运行 Tauri 应用
    tauri::Builder::default()
        // 初始化 dialog 插件，用于原生的文件/目录选择框等
        .plugin(tauri_plugin_dialog::init())
        // 初始化 opener 插件，用于在系统默认应用中打开链接或文件
        .plugin(tauri_plugin_opener::init())
        // 挂载由 specta 生成的命令处理器
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // 将全局的 app handle 保存起来以备后用
            utils::APP_HANDLE.get_or_init(|| app.handle().clone());

            // 挂载事件，这样后端就可以发送强类型的事件给前端
            builder.mount_events(app);

            // 获取应用的数据目录
            let app_data_dir = app
                .path()
                .app_data_dir()
                .context("get app_data_dir failed")?;

            // 确保应用数据目录存在，如果不存在则创建
            std::fs::create_dir_all(&app_data_dir).context(format!(
                "create app_data_dir `{}` failed",
                app_data_dir.display()
            ))?;

            // 初始化应用配置并交由 Tauri 的状态管理
            let config = RwLock::new(Config::new(app.handle())?);
            app.manage(config);

            // 初始化 Hitomi 客户端并交由 Tauri 的状态管理
            let hitomi_client = HitomiClient::new(app.handle().clone());
            app.manage(hitomi_client);

            // 初始化下载管理器并交由 Tauri 的状态管理
            let download_manager = DownloadManager::new(app.handle());
            app.manage(download_manager);

            // 初始化日志系统
            logger::init(app.handle())?;

            Ok(())
        })
        .run(generate_context())
        .expect("error while running tauri application");
}
pub mod tags;
