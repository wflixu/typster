//! 配置加载器
//!
//! 负责加载和合并全局配置、项目配置，生成运行时配置

use super::*;
use crate::typst_service::Config;
use std::path::{Path, PathBuf};

/// 配置加载器
pub struct ConfigLoader {
    global_manager: GlobalConfigManager,
    project_manager: ProjectConfigManager,
}

impl ConfigLoader {
    /// 创建新的配置加载器
    pub fn new() -> Self {
        Self {
            global_manager: GlobalConfigManager::new(),
            project_manager: ProjectConfigManager::new(),
        }
    }

    /// 加载配置的完整流程
    pub fn load_config(&self, project_root: Option<&Path>) -> Result<RuntimeConfig, ConfigError> {
        // 1. 加载全局配置
        let global_config = self.global_manager.load_or_create()?;

        // 2. 加载项目配置（如果有项目根目录）
        let project_config = if let Some(root) = project_root {
            self.project_manager.load(root).ok()
        } else {
            None
        };

        // 3. 合并配置
        let raw_config = RawConfig::merge(global_config, project_config);

        // 4. 验证配置
        ConfigValidator::validate(&raw_config)?;

        // 5. 转换为运行时配置
        RuntimeConfig::from_raw(raw_config)
    }

    /// 加载配置并处理错误（自动修复损坏的配置）
    pub fn load_config_with_fallback(&self, project_root: Option<&Path>) -> Result<RuntimeConfig, ConfigError> {
        match self.load_config(project_root) {
            Ok(config) => Ok(config),
            Err(ConfigError::ParseError(_)) | Err(ConfigError::ValidationError(_)) => {
                // 配置损坏，尝试修复
                log::warn!("配置损坏，尝试修复...");

                // 重新生成全局配置
                self.global_manager.create_default()?;

                // 重新加载
                self.load_config(project_root)
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl RawConfig {
    /// 合并配置
    pub fn merge(global: ConfigFile, project: Option<ConfigFile>) -> Self {
        RawConfig {
            global: global.config,
            project: project.map(|p| p.config),
            user: None,
        }
    }

    /// 合并两个配置
    fn merge_configs(global: Config, project: Config) -> Config {
        Config {
            compiler: Self::merge_compiler(global.compiler, project.compiler),
            world: Self::merge_world(global.world, project.world),
            package: Self::merge_package(global.package, project.package),
            init: Self::merge_init(global.init, project.init),
            query: Self::merge_query(global.query, project.query),
            process: Self::merge_process(global.process, project.process),
            fonts: Self::merge_fonts(global.fonts, project.fonts),
            cert: project.cert.or(global.cert),
        }
    }

    fn merge_compiler(global: crate::typst_service::CompilerConfig, project: crate::typst_service::CompilerConfig) -> crate::typst_service::CompilerConfig {
        crate::typst_service::CompilerConfig {
            input: project.input,
            output: project.output,
            format: project.format.or(global.format),
            pages: project.pages.or(global.pages),
            pdf_standard: if project.pdf_standard.is_empty() { global.pdf_standard } else { project.pdf_standard },
            ppi: project.ppi,
            make_deps: project.make_deps,
            open: project.open,
            timings: project.timings,
            optimize: project.optimize,
        }
    }

    fn merge_world(global: crate::typst_service::WorldConfig, project: crate::typst_service::WorldConfig) -> crate::typst_service::WorldConfig {
        crate::typst_service::WorldConfig {
            root: project.root,
            inputs: if project.inputs.is_empty() { global.inputs } else { project.inputs },
            creation_timestamp: if project.creation_timestamp.is_empty() { global.creation_timestamp } else { project.creation_timestamp },
        }
    }

    fn merge_package(global: crate::typst_service::PackageConfig, project: crate::typst_service::PackageConfig) -> crate::typst_service::PackageConfig {
        crate::typst_service::PackageConfig {
            package_path: project.package_path,
            package_cache_path: project.package_cache_path,
        }
    }

    fn merge_init(global: crate::typst_service::InitConfig, project: crate::typst_service::InitConfig) -> crate::typst_service::InitConfig {
        crate::typst_service::InitConfig {
            template: if project.template.is_empty() { global.template } else { project.template },
            dir: project.dir,
        }
    }

    fn merge_query(global: crate::typst_service::QueryConfig, project: crate::typst_service::QueryConfig) -> crate::typst_service::QueryConfig {
        crate::typst_service::QueryConfig {
            input: project.input,
            selector: if project.selector.is_empty() { global.selector } else { project.selector },
            field: if project.field.is_empty() { global.field } else { project.field },
            one: project.one,
            format: if project.format.is_empty() { global.format } else { project.format },
            pretty: project.pretty,
        }
    }

    fn merge_process(global: crate::typst_service::ProcessConfig, project: crate::typst_service::ProcessConfig) -> crate::typst_service::ProcessConfig {
        crate::typst_service::ProcessConfig {
            jobs: project.jobs,
            features: if project.features.is_empty() { global.features } else { project.features },
            diagnostic_format: project.diagnostic_format,
        }
    }

    fn merge_fonts(global: crate::typst_service::FontsConfig, project: crate::typst_service::FontsConfig) -> crate::typst_service::FontsConfig {
        crate::typst_service::FontsConfig {
            font_paths: if project.font_paths.is_empty() { global.font_paths } else { project.font_paths },
            ignore_system_fonts: project.ignore_system_fonts,
        }
    }
}

impl RuntimeConfig {
    /// 从原始配置转换为运行时配置
    pub fn from_raw(raw: RawConfig) -> Result<Self, ConfigError> {
        // 使用项目配置（如果存在），否则使用全局配置
        let config = if let Some(project) = &raw.project {
            project
        } else {
            &raw.global
        };

        // 转换为编译参数
        let compile_args = config.get_compile_args();

        // 构建世界参数
        let world_args = crate::typst_service::WorldArgs {
            root: Some(config.world.root.clone()),
            inputs: config.world.inputs.iter().map(|s| (s.clone(), s.clone())).collect(),
            font: crate::typst_service::FontArgs {
                font_paths: config.fonts.font_paths.clone(),
                ignore_system_fonts: config.fonts.ignore_system_fonts,
            },
            package: crate::typst_service::PackageArgs {
                package_path: Some(config.package.package_path.clone()),
                package_cache_path: Some(config.package.package_cache_path.clone()),
            },
            creation_timestamp: if !config.world.creation_timestamp.is_empty() {
                Some(chrono::DateTime::parse_from_rfc3339(&config.world.creation_timestamp)
                    .unwrap()
                    .with_timezone(&chrono::Utc))
            } else {
                None
            },
        };

        // 构建进程参数
        let process_args = crate::typst_service::ProcessArgs {
            jobs: Some(config.process.jobs),
            features: config.process.features.iter().map(|s| s.clone()).collect(),
            diagnostic_format: config.process.diagnostic_format,
        };

        Ok(RuntimeConfig {
            compile_args,
            world_args,
            process_args,
        })
    }
}