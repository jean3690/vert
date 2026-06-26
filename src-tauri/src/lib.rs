pub mod commands;
pub mod converter;
pub mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::convert_file,
            commands::get_valid_targets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
