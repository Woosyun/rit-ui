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

use rit::{
    self,
    commands,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    path: PathBuf,
    status: EntryStatus,
}
impl Entry {
    pub fn new(path: PathBuf, status: EntryStatus) -> Self {
        Self {
            path,
            status,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntryStatus {
    Added,
    Modified,
    NotChanged,
}

#[tauri::command] 
async fn read_workspace(wd: tauri::State::<'_, Mutex<WorkingDirectory>>) -> rit::Result<Vec<Entry>> {
    let wd = wd.lock().unwrap();
    let wd = (*wd).0.clone().unwrap();
    let status = commands::Status::build(wd)?;
        //.map_err(|e| e.to_string())?;

    let ws_rev = status.scan_workspace()?;
    //    .map_err(|e| e.to_string())?;
    let head_rev = status.scan_head()?;
        //.map_err(|e| e.to_string())?;

    let rev_diff = head_rev.diff(&ws_rev)?;
        //.map_err(|e| e.to_string())?;

    let ws = ws_rev.0.into_iter()
        .map(|(index, _)| {
            let entry = if rev_diff.added.get(&index).is_some() {
                Entry::new(index, EntryStatus::Added)
            } else if rev_diff.modified.get(&index).is_some() {
                Entry::new(index, EntryStatus::Modified)
            } else {
                Entry::new(index, EntryStatus::NotChanged)
            };

            entry
        })
        .collect::<Vec<Entry>>();
    
    Ok(ws)
}

pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(WorkingDirectory(None)))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_working_directory, read_workspace])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
