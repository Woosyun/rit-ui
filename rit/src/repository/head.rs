use std::path::PathBuf;
use crate::{
    prelude::*,
    fs,
    utils
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Head {
    Oid(Oid),
    Branch(String),
}
impl Head {
    pub fn is_branch(&self) -> bool {
        match self {
            Head::Branch(_) => true,
            _ => false,
        }
    }
    pub fn branch(&self) -> Result<&str> {
        match self {
            Head::Branch(branch) => Ok(branch),
            _ => Err(Error::Repository("Head.branch method called on wrong spot".into())),
        }
    }
    pub fn oid(&self) -> Result<&Oid> {
        match self {
            Head::Oid(oid) => Ok(oid),
            _ => Err(Error::Repository("Head.oid method called on wrong spot".into())),
        }
    }
}

const LOCAL_HEAD: &str = "LOCAL_HEAD";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct LocalHead {
    path: PathBuf,
}
impl LocalHead {
    pub fn build(repo: PathBuf) -> Result<Self> {
        let mut path = repo;
        path.push(LOCAL_HEAD);
        if !path.exists() {
            return Err(Error::Repository("LOCAL_HEAD not found".into()));
        }

        Ok(Self {
            path
        })
    }
    pub fn init(repo: PathBuf) -> Result<()> {
        let mut path = repo;
        path.push(LOCAL_HEAD);
        let lh = Self {
            path
        };

        lh.set_to_branch("main")
    }
    pub fn get(&self) -> crate::Result<Head> {
        let content = fs::read_to_string(&self.path)?;
        let head: Head = utils::encode(&content)
            .map_err(|_| Error::Repository("cannot parse head".into()))?;
        Ok(head)
    }
    fn set(&self, head: Head) -> crate::Result<()> {
        let content = utils::decode(&head)
            .map_err(|_| Error::Repository("cannot decode head".into()))?;
        fs::lock_write(&self.path, &content)?;

        Ok(())
    }

    pub fn set_to_branch(&self, branch: &str) -> Result<()> {
        let head = Head::Branch(branch.to_string());

        self.set(head)
    }
    pub fn set_to_oid(&self, oid: &Oid) -> Result<()> {
        let head = Head::Oid(oid.to_owned());

        self.set(head)
    }
}
