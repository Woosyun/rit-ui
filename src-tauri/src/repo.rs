use super::ws::WorkingDirectory;
use rit::{
    self,
    commands,
};
use std::{
    sync::Mutex,
    collections::{HashMap, HashSet},
};
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
pub struct History(HashMap<String, Commit>);

#[derive(Serialize, Deserialize)]
struct Commit {
    parents: HashSet<String>,
    children: HashSet<String>,
}

#[tauri::command]
pub async fn read_entire_history(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<History> {
    let wd = {
        let wd_lock = wd.lock().unwrap();
        (*wd_lock).0.clone().unwrap()
    };

    let cmd_branch = commands::Branch::build(wd.clone())?;
    let cmd_log = commands::Log::build(wd)?;

    let mut history = HashMap::new();

    for branch in cmd_branch.list()? {
        let mut current = Some(cmd_log.node(&branch)?);
        history.insert(branch, Commit{
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        for node in current {
            if history.get(node.oid()).is_some() {
                break;
            }

            if let Some(parent_oid) = node.commit().parent() {
                let parent_oid = parent_oid.to_string();
                let child_oid = node.oid().to_string();

                history
                    .entry(parent_oid.clone())
                    .or_insert_with(|| Commit {
                        parents: HashSet::new(),
                        children: HashSet::new(),
                    })
                    .children.insert(child_oid.clone());

                history
                    .entry(child_oid)
                    .or_insert_with(|| Commit {
                        parents: HashSet::new(),
                        children: HashSet::new(),
                    })
                    .parents.insert(parent_oid);
            }
        }
    }

    let history = History(history);
    Ok(history)
}
