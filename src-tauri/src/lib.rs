// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

mod cmd;
mod engine;
mod ipc;
mod project;

use crate::project::ProjectManager;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use tauri::Wry;

pub fn run() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    info!("initializing typstudio");

    let project_manager = Arc::new(ProjectManager::<Wry>::new());
    if let Ok(watcher) = ProjectManager::init_watcher(project_manager.clone()) {
        project_manager.set_watcher(watcher);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(project_manager)
        .invoke_handler(tauri::generate_handler![
            cmd::greet,
            ipc::commands::fs_list_dir,
            ipc::commands::fs_read_file_binary,
            ipc::commands::fs_read_file_text,
            ipc::commands::fs_create_file,
            ipc::commands::fs_write_file_binary,
            ipc::commands::fs_write_file_text,
            ipc::commands::load_project_from_path,
            ipc::commands::typst_compile_doc,
            ipc::commands::typst_render,
            ipc::commands::typst_autocomplete,
            ipc::commands::typst_slot_update,
            ipc::commands::export_pdf,
            ipc::commands::clipboard_paste
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
