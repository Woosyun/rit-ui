use super::ws::WorkingDirectory;
use rit::{
    self,
    commands,
};
use std::sync::Mutex;

#[tauri::command]
pub async fn is_repository_initialized(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<bool> {
    let wd = wd.lock().unwrap();
    let wd = (*wd).0.clone().unwrap();

    let status = commands::Status::scan(wd)?;
    Ok(status.is_repository_initialized())
}

#[tauri::command] 
pub async fn initialize_repository(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<()> {
    let wd = wd.lock().unwrap();
    let wd = (*wd).0.clone().unwrap();

    let init = commands::Init::build(wd)?;
    init.execute()
}