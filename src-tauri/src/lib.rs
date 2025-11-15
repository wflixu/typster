// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

mod cmd;
// mod ipc;
// mod project;
// mod cmds;
mod typst_service;
// mod state;
mod util;
mod config;

// use crate::project::ProjectManager;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use tauri::Wry;
// use state::AppState;

pub fn run() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    info!("initializing typster");

    // let appstate = Arc::new(AppState::<Wry>::new());
  
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // .manage(appstate)
        .invoke_handler(tauri::generate_handler![
            cmd::greet,
            // cmds::greet2,
            // cmds::load_doc_from_path,
            // ipc::commands::fs_list_dir,
            // ipc::commands::fs_read_file_binary,
            // ipc::commands::fs_read_file_text,
            // ipc::commands::fs_create_file,
            // ipc::commands::fs_write_file_binary,
            // ipc::commands::fs_write_file_text,
            // ipc::commands::load_project_from_path,
            // ipc::commands::typst_compile_doc,
            // ipc::commands::typst_render,
            // ipc::commands::typst_autocomplete,
            // ipc::commands::typst_slot_update,
            // ipc::commands::export_pdf,
            // ipc::commands::clipboard_paste
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
