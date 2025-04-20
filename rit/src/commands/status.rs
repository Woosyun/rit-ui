use std::path::PathBuf;
use crate::prelude::*;

pub struct Status {
    ws: Workspace,
    repo: Repository,
}
impl Status {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            ws,
            repo,
        })
    }
    
    pub fn execute(&self) -> crate::Result<RevDiff> {
        let prev_rev = self.scan_head()?;
        let curr_rev = self.ws.into_rev()?;

        let rev_diff = prev_rev.diff(&curr_rev)?;
        Ok(rev_diff)
    }

    pub fn scan_workspace(&self) -> Result<Rev> {
        self.ws.into_rev()
    }
    pub fn scan_head(&self) -> Result<Rev> {
        let head = self.repo.local_head.get()?;
        if !head.is_branch() {
            return Err(Error::Repository("cannot scan on non-branch revision yet".into()));
        }
        let branch = head.branch()?;
        let parent = self.repo.refs.get(branch)?;
        let prev_rev = Revision::build(self.repo.clone(), &parent)?;

        prev_rev.into_rev()
    }
}
