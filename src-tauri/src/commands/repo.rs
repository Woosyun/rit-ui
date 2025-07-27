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
    let mut repo = workdir(wd);
    repo.push(".rit");
    Ok(repo.exists())
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
) -> Result<Head> {
    let workdir = workdir(wd);
    let ws = Workspace::build(workdir)?;
    let repo = Repository::build(&ws)?;
    repo.local_head.get()
}

#[tauri::command]
pub async fn commit(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>,
    msg: String,
) -> Result<()> {
    let workdir = workdir(wd);
    let mut cmd = commit::Commit::build(workdir)?;
    cmd.set_message(msg);
    cmd.execute()
}

#[tauri::command]
pub async fn create_branch(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>,
    new_branch: String,
) -> Result<()> {
    let workdir = workdir(wd);
    let cmd = branch::Branch::build(workdir)?;
    cmd.create(&new_branch)
}

#[tauri::command]
pub async fn checkout_to_revision(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>,
    oid: Oid,
) -> Result<()> {
    let workdir = workdir(wd);
    let mut cmd = checkout::Checkout::build(workdir)?;
    cmd.set_target_to_oid(oid);
    cmd.execute()
}

#[tauri::command]
pub async fn checkout_to_branch(
    wd: tauri::State<'_, Mutex<WorkingDirectory>>,
    branch: String,
) -> Result<()> {
    let workdir = workdir(wd);
    let mut cmd = checkout::Checkout::build(workdir)?;
    cmd.set_target_to_branch(branch);
    cmd.execute()
}
