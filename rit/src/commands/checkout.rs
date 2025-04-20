use crate::{
    prelude::*,
    fs,
};
use std::path::{PathBuf, Path};

pub struct Checkout {
    ws: Workspace,
    repo: Repository,
}

impl Checkout {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        let re = Self {
            ws,
            repo,
        };
        Ok(re)
    }
    
    pub fn execute(&self, branch: &str) -> crate::Result<()> {
        //if target revision is based on same commit,
        //and just different on branch name,
        //move head and return
        let target_oid = self.repo.refs.get(branch)?;

        let head = self.repo.local_head.get()?;
        let prev_oid = if head.is_branch() {
            let branch = head.branch()?;
            let prev_oid = self.repo.refs.get(branch)?;
            if prev_oid == target_oid {
                self.repo.local_head.set_to_branch(branch)?;
                return Ok(());
            }

            prev_oid
        } else {
            head.oid()?.to_owned()
        };
        let prev_rev = Revision::build(self.repo.clone(), &prev_oid)?;
        let prev_rev = prev_rev.into_rev()?;

        let curr_rev = self.ws.into_rev()?;

        let rev_diff_for_check = prev_rev.diff(&curr_rev)?;
        if !rev_diff_for_check.is_clean() {
            return Err(crate::Error::Workspace("workspace is not clean. cannot use checkout".into()));
        }

        let target_rev = Revision::build(self.repo.clone(), &target_oid)?;
        let target_rev = target_rev.into_rev()?;
        let rev_diff = curr_rev.diff(&target_rev)?;

        //modify workspace
        let insert_to_workspace = |index: &Path| -> crate::Result<()> {
            let oid = target_rev.0.get(index).unwrap().oid()?;
            let blob: Blob = self.repo.db.retrieve(oid)?;
            let path = self.ws.path().join(index);
            fs::write(&path, blob.content())?;

            Ok(())
        };
        for a in rev_diff.added.iter() {
            insert_to_workspace(a)?;
        }
        for m in rev_diff.modified.iter() {
            insert_to_workspace(m)?;
        }
        for r in rev_diff.removed.iter() {
            let path = self.ws.path().join(r);
            fs::remove_file(&path)?;
        }

        //clear empty directories

        //update head
        self.repo.local_head.set_to_oid(&target_oid)?;

        Ok(())
    }
}
