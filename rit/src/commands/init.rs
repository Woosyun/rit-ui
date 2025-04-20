use std::path::PathBuf;
use crate::prelude::*;

pub struct Init {
    ws: Workspace,
}
impl Init {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let init = Self {
            ws
        };
        Ok(init)
    }

    pub fn execute(&self) -> crate::Result<()> {
        Repository::init(&self.ws)
    }
}
