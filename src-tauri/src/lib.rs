// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

// mod cmd;
// mod ipc;
// mod project;
mod cmds;
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
            cmds::doc::load_doc_from_path,
            cmds::fs::fs_list_dir,
            cmds::fs::fs_read_file_binary,
            cmds::fs::fs_read_file_text,
            cmds::fs::fs_create_file,
            cmds::fs::fs_write_file_binary,
            cmds::fs::fs_write_file_text,
            // cmds::fs::load_project_from_path,
            cmds::typst::typst_compile_doc,
            cmds::typst::typst_render,
            cmds::typst::typst_autocomplete,
            cmds::typst::typst_slot_update,
            cmds::typst::export_pdf,
            // cmds::typst::clipboard_paste
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
