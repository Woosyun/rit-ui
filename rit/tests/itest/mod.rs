#![allow(unused)]

mod fs;
mod utils;

use rand::prelude::*;
use std::{
    path::{PathBuf, Path},
    collections::HashSet,
    io,
};
use rit::{
    self,
    prelude::*,
};
use tempdir::TempDir;

#[derive(Debug)]
pub struct Client {
    pub tempdir: TempDir,
    pub added: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
    pub removed: HashSet<PathBuf>,
}

impl Client {
    pub fn build(test_name: &str) -> io::Result<Self> {
        let tempdir = TempDir::new(test_name)?;

        Ok(Self {
            tempdir,
            added: HashSet::new(),
            modified: HashSet::new(),
            removed: HashSet::new(),
        })
    }
    pub fn workdir(&self) -> &Path {
        self.tempdir.path()
    }
    pub fn workspace(&self) -> rit::Result<Workspace> {
        Workspace::build(self.workdir().to_path_buf())
    }
    pub fn repository(&self) -> rit::Result<Repository> {
        Repository::build(&self.workspace()?)
    }

    pub fn init(&self) -> rit::Result<()> {
        let ws = self.workspace()?;
        Repository::init(&ws)
    }

    pub fn work(&mut self) -> rit::Result<()> {
        utils::sleep_1_sec();

        let ws = self.workspace()?;
        let curr_rev = ws.into_rev()
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e.to_string()))?;

        let mut files = curr_rev.0.keys().cloned().collect::<Vec<_>>();
        let mut rng = rand::rng();
        files.shuffle(&mut rng);

        let number_of_files = files.len();

        //remove
        let number_of_deletion = number_of_files/3;
        for file in files.iter().take(number_of_deletion) {
            self.removed.insert(file.to_path_buf());

            let path = self.workdir().join(file);
            if path.exists() {
                fs::remove_file(&path)?;
            }
        }

        //modify
        let remaining_files = files
            .iter()
            .skip(number_of_deletion)
            .cloned()
            .collect::<Vec<_>>();
        let number_of_modification = number_of_files/3;
        for file in remaining_files.iter().take(number_of_modification) {
            self.modified.insert(file.to_path_buf());

            let path = self.workdir().join(&file);
            fs::appendln(&path, "\n//modified for integration testing")?;
        }

        //add
        let number_of_creation = if number_of_files < 10 {
            10
        } else {
            number_of_files/3
        };
        for i in 0..number_of_creation {
            let new_file = format!("new_file_{}_{}.txt", i, rng.random::<u32>());
            self.added.insert(Path::new(&new_file).to_path_buf());

            let path = self.workdir().join(new_file);
            if !path.exists() {
                fs::write(&path, "newly created for integration testing")?;
            }
        }

        Ok(())
    }

    fn commit(&self) -> rit::Result<()> {
        let cmd = rit::commands::Commit::build(self.workdir().to_path_buf())?;
        let message = format!("commit-{}", rand::rng().random::<u32>());
        cmd.execute(message)
    }
    pub fn try_commit(&mut self) -> rit::Result<()> {
        self.commit()?;

        //check deletion worked
        for file in self.removed.iter() {
            let path = self.workdir().join(file);
            if path.exists() {
                let f = format!("{:?} not removed", file);
                return Err(rit::Error::Workspace(f));
            }
        }

        //check addition/modification worked
        let repo = self.repository()?;
        let compare_blobs = |file: &Path| -> rit::Result<()> {
            let path = self.workdir().join(file);
            let blob_ws = Blob::new(fs::read_to_string(&path)?);
            let json = decode(&blob_ws).unwrap();
            let oid = Oid::build(&json);
            let blob_db: Blob = repo.db.retrieve(&oid)?;

            assert_eq!(blob_ws, blob_db);

            Ok(())
        };
        for file in self.added.iter() {
            compare_blobs(file)?;
        }
        for file in self.modified.iter() {
            compare_blobs(file)?;
        }

        self.added.clear();
        self.modified.clear();
        self.removed.clear();

        Ok(())
    }

    pub fn try_status(&self) -> Result<()> {
        let status = commands::Status::build(self.workdir().to_path_buf())?;
        let rev_diff = status.execute()?;

        assert_eq!(self.added, rev_diff.added, "comparing added files for one that was recorded and one returned by status command");
        assert_eq!(self.modified, rev_diff.modified, "comparing modified files for one that was recorded and one returned by status command");
        assert_eq!(self.removed, rev_diff.removed, "comparing removed files for one that was recorded and one returned by status command");

        Ok(())
    }

    //일단은 branch 사이에서만 테스트 진행
    pub fn try_checkout(&self, branch: &str) -> Result<()> {
        let repo = self.repository()?;
        let target_oid = repo.refs.get(branch)?;
        let target_rev = Revision::build(repo, &target_oid)?
            .into_rev()?;

        let checkout = commands::Checkout::build(self.workdir().to_path_buf())?;
        checkout.execute(branch)?;

        let repo = self.repository()?;
        let head = repo.local_head.get()?;
        let curr_oid= repo.refs.get(head.branch()?)?;
        let curr_rev = Revision::build(repo, &curr_oid)?
            .into_rev()?;
        let rev_diff = target_rev.diff(&curr_rev)?;
        assert!(rev_diff.is_clean());

        Ok(())
    }

    pub fn try_branch_create(&self, new_branch: &str) -> Result<()> {
        let branch = commands::Branch::build(self.workdir().to_path_buf())?;
        branch.create(new_branch)?;

        Ok(())
    }
}
