use serde::{Serialize, Deserialize};
use tauri_plugin_dialog::DialogExt;
use std::{
    path::PathBuf,
    sync::Mutex,
};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug)]
struct WorkingDirectory(pub Option<PathBuf>);

#[tauri::command]
async fn set_working_directory(app: tauri::AppHandle) -> Option<PathBuf> {
    let result = app.dialog()
        .file()
        .blocking_pick_folder();

    let result = result.map(|file_path| {
        file_path.into_path()
            .unwrap()
    });

    let cwd = app.state::<Mutex<WorkingDirectory>>();
    let mut cwd = cwd.lock().unwrap();
    if result.is_some() {
        cwd.0 = result;
    }

    cwd.0.clone()
}

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(WorkingDirectory(None)))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_working_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
