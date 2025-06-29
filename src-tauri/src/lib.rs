mod commands;
pub use commands::*;

use std::{
    sync::Mutex,
    path::PathBuf,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingDirectory(pub Option<PathBuf>);
pub fn workdir(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>
) -> PathBuf {
    let wd_lock = wd.lock().unwrap();
    (*wd_lock).0.clone().unwrap()
}

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(WorkingDirectory(None)))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            is_repository_initialized,
            initialize_repository,
            read_workspace,
            set_working_directory,
            get_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
