pub mod rev;
pub use rev::*;

use crate::{
    repository::{Repository, self, Oid},
    workspace::Stat,
};
use std::{
    collections::HashMap,
    path::{PathBuf, Path},
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Revision {
    repo: Repository,
    commit: repository::Commit
}
impl Revision {
    pub fn build(repo: Repository, oid: &Oid) -> crate::Result<Self> {
        let commit = repo.db.retrieve(oid)?;
        Ok(Self {
            repo,
            commit,
        })
    }
    fn list_entries(&self, base: &Path, tree: &repository::Tree, result: &mut HashMap<PathBuf, Box<dyn Stat>>) -> crate::Result<()> {
        for entry in tree.entries() {
            let new_base = base.join(entry.name());

            if entry.is_dir() {
                let sub_tree = self.repo.db.retrieve(entry.oid())?;
                self.list_entries(&new_base, &sub_tree, result)?;
            } else {
                let _ = result.insert(new_base, Box::new(entry.clone()));
            }
        }

        Ok(())
    }
}

impl IntoRev for Revision {
    fn into_rev(&self) -> crate::Result<Rev> {
        let mut rev = HashMap::new();

        let root = self.commit.root();
        let root_tree = self.repo.db.retrieve(root)?;
        self.list_entries(Path::new(""), &root_tree, &mut rev)?;

        let rev = Rev::new(rev);
        Ok(rev)
    }
}
