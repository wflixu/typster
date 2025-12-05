use std::path::PathBuf;
use tauri::command;
use crate::util::AppError;

#[command]
pub async fn fs_read_file_text(path: String) -> Result<String, AppError> {
    use std::fs;
    use std::io::Read;

    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err(AppError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display())
        )));
    }

    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

#[command]
pub async fn fs_write_file_text(path: String, content: String) -> Result<(), AppError> {
    use std::fs;

    let path = PathBuf::from(&path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    Ok(())
}

#[command]
pub async fn fs_read_file_binary(path: String) -> Result<Vec<u8>, AppError> {
    use std::fs;
    use std::io::Read;

    let path = PathBuf::from(&path);

    if !path.exists() {
        return Err(AppError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display())
        )));
    }

    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[command]
pub async fn fs_write_file_binary(path: String, content: Vec<u8>) -> Result<(), AppError> {
    use std::fs;

    let path = PathBuf::from(&path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    Ok(())
}

#[command]
pub async fn fs_create_file(path: String) -> Result<(), AppError> {
    use std::fs;

    let path = PathBuf::from(&path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::File::create(path)?;

    Ok(())
}

#[command]
pub async fn fs_list_dir(path: String) -> Result<Vec<String>, AppError> {
    use std::fs;

    let path = PathBuf::from(&path);

    if !path.exists() || !path.is_dir() {
        return Err(AppError::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", path.display())
        )));
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(path_str) = path.to_str() {
            entries.push(path_str.to_string());
        }
    }

    entries.sort();

    Ok(entries)
}