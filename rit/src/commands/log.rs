use crate::prelude::*;
use std::{
    path::PathBuf,
    fmt::Write,
};

pub struct Log {
    repo: Repository,
    pub history: Vec<(Oid, repository::Commit)>
}
impl Log {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            repo,
            history: Vec::new(),
        })
    }

    pub fn execute(&mut self, branch: &str) -> Result<Vec<(Oid, repository::Commit)>> {
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

    pub fn print(&self) {
        let mut output = String::new();
        for (oid, commit) in self.history.iter() {
            writeln!(&mut output, "{}\n{}", oid, commit.message()).unwrap();
        }
        println!("{}", output);
    }
}
