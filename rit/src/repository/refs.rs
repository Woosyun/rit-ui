use std::{
    path::PathBuf,
    collections::HashSet,
};
use crate::{
    prelude::*,
    utils,
    fs,
};
use serde::{Serialize, Deserialize};

const REFS: &str = "refs";
const LOCAL: &str = "local";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(REFS);
        if !path.exists() {
            return Err(crate::Error::Repository("refs not found".into()));
        }

        Ok(Self {
            path 
        })
    }
    pub fn init(repo: PathBuf, oid: &Oid) -> crate::Result<()> {
        let mut path = repo;
        path.push(REFS);
        let refs = Self {
            path: path.clone()
        };

        path.push(LOCAL);
        fs::create_dir_all(&path)?;

        refs.set("main", oid)
    }

    pub fn contains(&self, branch: &str) -> bool {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);
        path.exists()
    }

    pub fn list_branches(&self) -> Result<HashSet<String>> {
        let mut path = self.path.clone();
        path.push(LOCAL);

        let mut result = HashSet::new();
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let branch = entry.file_name()
                .to_str().unwrap()
                .to_string();
            result.insert(branch);
        }

        Ok(result)
    }

    pub fn get(&self, branch: &str) -> crate::Result<Oid> {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);
        if !path.exists() {
            let msg = format!("branch {} not found", branch);
            return Err(crate::Error::Repository(msg));
        }

        let latest_commit: Oid = fs::read_to_string(&path)
            .map(|content| {
                utils::encode(&content)
                    .map_err(|e| crate::Error::Repository(e))
            })??;

        Ok(latest_commit)
    }

    pub fn set(&self, branch: &str, oid: &Oid) -> crate::Result<()> {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);

        let oid = utils::decode(oid)
            .map_err(|e| crate::Error::Repository(e))?;
        fs::lock_write(&path, &oid)?;
        Ok(())
    }
}
