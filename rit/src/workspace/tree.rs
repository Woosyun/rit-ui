use std::{
    collections::HashMap,
};
use crate::{
    repository,
    workspace::stat::*,
};

pub enum Entry {
    Tree(Tree),
    Entry(Box<dyn Stat>),
}

pub struct Tree{
    name: Name,
    oid: Option<repository::Oid>,
    pub entries: HashMap<Name, Entry>,
}
impl Tree {
    pub fn new(name: String) -> Self {
        Self {
            name,
            oid: None,
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, ancestors: &mut Vec<String>, entry: Box<dyn Stat>) {
        let entry_name = ancestors.pop().unwrap();

        if ancestors.is_empty() {
            let _ = self.entries.insert(entry_name, Entry::Entry(entry));
            return;
        }

        if let Some(tree) = self.entries.get_mut(&entry_name) {
            if let Entry::Tree(tree) = tree {
                tree.add_entry(ancestors, entry);
            }
        } else {
            let mut tree = Tree::new(entry_name.clone());
            tree.add_entry(ancestors, entry);
            self.entries.insert(entry_name, Entry::Tree(tree));
        }
    }

    // depending on the return type of traverse function,
    // Tree might not needs to store oid, name, or other informations
    // those needed to create entry, of database::Tree.
    pub fn traverse_mut<F: Fn(&mut Tree) -> crate::Result<()> + Copy>(&mut self, f: F) -> crate::Result<()> {
        for (_, entry) in self.entries.iter_mut() {
            if let Entry::Tree(tree) = entry {
                let _ = tree.traverse_mut(f)?;
            }
        }

        f(self)
    }
}

impl Stat for Tree {
    fn mode(&self) -> Mode {
        DIRECTORY_MODE
    }
    fn mtime(&self) -> Mtime {
        0
    }
    fn oid(&self) -> crate::Result<&repository::Oid> {
        match &self.oid {
            Some(oid) => Ok(oid),
            None => Err(crate::Error::Workspace("try to access oid of tree before it set".into()))
        }
    }
    fn set_oid(&mut self, oid: repository::Oid) {
        self.oid = Some(oid);
    }
    fn name(&self) -> &Name {
        &self.name
    }
}
