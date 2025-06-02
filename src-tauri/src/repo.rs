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

    let repo_status = rit::commands::Status::build(wd)?
        .repository_status();
    Ok(repo_status.is_repository_initialized())
}

#[tauri::command] 
pub async fn initialize_repository(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<()> {
    let wd = wd.lock().unwrap();
    let wd = (*wd).0.clone().unwrap();

    let init = commands::Init::build(wd)?;
    init.execute()
}

#[derive(Serialize, Deserialize)]
pub struct History(HashMap<String, Revision>);

#[derive(Serialize, Deserialize)]
struct Revision {
    parents: HashSet<String>,
    children: HashSet<String>,
}

#[tauri::command]
pub async fn read_entire_history(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<History> {
    let wd = {
        let wd_lock = wd.lock().unwrap();
        (*wd_lock).0.clone().unwrap()
    };
    let log = commands::Log::build(wd)?;

    let mut history = HashMap::new();

    for leaf_node in log.read_all_branches()? {
        if history.contains_key(&leaf_node.oid().to_string()) {
            continue;
        }
        history.insert(leaf_node.oid().to_string(), Revision {
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        for node in leaf_node {
            let child_oid = node.oid().to_string();
            if history.contains_key(&child_oid) {
                break;
            }

            if let Some(parent_oid) = node.commit().parent() {
                let parent_oid = parent_oid.to_string();

                history
                    .entry(parent_oid.clone())
                    .or_insert_with(|| Revision {
                        parents: HashSet::new(),
                        children: HashSet::new(),
                    })
                    .children.insert(child_oid.clone());

                history
                    .entry(child_oid)
                    .or_insert_with(|| Revision {
                        parents: HashSet::new(),
                        children: HashSet::new(),
                    })
                    .parents.insert(parent_oid);
            }
        }
    }

    Ok(History(history))
}

#[tauri::command]
pub async fn get_head(wd: tauri::State<'_, Mutex<WorkingDirectory>>) -> rit::Result<Option<String>> {
    let wd = {
        let wd_lock = wd.lock().unwrap();
        (*wd_lock).0.clone().unwrap()
    };
    let ws = rit::workspace::Workspace::build(wd)?;
    let repo = rit::repository::Repository::build(&ws)?;
    let head = repo.local_head.get()?;
    let branch = head.branch()?;
    let oid = if repo.refs.contains(branch) {
        Some(repo.refs.get(branch)?.to_string())
    } else {
        None
    };

    Ok(oid)
}
