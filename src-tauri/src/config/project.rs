//! 项目配置管理器
//!
//! 负责管理项目目录下的 .typster/config.toml 配置文件

use super::*;
use crate::typst_service::Config;
use std::path::{Path, PathBuf};

/// 项目配置管理器
pub struct ProjectConfigManager;

impl ProjectConfigManager {
    /// 创建新的项目配置管理器
    pub fn new() -> Self {
        Self
    }

    /// 加载项目配置
    pub fn load(&self, project_root: &Path) -> Result<ConfigFile, ConfigError> {
        let config_path = Self::project_config_path(project_root);

        if !config_path.exists() {
            return Err(ConfigError::FileNotFound(config_path));
        }

        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|_| ConfigError::FileNotFound(config_path.clone()))?;

        let config: Config = toml::from_str(&config_str)?;

        Ok(ConfigFile {
            path: config_path,
            config,
            source: ConfigSource::Project,
        })
    }

    /// 创建项目默认配置
    pub fn create_default(&self, project_root: &Path) -> Result<ConfigFile, ConfigError> {
        let config_dir = Self::project_config_dir(project_root);
        let config_path = Self::project_config_path(project_root);

        // 确保配置目录存在
        std::fs::create_dir_all(&config_dir)?;

        // 生成项目特定的默认配置
        let project_config = Self::generate_project_config(project_root);

        // 保存配置
        let config_str = toml::to_string_pretty(&project_config)?;
        std::fs::write(&config_path, config_str)?;

        Ok(ConfigFile {
            path: config_path,
            config: project_config,
            source: ConfigSource::Project,
        })
    }

    /// 检查项目配置是否存在
    pub fn exists(&self, project_root: &Path) -> bool {
        Self::project_config_path(project_root).exists()
    }

    /// 获取项目配置路径
    pub fn get_config_path(project_root: &Path) -> PathBuf {
        Self::project_config_path(project_root)
    }

    /// 生成项目特定的默认配置
    fn generate_project_config(project_root: &Path) -> Config {
        let mut config = Self::generate_base_config();

        // 根据项目结构调整默认配置
        config.compiler.input = project_root.join("src").join("main.typ");
        config.compiler.output = project_root.join("dist").join("document.pdf");
        config.world.root = project_root.join("src");
        config.package.package_path = Self::project_config_dir(project_root).join("packages");
        config.package.package_cache_path = Self::project_config_dir(project_root).join("cache");
        config.fonts.font_paths = vec![
            project_root.join("assets").join("fonts"),
            Self::project_config_dir(project_root).join("fonts"),
        ];

        config
    }

    /// 生成基础配置
    fn generate_base_config() -> Config {
        Config {
            compiler: crate::typst_service::CompilerConfig {
                input: PathBuf::from("src/main.typ"),
                output: PathBuf::from("dist/document.pdf"),
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
                root: PathBuf::from("src"),
                inputs: Vec::new(),
                creation_timestamp: String::new(),
            },
            package: crate::typst_service::PackageConfig {
                package_path: PathBuf::from(".typster/packages"),
                package_cache_path: PathBuf::from(".typster/cache"),
            },
            init: crate::typst_service::InitConfig {
                template: "default".to_string(),
                dir: PathBuf::from("new_project"),
            },
            query: crate::typst_service::QueryConfig {
                input: PathBuf::from("src/main.typ"),
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
                font_paths: vec![PathBuf::from("assets/fonts"), PathBuf::from(".typster/fonts")],
                ignore_system_fonts: false,
            },
            cert: None,
        }
    }

    /// 获取项目配置目录
    fn project_config_dir(project_root: &Path) -> PathBuf {
        project_root.join(".typster")
    }

    /// 获取项目配置文件路径
    fn project_config_path(project_root: &Path) -> PathBuf {
        Self::project_config_dir(project_root).join("config.toml")
    }
}

impl Default for ProjectConfigManager {
    fn default() -> Self {
        Self::new()
    }
}