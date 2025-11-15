//! 配置验证器
//!
//! 负责验证配置的完整性和有效性

use super::*;

/// 配置验证器
pub struct ConfigValidator;

impl ConfigValidator {
    /// 验证配置
    pub fn validate(raw_config: &RawConfig) -> Result<(), ConfigError> {
        // 使用项目配置（如果存在），否则使用全局配置
        let config = if let Some(project) = &raw_config.project {
            project
        } else {
            &raw_config.global
        };

        Self::validate_required_fields(config)?;
        Self::validate_paths(config)?;
        Self::validate_ranges(config)?;

        Ok(())
    }

    /// 验证必需字段
    fn validate_required_fields(config: &crate::typst_service::Config) -> Result<(), ConfigError> {
        if config.compiler.input.to_string_lossy().is_empty() {
            return Err(ConfigError::RequiredParameterMissing("compiler.input".to_string()));
        }

        if config.compiler.output.to_string_lossy().is_empty() {
            return Err(ConfigError::RequiredParameterMissing("compiler.output".to_string()));
        }

        Ok(())
    }

    /// 验证路径
    fn validate_paths(config: &crate::typst_service::Config) -> Result<(), ConfigError> {
        // 验证字体路径格式
        for path in &config.fonts.font_paths {
            if path.to_string_lossy().is_empty() {
                return Err(ConfigError::ValidationError("字体路径不能为空".to_string()));
            }
        }

        // 验证包路径
        if config.package.package_path.to_string_lossy().is_empty() {
            return Err(ConfigError::ValidationError("包路径不能为空".to_string()));
        }

        if config.package.package_cache_path.to_string_lossy().is_empty() {
            return Err(ConfigError::ValidationError("包缓存路径不能为空".to_string()));
        }

        Ok(())
    }

    /// 验证参数范围
    fn validate_ranges(config: &crate::typst_service::Config) -> Result<(), ConfigError> {
        if config.compiler.ppi <= 0.0 {
            return Err(ConfigError::ValidationError("PPI必须大于0".to_string()));
        }

        if config.process.jobs == 0 {
            return Err(ConfigError::ValidationError("作业数必须大于0".to_string()));
        }

        // 验证页面范围格式
        if let Some(pages) = &config.compiler.pages {
            for page_range in pages {
                // 检查页面范围是否有效
                let start = page_range.0.start();
                let end = page_range.0.end();

                // 如果开始和结束都有值，确保开始 <= 结束
                if let (Some(start_val), Some(end_val)) = (start, end) {
                    if start_val > end_val {
                        return Err(ConfigError::ValidationError("页面范围无效：开始页大于结束页".to_string()));
                    }
                }
            }
        }

        Ok(())
    }
}