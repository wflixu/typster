use super::config::*;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_file() {
        let config_path = PathBuf::from("/Users/lixu/code/typster/src-tauri/data/config.toml");
        let config = Config::load_from_file(config_path);
        assert_eq!(config.compiler.input, PathBuf::from("input.typ"));
        assert_eq!(config.compiler.output, PathBuf::from("output.pdf"));
        assert_eq!(config.compiler.optimize, true);
        assert_eq!(config.compiler.pages, Some(vec!["1-".parse().unwrap()]));
    }

    #[test]
    fn test_save_to_file() {
        let config_path = PathBuf::from("/Users/lixu/code/typster/src-tauri/data/test_config.toml");

        let config = Config {
            compiler: CompilerConfig {
                input: PathBuf::from("input.typ"),
                output: PathBuf::from("output.pdf"),
                format: Some(OutputFormat::Pdf),
                pages: Some(vec!["1-".parse().unwrap()]),
                pdf_standard: vec![String::from("1.7")],
                ppi: 144.0,
                make_deps: PathBuf::from("Makefile"),
                open: false,
                timings: PathBuf::from("timings.json"),
                optimize: true,
            },
            world: WorldConfig {
                root: PathBuf::from("."),
                inputs: vec![String::from("key=value")],
                creation_timestamp: String::from(""),
            },
            package: PackageConfig {
                package_path: PathBuf::from("./packages"),
                package_cache_path: PathBuf::from("./cache"),
            },
            init: InitConfig {
                template: String::from("default"),
                dir: PathBuf::from("new_project"),
            },
            query: QueryConfig {
                input: PathBuf::from("input.typ"),
                selector: String::from("title"),
                field: String::from("text"),
                one: true,
                format: String::from("json"),
                pretty: true,
            },
            process: ProcessConfig {
                jobs: 4,
                features: vec![String::from("html")],
                diagnostic_format: String::from("short"),
            },
            fonts: FontsConfig {
                font_paths: vec![PathBuf::from("./fonts")],
                ignore_system_fonts: false,
            },
            cert: Some(PathBuf::from("cert.pem")), // 添加 cert 配置项
        };

        config.save_to_file(config_path.clone());

        let loaded_config = Config::load_from_file(config_path);
        assert_eq!(loaded_config.compiler.input, PathBuf::from("input.typ"));
        assert_eq!(loaded_config.compiler.output, PathBuf::from("output.pdf"));
        assert_eq!(loaded_config.compiler.optimize, true);
        assert_eq!(loaded_config.compiler.pages, Some(vec!["1-".parse().unwrap()]));
    }
}
