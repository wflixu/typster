use super::args::{CompileArgs, FontsArgs, InitArgs, QueryArgs, OutputFormat, Pages, DiagnosticFormat, Feature};
use typst_pdf::PdfStandard;

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use chrono::{DateTime, Utc};
use toml::Value;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};

use std::num::NonZeroUsize;
use std::ops::RangeInclusive;

use std::str::FromStr;

// 定义全局静态单例
pub static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub compiler: CompilerConfig,
    pub world: WorldConfig,
    pub package: PackageConfig,
    pub init: InitConfig,
    pub query: QueryConfig,
    pub process: ProcessConfig,
    pub fonts: FontsConfig,
    pub cert: Option<PathBuf>, // 添加 cert 配置项
}

impl Config {
    pub fn load_from_file(path: PathBuf) -> Self {
        let config_str = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config file")
    }

    pub fn save_to_file(&self, path: PathBuf) {
        let config_str = toml::to_string(self).expect("Failed to serialize config");
        fs::write(path, config_str).expect("Failed to write config file");
    }
    
    pub fn get_compile_args(&self) -> CompileArgs {
        CompileArgs {
            input: self.compiler.input.clone(),
            output: Some(self.compiler.output.clone()),
            format: self.compiler.format,
            pages: self.compiler.pages.clone(),
            pdf_standard: self.compiler.pdf_standard.clone(),
            ppi: self.compiler.ppi,
            make_deps: Some(self.compiler.make_deps.clone()),
            open: if self.compiler.open { Some("".to_string()) } else { None },
            timings: Some(self.compiler.timings.clone()),
            root: Some(self.world.root.clone()),
            inputs: self.world.inputs.iter().map(|s| (s.clone(), s.clone())).collect(),
            font_paths: self.fonts.font_paths.clone(),
            ignore_system_fonts: self.fonts.ignore_system_fonts,
            creation_timestamp: if !self.world.creation_timestamp.is_empty() {
                Some(DateTime::parse_from_rfc3339(&self.world.creation_timestamp).unwrap().with_timezone(&Utc))
            } else {
                None
            },
            package_path: Some(self.package.package_path.clone()),
            package_cache_path: Some(self.package.package_cache_path.clone()),
            jobs: Some(self.process.jobs),
            diagnostic_format: self.process.diagnostic_format,
            features: self.process.features.iter().map(|s| Feature::Html).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompilerConfig {
    pub input: PathBuf,
    pub output: PathBuf,
    pub format: Option<OutputFormat>,
    pub pages: Option<Vec<Pages>>,
    pub pdf_standard: Vec<PdfStandard>,
    pub ppi: f32,
    pub make_deps: PathBuf,
    pub open: bool,
    pub timings: PathBuf,
    pub optimize: bool,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldConfig {
    pub root: PathBuf,
    pub inputs: Vec<String>,
    pub creation_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageConfig {
    pub package_path: PathBuf,
    pub package_cache_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitConfig {
    pub template: String,
    pub dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryConfig {
    pub input: PathBuf,
    pub selector: String,
    pub field: String,
    pub one: bool,
    pub format: String,
    pub pretty: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcessConfig {
    pub jobs: usize,
    pub features: Vec<String>,
    pub diagnostic_format: DiagnosticFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FontsConfig {
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
}
