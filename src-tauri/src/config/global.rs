//! 全局配置管理器
//!
//! 负责管理 ~/.config/typster/config.toml 全局配置文件

use super::*;
use crate::typst_service::Config;
use std::path::PathBuf;

/// 全局配置管理器
pub struct GlobalConfigManager {
    config_dir: PathBuf,
    config_path: PathBuf,
}

impl GlobalConfigManager {
    /// 创建新的全局配置管理器
    pub fn new() -> Self {
        let config_dir = Self::global_config_dir();
        let config_path = config_dir.join("config.toml");

        Self {
            config_dir,
            config_path,
        }
    }

    /// 加载或创建全局配置
    pub fn load_or_create(&self) -> Result<ConfigFile, ConfigError> {
        if self.config_path.exists() {
            self.load()
        } else {
            self.create_default()
        }
    }

    /// 加载全局配置
    pub fn load(&self) -> Result<ConfigFile, ConfigError> {
        let config_str = std::fs::read_to_string(&self.config_path)
            .map_err(|_| ConfigError::FileNotFound(self.config_path.clone()))?;

        let config: Config = toml::from_str(&config_str)?;

        Ok(ConfigFile {
            path: self.config_path.clone(),
            config,
            source: ConfigSource::Global,
        })
    }

    /// 创建默认全局配置
    pub fn create_default(&self) -> Result<ConfigFile, ConfigError> {
        // 确保配置目录存在
        std::fs::create_dir_all(&self.config_dir)?;

        // 生成默认配置
        let default_config = Self::generate_default_config();

        // 保存配置
        let config_str = toml::to_string_pretty(&default_config)?;
        std::fs::write(&self.config_path, config_str)?;

        Ok(ConfigFile {
            path: self.config_path.clone(),
            config: default_config,
            source: ConfigSource::Global,
        })
    }

    /// 获取全局配置路径
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    /// 生成默认配置
    fn generate_default_config() -> Config {
        Config {
            compiler: crate::typst_service::CompilerConfig {
                input: PathBuf::from("main.typ"),
                output: PathBuf::from("output.pdf"),
                format: Some(crate::typst_service::OutputFormat::Pdf),
                pages: None,
                pdf_standard: vec![],
                ppi: 144.0,
                make_deps: PathBuf::from("Makefile"),
                open: false,
                timings: PathBuf::from("timings.json"),
                optimize: true,
            },
            world: crate::typst_service::WorldConfig {
                root: PathBuf::from("."),
                inputs: Vec::new(),
                creation_timestamp: String::new(),
            },
            package: crate::typst_service::PackageConfig {
                package_path: PathBuf::from("./packages"),
                package_cache_path: PathBuf::from("./cache"),
            },
            init: crate::typst_service::InitConfig {
                template: "default".to_string(),
                dir: PathBuf::from("new_project"),
            },
            query: crate::typst_service::QueryConfig {
                input: PathBuf::from("main.typ"),
                selector: "title".to_string(),
                field: "text".to_string(),
                one: true,
                format: "json".to_string(),
                pretty: true,
            },
            process: crate::typst_service::ProcessConfig {
                jobs: num_cpus::get(),
                features: vec!["html".to_string()],
                diagnostic_format: crate::typst_service::DiagnosticFormat::Short,
            },
            fonts: crate::typst_service::FontsConfig {
                font_paths: vec![PathBuf::from("./fonts")],
                ignore_system_fonts: false,
            },
            cert: None,
        }
    }

    /// 获取全局配置目录
    fn global_config_dir() -> PathBuf {
        dirs::config_dir()
            .map(|mut path| {
                path.push("typster");
                path
            })
            .unwrap_or_else(|| PathBuf::from("~/.config/typster"))
    }
}

impl Default for GlobalConfigManager {
    fn default() -> Self {
        Self::new()
    }
}