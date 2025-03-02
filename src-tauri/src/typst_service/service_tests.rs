#[cfg(test)]
mod tests {
    use super::service::*;
    use super::service::Config;

    #[test]
    fn test_load_config() {
        // 创建一个 config，测试 是否能 正确的加载
        let config = Config::load();
        assert_eq!(config.compile.is_some(), true);
        assert_eq!(config.init.is_some(), true);
        assert_eq!(config.query.is_some(), true);
        assert_eq!(config.fonts.is_some(), true);
    }
}
