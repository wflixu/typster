extern crate mdxjs;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn compileMdx(text: &str) -> String {
    let result = match mdxjs::compile(text, &Default::default()) {
        Ok(res) => {
            println!("{}", &res);
            res
        }
        Err(err) => "rtt".to_string(),
    };

    result
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn compileToHtml(text: &str) -> String {
    let result = match markdown::to_html_with_options(text, &markdown::Options::gfm()) {
        Ok(res) => {
            println!("{}", &res);
            res
        }
        Err(err) => "rtt".to_string(),
    };

    result
}
