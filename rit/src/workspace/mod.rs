pub mod tree;
pub use tree::*;

pub mod stat;
pub use stat::*;

pub mod file;
pub use file::*;

pub mod ignore;
pub use ignore::*;

use std::{
    path::{PathBuf, Path},
    collections::HashMap,
};
use crate::{
    revision::{IntoRev, Rev},
    fs,
};
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    path: PathBuf,
    ignore: Ignore,
}
impl Workspace {
    pub fn build(path: PathBuf) -> crate::Result<Self> {
        if !path.exists() {
            return Err(crate::Error::Workspace("workspace not found".into()));
        }
        let ignore = Ignore::build(path.clone())?;

        let ws = Self {
            path,
            ignore,
        };
        Ok(ws)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn get_ancestors(&self, index: &Path) -> crate::Result<Vec<String>> {
        //let relative_path = self.get_relative_path(path)?;
        let mut ancestors = index.ancestors()
            .collect::<Vec<_>>();
        ancestors.pop();
        let ancestors = ancestors
            .into_iter()
            .map(|p| fs::get_file_name(p))
            .collect::<crate::Result<Vec<_>>>()?;
        Ok(ancestors)
    }

    pub fn get_relative_path(&self, path: &Path) -> crate::Result<PathBuf> {
        path.strip_prefix(&self.path)
            .map(|p| p.to_path_buf())
            .map_err(|e| {
                let f = format!("{:?}: {}", path, e);
                crate::Error::Workspace(f)
            })
    }

    pub fn list_files(&self, dir: &Path, rev: &mut HashMap<PathBuf, Box<dyn Stat>>) -> crate::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry.map_err(Into::<crate::Error>::into)?;

            if self.ignore.is_ignored(entry.file_name().to_str().unwrap()) {
                continue;
            }

            let file_type = entry
                .file_type()
                .map_err(Into::<crate::Error>::into)?;
            
            let entry_path = entry.path();
            if file_type.is_dir() {
                self.list_files(&entry_path, rev)?;
            } else {
                let file = Box::new(File::build(&entry_path)?);
                let relative_path = self.get_relative_path(&entry_path)?;
                let _ = rev.insert(relative_path, file);
            }
        }

        Ok(())
    }
}

impl IntoRev for Workspace {
    fn into_rev(&self) -> crate::Result<Rev> {
        let mut rev = HashMap::new();
        self.list_files(&self.path, &mut rev)?;
        Ok(Rev::new(rev))
    }
}
