// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet2(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
pub async fn load_doc_from_path<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: String,
) -> std::result::Result<(), Error> {
    let path_buf = PathBuf::from(&path);
    let project = Arc::new(Project::load_from_path(path_buf));
    project_manager.set_project(&window, Some(project));
    info!("succeed load_project_from_path {}", &path);
    
    Ok(())
}