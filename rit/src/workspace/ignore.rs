use serde::{Serialize, Deserialize};
use std::{
    path::PathBuf,
    collections::HashSet,
};
use crate::{
    utils,
    fs,
    repository::Repository,
};


// .ignore file should be at workspace
// so ignore file can be stored in each revision

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Ignore(HashSet<String>);
impl Ignore {
    pub fn name() -> &'static str {
        ".ignore"
    }
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let mut path = workdir;
        path.push(Ignore::name());
        if !path.exists() {
            return Ok(Ignore::default());
        }

        let content = fs::read_to_string(&path)?;
        let ignore: Ignore = utils::encode(&content)
            .map_err(|s| crate::Error::Repository(s))?;
        Ok(ignore)
    }
    pub fn add(repo: PathBuf, names: Vec<String>) -> crate::Result<()> {
        let mut path = repo;
        path.push(Ignore::name());
        let mut ignore = if !path.exists() {
            Ignore::default()
        } else {
            let content = fs::read_to_string(&path)?;
            utils::encode(&content)
                .map_err(|s| crate::Error::Repository(s))?
        };

        for name in names {
            ignore.0.insert(name);
        }

        let content = utils::decode(&ignore)
            .map_err(|s| crate::Error::Repository(s))?;
        fs::write(&path, &content)
    }
    pub fn is_ignored(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}
impl Default for Ignore {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(Repository::name().to_string());

        Self(set)
    }
}
