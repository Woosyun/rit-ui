use std::{
    path::PathBuf,
    sync::Mutex,
};
use tauri_plugin_dialog::DialogExt;
use tauri::Manager;
use crate::WorkingDirectory;
use serde::{Serialize, Deserialize};


#[tauri::command]
pub async fn set_working_directory(app: tauri::AppHandle) -> Option<PathBuf> {
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
pub async fn read_workspace(
    wd: tauri::State::<'_, Mutex<WorkingDirectory>>,
) -> rit::Result<Vec<Entry>> {
    use rit::prelude::*;

    let wd = wd.lock().unwrap();
    let wd = (*wd).0.clone().unwrap();

    let ws = Workspace::build(wd)?;
    let ws_rev = ws.into_rev()?;
    let repo_rev = Repository::build(&ws)?
        .into_rev()?;

    let rev_diff = repo_rev.diff(&ws_rev)?;
        //.map_err(|e| e.to_string())?;

    let ws = ws_rev.into_iter()
        .map(|(index, _)| {
            let entry = if rev_diff.added.contains(&index) {
                Entry::new(index, EntryStatus::Added)
            } else if rev_diff.modified.contains(&index) {
                Entry::new(index, EntryStatus::Modified)
            } else {
                Entry::new(index, EntryStatus::NotChanged)
            };

            entry
        })
        .collect::<Vec<Entry>>();
    
    Ok(ws)
}
