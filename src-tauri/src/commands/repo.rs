use rit::{
    prelude::*,
    commands::*,
};
use std::sync::Mutex;
use crate::{WorkingDirectory, workdir};


#[tauri::command]
pub async fn initialize_repository(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>
) -> Result<()> {
    let workdir = workdir(wd);
    let cmd = init::Init::build(workdir)?;
    cmd.execute()
}

#[tauri::command]
pub async fn is_repository_initialized(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>
) -> Result<bool> {
    let workdir = workdir(wd);
    let cmd = status::Status::build(workdir)?;
    Ok(cmd.repository_status()
        .is_repository_initialized())
}

#[tauri::command]
pub async fn get_history(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>
) -> Result<history::HistoryGraph> {
    let workdir = workdir(wd);
    let cmd = history::History::build(workdir)?;
    cmd.read_full()
}

#[tauri::command]
pub async fn get_head(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>
) -> Result<Option<Oid>> {
    let workdir = workdir(wd);
    let ws = Workspace::build(workdir)?;
    let repo = Repository::build(&ws)?;
    repo.read_head()
}
