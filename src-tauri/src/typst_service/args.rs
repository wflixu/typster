use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;
use typst::layout::{Frame, Page, PageRanges, PagedDocument};
use typst_pdf::PdfStandard;

#[derive(Debug, Clone)]
pub struct CompileArgs {
    pub input: PathBuf,
    pub output: Option<PathBuf>,
    pub format: Option<OutputFormat>,
    pub pages: Option<Vec<Pages>>,
    pub pdf_standard: Vec<PdfStandard>,
    pub ppi: f32,
    pub make_deps: Option<PathBuf>,
    pub open: Option<String>,
    pub timings: Option<PathBuf>,
    pub root: Option<PathBuf>,
    pub inputs: Vec<(String, String)>,
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub package_path: Option<PathBuf>,
    pub package_cache_path: Option<PathBuf>,
    pub jobs: Option<usize>,
    pub diagnostic_format: DiagnosticFormat,
    pub features: Vec<Feature>,
}

// impl Default for CompileArgs {
//     fn default() -> Self {
//         CompileArgs {
//             input: PathBuf::new(),
//             output: None,
//             format: Some(OutputFormat::Pdf),
//             pages: None,
//             pdf_standard: Vec::new(),
//             ppi: 72.0,
//             make_deps: None,
//             open: None,
//             timings: None,
//         }
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InitArgs {
    pub template: String,
    pub dir: Option<String>,
    pub package: PackageArgs,
}

impl Default for InitArgs {
    fn default() -> Self {
        InitArgs {
            template: String::new(),
            dir: None,
            package: PackageArgs::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryArgs {
    pub input: PathBuf,
    pub selector: String,
    pub field: Option<String>,
    pub one: bool,
    pub format: SerializationFormat,
    pub pretty: bool,
    pub world: WorldArgs,
    pub process: ProcessArgs,
}

impl Default for QueryArgs {
    fn default() -> Self {
        QueryArgs {
            input: PathBuf::new(),
            selector: String::new(),
            field: None,
            one: false,
            format: SerializationFormat::default(),
            pretty: false,
            world: WorldArgs::default(),
            process: ProcessArgs::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontsArgs {
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
}

impl Default for FontsArgs {
    fn default() -> Self {
        FontsArgs {
            font_paths: Vec::new(),
            ignore_system_fonts: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageArgs {
    pub package_path: Option<PathBuf>,
    pub package_cache_path: Option<PathBuf>,
}
impl Default for PackageArgs {
    fn default() -> Self {
        PackageArgs {
            package_path: None,
            package_cache_path: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorldArgs {
    pub root: Option<PathBuf>,
    pub inputs: Vec<(String, String)>,
    pub font: FontArgs,
    pub package: PackageArgs,
    pub creation_timestamp: Option<DateTime<Utc>>,
}

impl Default for WorldArgs {
    fn default() -> Self {
        WorldArgs {
            root: None,
            inputs: Vec::new(),
            font: FontArgs {
                font_paths: Vec::new(),
                ignore_system_fonts: false,
            },
            package: PackageArgs {
                package_path: None,
                package_cache_path: None,
            },
            creation_timestamp: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessArgs {
    pub jobs: Option<usize>,
    pub features: Vec<String>,
    pub diagnostic_format: DiagnosticFormat,
}
impl Default for ProcessArgs {
    fn default() -> Self {
        ProcessArgs {
            jobs: None,
            features: Vec::new(),
            diagnostic_format: DiagnosticFormat::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontArgs {
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
}

/// Which format to use for diagnostics.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum OutputFormat {
    Pdf,
    Png,
    Svg,
    Html,
}

/// Implements parsing of page ranges (`1-3`, `4`, `5-`, `-2`), used by the
/// `CompileCommand.pages` argument, through the `FromStr` trait instead of a
/// value parser, in order to generate better errors.
///
/// See also: https://github.com/clap-rs/clap/issues/5065
#[derive(Debug, Clone)]
pub struct Pages(pub RangeInclusive<Option<NonZeroUsize>>);

impl FromStr for Pages {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value
            .split('-')
            .map(str::trim)
            .collect::<Vec<_>>()
            .as_slice()
        {
            [] | [""] => Err("page export range must not be empty"),
            [single_page] => {
                let page_number = parse_page_number(single_page)?;
                Ok(Pages(Some(page_number)..=Some(page_number)))
            }
            ["", ""] => Err("page export range must have start or end"),
            [start, ""] => Ok(Pages(Some(parse_page_number(start)?)..=None)),
            ["", end] => Ok(Pages(None..=Some(parse_page_number(end)?))),
            [start, end] => {
                let start = parse_page_number(start)?;
                let end = parse_page_number(end)?;
                if start > end {
                    Err("page export range must end at a page after the start")
                } else {
                    Ok(Pages(Some(start)..=Some(end)))
                }
            }
            [_, _, _, ..] => Err("page export range must have a single hyphen"),
        }
    }
}

impl Serialize for Pages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let start = self.0.start().map(|n| n.to_string()).unwrap_or_default();
        let end = self.0.end().map(|n| n.to_string()).unwrap_or_default();
        let range_str = if start.is_empty() {
            format!("-{}", end)
        } else if end.is_empty() {
            format!("{}-", start)
        } else if start == end {
            start
        } else {
            format!("{}-{}", start, end)
        };
        serializer.serialize_str(&range_str)
    }
}

impl<'de> Deserialize<'de> for Pages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Parses a single page number.
fn parse_page_number(value: &str) -> Result<NonZeroUsize, &'static str> {
    if value == "0" {
        Err("page numbers start at one")
    } else {
        NonZeroUsize::from_str(value).map_err(|_| "not a valid page number")
    }
}

// Output file format for query command
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SerializationFormat {
    #[default]
    Json,
    Toml,
}
// 为 SerializationFormat 实现 Display
impl Display for SerializationFormat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            SerializationFormat::Json => write!(f, "json"),
            SerializationFormat::Toml => write!(f, "toml"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum Feature {
    Html,
}

impl Display for Feature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Feature::Html => write!(f, "html"),
        }
    }
}
