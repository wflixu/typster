use log::{info, warn};
use std::{path::PathBuf, sync::Arc};
use tauri::{Runtime, State, Window};

use crate::cmds::{TypstPage, TypstSourceDiagnostic};
use crate::config::ConfigLoader;
// use crate::state::AppState;
use crate::typst_service::SystemWorld;
use crate::util::AppError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet2(name: &str) -> String {
    // 测试配置系统集成
    let loader = ConfigLoader::new();

    // 尝试加载全局配置
    match loader.load_config(None) {
        Ok(runtime_config) => {
            format!("Hello, {}! Configuration loaded successfully. Compiler input: {:?}, Output: {:?}",
                   name,
                   runtime_config.compile_args.input,
                   runtime_config.compile_args.output)
        }
        Err(e) => {
            format!("Hello, {}! Configuration error: {}", name, e)
        }
    }
}

#[tauri::command]
pub async fn load_doc_from_path(path: String) -> Result<bool, AppError> {
    info!("load_doc_from_path : {}", &path);
    let path_buf = PathBuf::from(&path);

    // 使用新的配置系统创建SystemWorld
    match SystemWorld::load_from_path(&path_buf) {
        Ok(mut _world) => {
            info!("succeeded to load world from path: {}", &path);
            Ok(true)
        }
        Err(e) => {
            warn!("failed to load world from path {}: {}", &path, e);
            Err(AppError::Unknown)
        }
    }
}

// #[tauri::command]
// pub async fn compile_doc<R: Runtime>(
//     window: Window<R>,
//     appstate: State<'_, Arc<AppState<R>>>,
//     path: String,
// ) -> Result<bool, AppError> {
//     info!("load_doc_from_path : {}", &path);
//     let path_buf = PathBuf::from(&path);
//     let loaded = appstate.load_world_from_path(&path_buf, &window);
//     info!("succeed load_world_from_path {}", &path);
//     Ok(loaded)
// }

// #[tauri::command]
// pub async fn compile_doc<R: Runtime>(
//     window: Window<R>,
//     appstate: State<'_, Arc<AppState<R>>>,
//     path: PathBuf,
//     content: String,
// ) -> Result<(Vec<TypstPage>, AppError)> {
//     // let project = project(&window, &project_manager)?;
//     let mut world = appstate.get_world(&window).expect("get_world");
//     let source_id = world
//         .slot_update(&path, Some(content.clone()))
//         .map_err(Into::<Error>::into)?;

//     if !world.is_main_set() {
//         let config = project.config.read().unwrap();
//         if config.apply_main(&project, &mut world).is_err() {
//             debug!("skipped compilation for {:?} (main not set)", project);
//             return Err(Error::Unknown);
//         }
//     }

//     let now = Instant::now();

//     let mut pages: Vec<TypstPage> = Vec::new();
//     let mut diags: Vec<TypstSourceDiagnostic> = Vec::new();
//     match typst::compile(&*world).output {
//         Ok(doc) => {
//             let elapsed = now.elapsed();
//             debug!(
//                 "compilation succeeded for {:?} in {:?} ms",
//                 project,
//                 elapsed.as_millis()
//             );
//             let mut idx: u32 = 0;
//             for page in &doc.pages {
//                 let mut hasher = SipHasher::new();
//                 page.frame.hash(&mut hasher);
//                 let hash = hex::encode(hasher.finish128().as_bytes());
//                 let width = page.frame.width().to_pt();
//                 let height = page.frame.height().to_pt();
//                 idx += 1;
//                 let pag = TypstPage {
//                     num: idx,
//                     width,
//                     height,
//                     hash: hash.clone(),
//                 };
//                 pages.push(pag);
//             }

//             project.cache.write().unwrap().document = Some(doc);
//         }
//         Err(diagnostics) => {
//             debug!("compilation failed with {:?} diagnostics", &diagnostics);

//             let source = world.source(source_id);
//             let diagnostics: Vec<TypstSourceDiagnostic> = match source {
//                 Ok(source) => diagnostics
//                     .iter()
//                     .filter(|d| d.span.id() == Some(source_id))
//                     .filter_map(|d| {
//                         let span = source.find(d.span)?;
//                         let range = span.range();

//                         let message = d.message.to_string();
//                         Some(TypstSourceDiagnostic {
//                             pos: get_range_position(&content, range.clone()),
//                             range,
//                             severity: match d.severity {
//                                 Severity::Error => TypstDiagnosticSeverity::Error,
//                                 Severity::Warning => TypstDiagnosticSeverity::Warning,
//                             },
//                             message,
//                             hints: d.hints.iter().map(|hint| hint.to_string()).collect(),
//                         })
//                     })
//                     .collect(),
//                 Err(_) => vec![],
//             };

//             diags = diagnostics.clone();
//         }
//     }

//     Ok((pages, diags))
// }
