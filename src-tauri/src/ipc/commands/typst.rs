use super::{Error, Result};
use crate::ipc::commands::project;
use crate::ipc::model::TypstRenderResponse;
use crate::ipc::{
    TypstCompileEvent, TypstDiagnosticSeverity, TypstDocument, TypstPage, TypstSourceDiagnostic,
};
use crate::project::ProjectManager;
use base64::Engine;
use log::{debug, info};
use serde::Serialize;
use serde_repr::Serialize_repr;
use siphasher::sip128::{Hasher128, SipHasher};
use std::hash::Hash;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tauri::Runtime;
use typst::diag::Severity;
use typst::visualize::Color;
use typst::World;
use typst_ide::{Completion, CompletionKind};

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum TypstCompletionKind {
    Syntax = 1,
    Function = 2,
    Parameter = 3,
    Constant = 4,
    Symbol = 5,
    Type = 6,
}

#[derive(Serialize, Debug)]
pub struct TypstCompletion {
    kind: TypstCompletionKind,
    label: String,
    apply: Option<String>,
    detail: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct TypstCompleteResponse {
    offset: usize,
    completions: Vec<TypstCompletion>,
}

impl From<Completion> for TypstCompletion {
    fn from(value: Completion) -> Self {
        Self {
            kind: match value.kind {
                CompletionKind::Syntax => TypstCompletionKind::Syntax,
                CompletionKind::Func => TypstCompletionKind::Function,
                CompletionKind::Param => TypstCompletionKind::Parameter,
                CompletionKind::Constant => TypstCompletionKind::Constant,
                CompletionKind::Symbol(_) => TypstCompletionKind::Symbol,
                CompletionKind::Type => TypstCompletionKind::Type,
            },
            label: value.label.to_string(),
            apply: value.apply.map(|s| s.to_string()),
            detail: value.detail.map(|s| s.to_string()),
        }
    }
}
// todo delete
#[tauri::command]
pub async fn typst_slot_update<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
) -> Result<()> {
    let project = project(&window, &project_manager)?;

    let mut world = project.world.lock().unwrap();
    let _ = world
        .slot_update(&path, Some(content))
        .map_err(Into::<Error>::into)?;
    Ok(())
}

#[tauri::command]
pub async fn typst_compile_doc<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
) -> Result<(Vec<TypstPage>, Vec<TypstSourceDiagnostic>)> {

    let project = project(&window, &project_manager)?;
    let mut world = project.world.lock().unwrap();
    let source_id = world
        .slot_update(&path, Some(content.clone()))
        .map_err(Into::<Error>::into)?;

    if !world.is_main_set() {
        
        let config = project.config.read().unwrap();
        if config.apply_main(&project, &mut world).is_err() {
            debug!("skipped compilation for {:?} (main not set)", project);
            return Err(Error::Unknown);
        }
    }

    let now = Instant::now();

    let mut pages: Vec<TypstPage> = Vec::new();
    let mut diags: Vec<TypstSourceDiagnostic> = Vec::new();
    match typst::compile(&*world).output {
        Ok(doc) => {
            let elapsed = now.elapsed();
            debug!(
                "compilation succeeded for {:?} in {:?} ms",
                project,
                elapsed.as_millis()
            );
            let mut idx: u32 = 0;
            for page in &doc.pages {
                let mut hasher = SipHasher::new();
                page.frame.hash(&mut hasher);
                let hash = hex::encode(hasher.finish128().as_bytes());
                let width = page.frame.width().to_pt();
                let height = page.frame.height().to_pt();
                idx += 1;
                let pag = TypstPage {
                    num: idx,
                    width,
                    height,
                    hash: hash.clone(),
                };
                pages.push(pag);
            }

            project.cache.write().unwrap().document = Some(doc);
        }
        Err(diagnostics) => {
            debug!("compilation failed with {:?} diagnostics", &diagnostics);

            let source = world.source(source_id);
            let diagnostics: Vec<TypstSourceDiagnostic> = match source {
                Ok(source) => diagnostics
                    .iter()
                    .filter(|d| d.span.id() == Some(source_id))
                    .filter_map(|d| {
                        let span = source.find(d.span)?;
                        let range = span.range();

                        let message = d.message.to_string();
                        Some(TypstSourceDiagnostic {
                            pos: get_range_position(&content, range.clone()),
                            range,
                            severity: match d.severity {
                                Severity::Error => TypstDiagnosticSeverity::Error,
                                Severity::Warning => TypstDiagnosticSeverity::Warning,
                            },
                            message,
                            hints: d.hints.iter().map(|hint| hint.to_string()).collect(),
                        })
                    })
                    .collect(),
                Err(_) => vec![],
            };

            diags = diagnostics.clone();
        }
    }

    Ok((pages, diags))
}


pub fn get_range_position(text: &str, rang: Range<usize>) -> (usize, usize) {
    let mut ln = 0;
    let mut cn = 0;
    let mut total: usize = 0;
    for line in text.lines() {
       
        ln += 1;
        let row = line.chars().count() + 1;

        if total <= rang.start && rang.start <= total + row {
            cn = rang.start - total;
            break;
        }

        total += row;        
    }
    return (ln, cn);
}

#[tauri::command]
pub async fn typst_render<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    page: usize,
    scale: f32,
    nonce: u32,
) -> Result<TypstRenderResponse> {
    info!(
        "typst_render page:{} scale: {} nonce: {}",
        page, scale, nonce
    );
    let project = project_manager
        .get_project(&window)
        .ok_or(Error::UnknownProject)?;

    let cache = project.cache.read().unwrap();

    if let Some(p) = cache
        .document
        .as_ref()
        .and_then(|doc| doc.pages.get(page - 1))
    {
        let now = Instant::now();

        let bmp = typst_render::render(p, scale);
        if let Ok(image) = bmp.encode_png() {
            let elapsed = now.elapsed();
            debug!(
                "rendering complete for page {} in {} ms",
                page,
                elapsed.as_millis()
            );
            let b64 = base64::engine::general_purpose::STANDARD.encode(image);
            return Ok(TypstRenderResponse {
                image: b64,
                width: bmp.width(),
                height: bmp.height(),
                nonce,
            });
        }
    }

    Err(Error::Unknown)
}

#[tauri::command]
pub async fn typst_autocomplete<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
    offset: usize,
    explicit: bool,
) -> Result<TypstCompleteResponse> {
    let project = project(&window, &project_manager)?;
    let mut world = project.world.lock().unwrap();

    let offset = content
        .char_indices()
        .nth(offset)
        .map(|a| a.0)
        .unwrap_or(content.len());

    // TODO: Improve error typing
    let source_id = world
        .slot_update(&*path, Some(content.clone()))
        .map_err(Into::<Error>::into)?;

    let source = world.source(source_id).map_err(Into::<Error>::into)?;

    let (completed_offset, completions) =
        typst_ide::autocomplete(&*world, None, &source, offset, explicit)
            .ok_or_else(|| Error::Unknown)?;

    let completed_char_offset = content[..completed_offset].chars().count();
    Ok(TypstCompleteResponse {
        offset: completed_char_offset,
        completions: completions.into_iter().map(TypstCompletion::from).collect(),
    })
}
