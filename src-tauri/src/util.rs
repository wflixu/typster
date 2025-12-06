use std::io;
use typst::diag::FileError;
use std::process::ExitCode;
use std::cell::Cell;
use tauri::plugin::{Plugin, PluginApi};

thread_local! {
    /// The CLI's exit code.
    static EXIT: Cell<ExitCode> = const { Cell::new(ExitCode::SUCCESS) };
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("unknown error")]
    Unknown,
    #[error("unknown world")]
    UnknownWorld,
    #[error("io error occurred")]
    IO(#[from] io::Error),
    #[error("typst file error occurred")]
    TypstFile(#[from] FileError),
    #[error("the provided path does not belong to the world")]
    UnrelatedPath,
}

// 手动实现 Serialize，因为 thiserror 不自动提供
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        // 将错误序列化为字符串
        serializer.serialize_str(&self.to_string())
    }
}



/// Ensure a failure exit code.
pub fn set_failed() {
    EXIT.with(|cell| cell.set(ExitCode::FAILURE));
}
