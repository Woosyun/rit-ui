use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::{
    workspace::stat::*,
    fs,
    repository::Oid,
};
use filetime::FileTime;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    oid: Option<Oid>,
    mtime: Mtime,
    mode: Mode,
}
impl File {
    pub fn build(path: &Path) -> crate::Result<Self> {
        let metadata = fs::metadata(path)?;
        let mtime = FileTime::from_last_modification_time(&metadata)
            .unix_seconds();
        let mode = match metadata.permissions().readonly() {
            true => READONLY_FILE_MODE,
            _ => EXECUTABLE_FILE_MODE,
        };

        let name = fs::get_file_name(path)?;

        let re = Self {
            name,
            oid: None,
            mtime,
            mode
        };
        Ok(re)
    }
    
}

impl Stat for File {
    fn mtime(&self) -> Mtime {
        self.mtime
    }
    fn mode(&self) -> Mode {
        self.mode
    }
    fn oid(&self) -> crate::Result<&Oid> {
        match &self.oid {
            Some(oid) => Ok(oid),
            None => Err(crate::Error::Workspace("use of oid() of File before set".into()))
        }
    }
    fn set_oid(&mut self, oid: Oid) {
        self.oid = Some(oid);
    }
    fn name(&self) -> &Name {
        &self.name
    }
    /*
    fn clone_box(&self) -> Box<dyn Stat> {
        Box::new(self.clone())
    }
    */
}
