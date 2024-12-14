use crate::project::ProjectWorld;
use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::{self,Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};
use std::{fs, io};
use thiserror::Error;
use typst::diag::{FileError, FileResult};
use typst::model::Document;
use typst::syntax::VirtualPath;

const PATH_PROJECT_CONFIG_FILE: &str = ".typster/project.json";

pub struct Project {
    pub root: PathBuf,
    pub world: Mutex<ProjectWorld>,
    pub cache: RwLock<ProjectCache>,
    pub config: RwLock<ProjectConfig>,
}

#[derive(Default)]
pub struct ProjectCache {
    pub document: Option<Document>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct ProjectConfig {
    pub input: Option<PathBuf>,
    pub root: Option<PathBuf>,
    pub main: Option<PathBuf>,
    pub font_paths: Vec<PathBuf>,
    pub ignore_system_fonts: bool,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub diagnostic_format: DiagnosticFormat,
    pub package_path: Option<PathBuf>,
    pub package_cache_path: Option<PathBuf>,
    pub jobs: Option<usize>,
    pub cert: Option<PathBuf>,
}

/// Which format to use for diagnostics.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiagnosticFormat {
    Human,
    Short,
}

impl Display for DiagnosticFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticFormat::Human => write!(f, "human"),
            DiagnosticFormat::Short => write!(f, "short"),
        }
    }
}




#[derive(Error, Debug)]
pub enum ProjectConfigError {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("serial error")]
    Serial(#[from] serde_json::Error),
}

impl ProjectConfig {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<ProjectConfig, ProjectConfigError> {
        let json = fs::read_to_string(path).map_err(Into::<ProjectConfigError>::into)?;
        serde_json::from_str(&json).map_err(Into::into)
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ProjectConfigError> {
        let json = serde_json::to_string(&self).map_err(Into::<ProjectConfigError>::into)?;
        fs::write(path, json).map_err(Into::into)
    }

    pub fn apply(&self, project: &Project) {
        let mut world = project.world.lock().unwrap();
        match self.apply_main(project, &mut world) {
            Ok(_) => debug!(
                "applied main source configuration for project {:?}",
                project
            ),
            Err(e) => debug!(
                "unable to apply main source configuration for project {:?}: {:?}",
                project, e
            ),
        }
    }

    pub fn apply_main(&self, project: &Project, world: &mut ProjectWorld) -> FileResult<()> {
        if let Some(main) = self.main.as_ref() {
            let vpath = VirtualPath::new(main);
            debug!("setting main path {:?} for {:?}", main, project);
            world.set_main_path(vpath);
            return Ok(());
        }

        // ??
        // world.set_main(None);

        Err(FileError::NotSource)
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            input: None,
            root: None,
            main: None,
            font_paths: Vec::new(),
            ignore_system_fonts: false,
            creation_timestamp: Some(Utc::now()),
            diagnostic_format: DiagnosticFormat::Human,
            package_path: None,
            package_cache_path: None,
            jobs: None,
            cert: None,
        }
    }
}

impl Project {
    pub fn load_from_path(path: PathBuf) -> Self {
        let path = fs::canonicalize(&path).unwrap_or(path);
        let config: ProjectConfig = {
            match ProjectConfig::read_from_file(path.join(PATH_PROJECT_CONFIG_FILE)) {
                Ok(config) => config,
                Err(e) => {
                    let mut config = ProjectConfig::default();
                    config.input = Some(path.join("main.typ"));
                    config.root = Some(path.clone());
                    config.main = Some(path.join("main.typ"));
                    config
                }
            }
        };

        Self {
            world: Mutex::new(ProjectWorld::new(path.clone(), config.clone()).expect("failed to create project world")),
            cache: RwLock::new(Default::default()),
            config: RwLock::new(config),
            root: path,
        }
    }
}

impl Debug for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project").field("root", &self.root).finish()
    }
}

pub fn is_project_config_file(relative: &Path) -> bool {
    relative.as_os_str() == PATH_PROJECT_CONFIG_FILE
}
