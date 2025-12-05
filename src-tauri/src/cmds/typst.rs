use std::path::PathBuf;
use tauri::command;
use crate::util::AppError;
use crate::typst_service::{SystemWorld, Timer, CompileArgs, OutputFormat, Pages};
use ecow::eco_format;
use base64::Engine;

// Match the frontend interface
pub type TypstCompileResult = (Vec<PageData>, Vec<DiagnosticInfo>);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PageData {
    pub hash: String,
    pub width: f64,
    pub height: f64,
    pub num: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DiagnosticInfo {
    pub range: Range,
    pub severity: String, // "error" or "warning"
    pub message: String,
    pub hints: Vec<String>,
    pub pos: [u32; 2], // [line, column]
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[command]
pub async fn typst_compile_doc(path: String, content: String) -> Result<TypstCompileResult, AppError> {
    use std::fs;
    use std::io::Write;

    let input_path = PathBuf::from(&path);

    // Create a temporary file with the content
    let temp_path = input_path.with_extension("tmp.typ");

    {
        let mut temp_file = fs::File::create(&temp_path)?;
        temp_file.write_all(content.as_bytes())?;
    }

    // Prepare compile arguments
    let args = CompileArgs {
        input: temp_path.clone(),
        output: None,
        format: Some(OutputFormat::Png),
        pages: None,
        pdf_standard: vec![],
        ppi: 72.0,
        make_deps: None,
        open: None,
        timings: None,
        root: input_path.parent().map(|p| p.to_path_buf()),
        inputs: vec![],
        font_paths: vec![],
        ignore_system_fonts: false,
        creation_timestamp: None,
        package_path: None,
        package_cache_path: None,
        jobs: None,
        diagnostic_format: crate::typst_service::DiagnosticFormat::Human,
        features: vec![],
    };

    // Initialize timer
    let mut timer = Timer::new(&args);

    // Compile the document
    match crate::typst_service::compile(&mut timer, &args) {
        Ok(()) => {
            // Read generated PNG files
            let mut pages = Vec::new();
            let mut page_number = 1;

            // Look for generated PNG files
            let base_name = temp_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");

            let output_dir = temp_path.parent().unwrap_or(&temp_path);

            while let Some(png_path) = find_page_file(&output_dir.to_path_buf(), base_name, page_number, "png") {
                if let Ok(png_data) = std::fs::read(&png_path) {
                    let png_base64 = base64::prelude::BASE64_STANDARD.encode(&png_data);

                    // Get dimensions from PNG data (basic implementation)
                    let (width, height) = extract_png_dimensions(&png_data).unwrap_or((612.0, 792.0)); // Default letter size

                    // Calculate hash (for now, use page number)
                    let hash = format!("page-{}", page_number);

                    pages.push(PageData {
                        hash,
                        width: width as f64,
                        height: height as f64,
                        num: page_number,
                    });
                }
                page_number += 1;
            }

            // Clean up temporary files
            let _ = std::fs::remove_file(&temp_path);

            Ok((pages, vec![]))
        }
        Err(e) => {
            // Clean up temporary files
            let _ = std::fs::remove_file(&temp_path);

            Err(AppError::Unknown)
        }
    }
}

#[command]
pub async fn typst_render(path: String, content: String, page: u32) -> Result<String, AppError> {
    // This will be implemented for individual page rendering
    // For now, return a placeholder
    Ok(format!("Page {} rendering not yet implemented", page))
}

#[command]
pub async fn typst_autocomplete(path: String, position: u32) -> Result<Vec<String>, AppError> {
    // This will be implemented for code completion
    // For now, return empty vector
    Ok(vec![])
}

#[command]
pub async fn typst_slot_update(path: String, slot: String, content: String) -> Result<(), AppError> {
    // This will be implemented for slot updates in the virtual file system
    Ok(())
}

#[command]
pub async fn export_pdf(path: String, output: String) -> Result<String, AppError> {
    // This will be implemented for PDF export
    Ok(output)
}

// Helper function to find page files with proper numbering
fn find_page_file(dir: &PathBuf, base_name: &str, page: u32, extension: &str) -> Option<PathBuf> {
    let pattern = format!("{}-{}.{}", base_name, page, extension);
    let path = dir.join(&pattern);

    if path.exists() {
        Some(path)
    } else {
        None
    }
}

// Helper function to extract PNG dimensions
fn extract_png_dimensions(data: &[u8]) -> Option<(f32, f32)> {
    if data.len() < 24 {
        return None;
    }

    // PNG files start with 8-byte signature
    if &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }

    // The IHDR chunk starts at byte 8
    // Width and height are stored as 4-byte big-endian integers starting at byte 16
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);

    Some((width as f32, height as f32))
}