//! 配置系统模块
//!
//! 提供三级配置加载架构：
//! 配置文件 → rawConfig → runtimeConfig

pub mod loader;
pub mod global;
pub mod project;
pub mod validator;
pub mod repairer;

pub use loader::ConfigLoader;
pub use global::GlobalConfigManager;
pub use project::ProjectConfigManager;
pub use validator::ConfigValidator;
pub use repairer::ConfigRepairer;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// 配置来源
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigSource {
    Global,
    Project,
    User,
}

/// 配置文件结构
#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub config: crate::typst_service::Config,
    pub source: ConfigSource,
}

/// 原始配置结构（合并后的配置）
#[derive(Debug, Clone)]
pub struct RawConfig {
    pub global: crate::typst_service::Config,
    pub project: Option<crate::typst_service::Config>,
    pub user: Option<crate::typst_service::Config>,
}

/// 运行时配置结构（解析后的最终配置）
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub compile_args: crate::typst_service::CompileArgs,
    pub world_args: crate::typst_service::WorldArgs,
    pub process_args: crate::typst_service::ProcessArgs,
}

/// 配置错误类型
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("配置文件不存在: {0}")]
    FileNotFound(PathBuf),

    #[error("配置文件格式错误: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("配置文件序列化错误: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("配置验证失败: {0}")]
    ValidationError(String),

    #[error("路径解析失败: {0}")]
    PathResolutionError(String),

    #[error("必需参数缺失: {0}")]
    RequiredParameterMissing(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}