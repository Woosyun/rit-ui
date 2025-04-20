use crate::{
    prelude::*,
    fs,
};
use std::{
    path::{PathBuf, Path},
};

pub struct Commit {
    ws: Workspace,
    repo: Repository,
}
impl Commit {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        let commit = Self {
            ws,
            repo,
        };

        Ok(commit)
    }

    pub fn execute(&self, message: String) -> crate::Result<()> {
        let head = self.repo.local_head.get()?;
        //
        // if head is not tip of branch, cannot commit
        if !head.is_branch() {
            return Err(Error::Repository("cannot run commit on non-branch head".into()));
        }

        let branch = head.branch()?;
        let parent = self.repo.refs.get(branch)?;
        // 1. read revisions
        let prev_revision = Revision::build(self.repo.clone(), &parent)?;
        let mut prev_rev = prev_revision.into_rev()?;
        let mut curr_rev = self.ws.into_rev()?;

        let rev_diff = prev_rev.diff(&curr_rev)?;
        if rev_diff.is_clean() {
            return Err(crate::Error::Workspace("Workspace is clean. Nothing to commit".into()));
        }

        // 2. store Blobs and update oid for entry
        let mut store_and_update = |index: &Path| -> crate::Result<()> {
            let path = self.ws.path().join(&index);
            let content = fs::read_to_string(&path)?;
            let blob = Blob::new(content);
            let oid = self.repo.db.store(&blob)?;

            let file = curr_rev.0.get_mut(index).unwrap();
            file.set_oid(oid);

            let entry = repository::Entry::build(file.as_ref())?;
            prev_rev.0.insert(index.to_path_buf(), Box::new(entry));
            Ok(())
        };
        for index in rev_diff.added.iter() {
            store_and_update(index)?;
        }
        for index in rev_diff.modified.iter() {
            store_and_update(index)?;
        }
        for index in rev_diff.removed.iter() {
            prev_rev.0.remove(index).unwrap();
        }

        // 3. store tree and update oid for entry
        let mut ws_tree = workspace::Tree::new("".into());
        for (path, entry) in prev_rev.0 {
            let mut ancestors = self.ws.get_ancestors(&path)?;
            ws_tree.add_entry(&mut ancestors, entry);
        }
        
        let f = |tree: &mut workspace::Tree| -> crate::Result<()> {
            let repo_tree = tree
                .entries
                .iter()
                .map(|(_, tree_entry)| {
                    match tree_entry {
                        workspace::Entry::Tree(tree) => repository::Entry::build(tree),
                        workspace::Entry::Entry(entry) => repository::Entry::build(entry.as_ref()),
                    }
                })
                .collect::<crate::Result<Vec<_>>>()?;
            let oid = self.repo.db.store(&repo_tree)?;
            tree.set_oid(oid);

            Ok(())
        };
        ws_tree.traverse_mut(f)?;

        // 4. store commit and update head
        let root = ws_tree.oid()?.clone();
        let commit = repository::Commit::new(Some(parent), root, message);
        let new_head = self.repo.db.store(&commit)?;

        self.repo.refs.set(branch, &new_head)?;

        Ok(())
    }
}
