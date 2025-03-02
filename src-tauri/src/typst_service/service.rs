use super::args::{CompileArgs, FontsArgs, InitArgs, QueryArgs};
use super::compile;
use super::init;
use super::query;
use super::package::*;
use super::world::*;

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, Deserialize)]
struct Config {
    compile: Option<CompileArgs>,
    init: Option<InitArgs>,
    query: Option<QueryArgs>,
    fonts: Option<FontsArgs>,
}

impl Config {
    fn load() -> Self {
        let global_config = fs::read_to_string("~/config.toml").unwrap_or_default();
        let local_config = fs::read_to_string("./doc_config.toml").unwrap_or_default();

        let global_config: Value = toml::from_str(&global_config).unwrap_or_default();
        let local_config: Value = toml::from_str(&local_config).unwrap_or_default();

        let merged_config = global_config
            .as_table()
            .unwrap()
            .clone()
            .into_iter()
            .chain(local_config.as_table().unwrap().clone())
            .collect::<Value>();

        toml::from_str(&merged_config.to_string()).unwrap_or_default()
    }
}

pub fn compile() {
    let config = Config::load();
    if let Some(args) = config.compile {
        compile::compile(args);
    }
}

pub fn init() {
    let config = Config::load();
    if let Some(args) = config.init {
        init::init(args);
    }
}

pub fn query() {
    let config = Config::load();
    if let Some(args) = config.query {
        query::query(args);
    }
}

pub fn fonts() {
    let config = Config::load();
    if let Some(args) = config.fonts {
        fonts::fonts(args);
    }
}
