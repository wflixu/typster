use std::collections::{HashMap, HashSet};
use std::io;
use std::iter;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use codespan_reporting::term::termcolor::WriteColor;
use codespan_reporting::term::{self, termcolor};
use ecow::eco_format;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher as _};
use typst::diag::{bail, StrResult};
use typst::utils::format_duration;

// 删除无效的 use 语句
// use super::args::Output;
use super::compiler::CompileConfig;
use super::timings::Timer;
use super::world::{SystemWorld, WorldCreationError};

pub enum Status {
    Compiling,
    Success(std::time::Duration),
    PartialSuccess(std::time::Duration),
    Error,
}

impl Status {
    /// Log the status message.
    pub fn print(&self, config: &CompileConfig) -> io::Result<()> {
        let timestamp = chrono::offset::Local::now().format("%H:%M:%S");
        let message = format!("watching {}", config.input.display());

        log::info!("{}", message);
        log::info!("writing to {}", config.output.display());
        log::info!("[{timestamp}] {}", self.message());

        Ok(())
    }

    fn message(&self) -> String {
        match *self {
            Self::Compiling => "compiling ...".into(),
            Self::Success(duration) => {
                format!("compiled successfully in {}", format_duration(duration))
            }
            Self::PartialSuccess(duration) => {
                format!("compiled with warnings in {}", format_duration(duration))
            }
            Self::Error => "compiled with errors".into(),
        }
    }
}
