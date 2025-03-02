use chrono::{DateTime, Utc};
use serde::Deserialize;
use typst::layout::{Frame, Page, PageRanges, PagedDocument};
use typst_pdf::PdfStandard;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct CompileArgs {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub format: Option<OutputFormat>,
    pub pages: Option<PageRanges>,
    pub pdf_standard: Vec<PdfStandard>,
    pub ppi: f32,
    pub make_deps: Option<PathBuf>,
    pub open: Option<Option<String>>,
    pub timings: Option<Option<PathBuf>>,
    pub world: WorldArgs,
    pub process: ProcessArgs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitArgs {
    pub template: String,
    pub dir: Option<String>,
    pub package: PackageArgs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QueryArgs {
    pub input: PathBuf,
    pub selector: String,
    pub field: Option<String>,
    pub one: bool,
    pub format: String,
    pub pretty: bool,
    pub world: WorldArgs,
    pub process: ProcessArgs,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FontsArgs {
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackageArgs {
    pub package_path: Option<PathBuf>,
    pub package_cache_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldArgs {
    pub root: Option<PathBuf>,
    pub inputs: Vec<(String, String)>,
    pub font: FontArgs,
    pub package: PackageArgs,
    pub creation_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessArgs {
    pub jobs: Option<usize>,
    pub features: Vec<String>,
    pub diagnostic_format: DiagnosticFormat,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FontArgs {
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
}

/// Which format to use for diagnostics.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub enum DiagnosticFormat {
    #[default]
    Human,
    Short,
}

impl Display for DiagnosticFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DiagnosticFormat::Human => write!(f, "human"),
            DiagnosticFormat::Short => write!(f, "short"),
        }
    }
}
impl FromStr for DiagnosticFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "human" => Ok(DiagnosticFormat::Human),
            "short" => Ok(DiagnosticFormat::Short),
            _ => Err(format!("unknown diagnostic format: {}", s)),
        }
    }
}

/// Which format to use for the generated output file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize)]
pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    Html,
}

