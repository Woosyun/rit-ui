use crate::{
    repository::Oid,
    workspace::stat::*,
};
use serde::{Serialize, Deserialize};

type Name = String;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Entry(pub Mode, pub Mtime, pub Oid, pub Name);
impl Entry {
    pub fn new(mode: Mode, mtime: Mtime, oid: Oid, name: Name) -> Self {
        Self(mode, mtime, oid, name)
    }
    pub fn build(stat: &dyn Stat) -> crate::Result<Self> {
        let oid = stat.oid()?.clone();
        let name = stat.name().to_string();
        let result = Self(stat.mode(), stat.mtime(), oid, name);
        Ok(result)
    }
    pub fn mode(&self) -> Mode {
        self.0
    }
    pub fn mtime(&self) -> Mtime {
        self.1
    }
    pub fn oid(&self) -> &Oid {
        &self.2
    }
    pub fn set_oid(&mut self, oid: Oid) {
        self.2 = oid;
    }
    pub fn name(&self) -> &Name {
        &self.3
    }
}

impl Stat for Entry {
    fn mtime(&self) -> Mtime {
        self.mtime()
    }
    fn mode(&self) -> Mode {
        self.mode()
    }
    fn oid(&self) -> crate::Result<&Oid> {
        Ok(self.oid())
    }
    fn set_oid(&mut self, _oid: Oid) {
        ()
    }
    fn name(&self) -> &Name {
        self.name()
    }
    /*
    fn clone_box(&self) -> Box<dyn Stat> {
        Box::new(self.clone())
    }
    */
}
