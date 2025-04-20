use crate::{
    repository::Oid,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    parent: Option<Oid>,
    root: Oid,
    message: String,
    //commiter: String,
}

impl Commit {
    pub fn new(parent: Option<Oid>, root: Oid, message: String) -> Self {
        Self {
            parent,
            root,
            message,
        }
    }
    pub fn root(&self) -> &Oid {
        &self.root
    }
    pub fn parent(&self) -> &Option<Oid> {
        &self.parent
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
