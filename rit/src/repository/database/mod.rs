pub mod tree;
pub use tree::*;

pub mod blob;
pub use blob::*;

pub mod entry;
pub use entry::*;

pub mod oid;
pub use oid::*;

pub mod commit;
pub use commit::*;


use std::path::PathBuf;
use crate::{
    utils,
    fs,
};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    path: PathBuf
}
impl Database {
    pub fn name() -> &'static str {
        "objects"
    }

    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(Database::name());
        if !path.exists() {
            return Err(crate::Error::Repository(".rit/objects not found".into()));
        }

        let db = Self {
            path
        };
        Ok(db)
    }

    pub fn init(repo: PathBuf) -> crate::Result<()> {
        let mut path = repo;
        path.push(Database::name());
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        Ok(())
    }

    pub fn store<O: Serialize>(&self, o: &O) -> crate::Result<Oid> {
        let content = utils::decode(o)
            .map_err(|s| crate::Error::Repository(s))?;
        let oid = Oid::build(&content);

        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(dir);
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        path.push(file);
        if path.exists() {
            return Ok(oid);
        }

        fs::lock_write(&path, &content)?;
        Ok(oid)
    }

    pub fn retrieve<O: DeserializeOwned>(&self, oid: &Oid) -> crate::Result<O> {
        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(dir);
        path.push(file);
        if !path.exists() {
            return Err(crate::Error::Repository("such object not found".into()));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| crate::Error::Repository(e.to_string()))?;
        let obj: O = utils::encode(&content)
                    .map_err(|s| crate::Error::Repository(s))?;
        Ok(obj)
    }

    pub fn get_oid<O: Serialize>(&self, o: &O) -> crate::Result<Oid> {
        let content = utils::decode(o)
            .map_err(|s| crate::Error::Repository(s))?;
        let oid = Oid::build(&content);
        Ok(oid)
    }
}
