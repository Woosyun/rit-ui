use crate::prelude::*;
use std::{
    path::PathBuf,
    collections::HashSet,
};

pub struct Branch {
    repo: Repository,
}
impl Branch {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        Ok(Self {
            repo
        })
    }

    // create new branch and copy-paste head
    pub fn create(&self, new_branch: &str) -> Result<()> {
        if self.repo.refs.contains(new_branch) {
            return Err(Error::Repository("branch already exists. Not supported yet".into()));
        }

        // todo:
        // !!! what if repository is empty(= head is None)?
        // Current Refs does not have ability to handle None type.
        // Only Oid can be stored by Refs.
        // => repository is never empty. == always there is head pointing a revision
        let head = self.repo.local_head.get()?;
        let oid = if head.is_branch() {
            &self.repo.refs.get(head.branch()?)?
        } else {
            head.oid()?
        };
        self.repo.refs.set(new_branch, oid)?;

        Ok(())
    }

    pub fn list(&self) -> Result<HashSet<String>> {
        let branches = self.repo.refs.list_branches()?;

        Ok(branches)
    }
}
