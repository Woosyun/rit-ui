use crate::{
    prelude::*,
    repository::Commit,
};
use std::{
    path::PathBuf,
    fmt::Write,
};

pub struct Log {
    repo: Repository,
}
impl Log {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            repo,
        })
    }

    pub fn execute(&self, branch: &str) -> Result<Vec<(Oid, repository::Commit)>> {
        let mut result = Vec::new();

        let mut leaf = Some(self.repo.refs.get(branch)?);
        while !leaf.is_none() {
            let oid = leaf.unwrap();
            let commit: repository::Commit = self.repo.db.retrieve(&oid)?;
            result.push((oid, commit.clone()));

            leaf = commit.parent().clone();
        }

        Ok(result)
    }

    pub fn node(&self, branch: &str) -> Result<Node> {
        let oid = self.repo.refs.get(branch)?;
        let commit: Commit = self.repo.db.retrieve(&oid)?;
        let node = Node {
            repo: &self.repo,
            oid,
            commit,
        };

        Ok(node)
    }
}

pub struct Node<'a> {
    repo: &'a Repository,
    oid: Oid,
    commit: Commit,
}
impl<'a> Node<'a> {
    pub fn commit(&self) -> &Commit {
        &self.commit
    }
    pub fn oid(&self) -> &Oid {
        &self.oid
    }
}
impl<'a> Iterator for Node<'a> {
    type Item = Node<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(oid) = self.commit.parent() {
            let parent: Commit = self.repo.db.retrieve(&oid)
                .unwrap_or_else(|_| None)?;
            let parent_node= Node {
                repo: self.repo,
                oid: oid.to_owned(),
                commit: parent,
            };
            Some(parent_node)
        } else {
            None
        }
    }
}