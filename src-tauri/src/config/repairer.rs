//! 配置修复器
//!
//! 负责修复损坏的配置文件

use super::*;
use crate::typst_service::Config;
use std::path::Path;

/// 配置修复器
pub struct ConfigRepairer;

impl ConfigRepairer {
    /// 修复损坏的配置
    pub fn repair_config(config_path: &Path) -> Result<Config, ConfigError> {
        // 尝试读取现有配置
        let existing_config = match Self::read_config(config_path) {
            Ok(config) => config,
            Err(_) => {
                // 配置文件损坏，生成新的默认配置
                return Self::regenerate_config(config_path);
            }
        };

        // 验证配置
        if let Err(_) = crate::config::validator::ConfigValidator::validate(&RawConfig {
            global: existing_config.clone(),
            project: None,
            user: None,
        }) {
            // 配置验证失败，修复配置
            return Self::fix_config(config_path, existing_config);
        }

        Ok(existing_config)
    }

    /// 读取配置
    fn read_config(config_path: &Path) -> Result<Config, ConfigError> {
        let config_str = std::fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// 重新生成配置
    fn regenerate_config(config_path: &Path) -> Result<Config, ConfigError> {
        let default_config = Self::generate_default_config();
        let config_str = toml::to_string_pretty(&default_config)?;
        std::fs::write(config_path, config_str)?;
        Ok(default_config)
    }

    /// 修复配置
    fn fix_config(config_path: &Path, mut config: Config) -> Result<Config, ConfigError> {
        let default_config = Self::generate_default_config();

        // 修复缺失的字段
        if config.compiler.input.to_string_lossy().is_empty() {
            config.compiler.input = default_config.compiler.input;
        }

        if config.compiler.output.to_string_lossy().is_empty() {
            config.compiler.output = default_config.compiler.output;
        }

        // 修复无效的参数
        if config.compiler.ppi <= 0.0 {
            config.compiler.ppi = default_config.compiler.ppi;
        }

        if config.process.jobs == 0 {
            config.process.jobs = default_config.process.jobs;
        }

        // 修复空的向量
        if config.fonts.font_paths.is_empty() {
            config.fonts.font_paths = default_config.fonts.font_paths;
        }

        if config.process.features.is_empty() {
            config.process.features = default_config.process.features;
        }

        // 保存修复后的配置
        let config_str = toml::to_string_pretty(&config)?;
        std::fs::write(config_path, config_str)?;

        Ok(config)
    }

    /// 生成默认配置
    fn generate_default_config() -> Config {
        Config {
            compiler: crate::typst_service::CompilerConfig {
                input: std::path::PathBuf::from("main.typ"),
                output: std::path::PathBuf::from("output.pdf"),
                format: Some(crate::typst_service::OutputFormat::Pdf),
                pages: None,
                pdf_standard: vec![],
                ppi: 144.0,
                make_deps: std::path::PathBuf::from("Makefile"),
                open: false,
                timings: std::path::PathBuf::from("timings.json"),
                optimize: true,
            },
            world: crate::typst_service::WorldConfig {
                root: std::path::PathBuf::from("."),
                inputs: Vec::new(),
                creation_timestamp: String::new(),
            },
            package: crate::typst_service::PackageConfig {
                package_path: std::path::PathBuf::from("./packages"),
                package_cache_path: std::path::PathBuf::from("./cache"),
            },
            init: crate::typst_service::InitConfig {
                template: "default".to_string(),
                dir: std::path::PathBuf::from("new_project"),
            },
            query: crate::typst_service::QueryConfig {
                input: std::path::PathBuf::from("main.typ"),
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
                font_paths: vec![std::path::PathBuf::from("./fonts")],
                ignore_system_fonts: false,
            },
            cert: None,
        }
    }
}