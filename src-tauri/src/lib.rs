mod ws;
mod repo;

use std::sync::Mutex;

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(ws::WorkingDirectory(None)))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ws::set_working_directory, 
            ws::read_workspace,
            repo::is_repository_initialized,
            repo::initialize_repository,
            repo::read_entire_history,
            repo::get_head,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
