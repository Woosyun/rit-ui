use crate::{
    repository::Entry,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree(Vec<Entry>);
impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self (entries)
    }
    pub fn entries(&self) -> &Vec<Entry> {
        &self.0
    }
}
