use crate::repository::Oid;
//use std::path::PathBuf;

pub type Mode = u32;
pub type Mtime = i64;
pub type Name = String;

pub const DIRECTORY_MODE: Mode = 0o40000;
pub const READONLY_FILE_MODE: Mode = 0o100644;
pub const EXECUTABLE_FILE_MODE: Mode = 0o100755;

pub trait Stat {
    fn mtime(&self) -> Mtime;
    fn mode(&self) -> Mode;
    fn oid(&self) -> crate::Result<&Oid>;
    fn set_oid(&mut self, oid: Oid);
    fn name(&self) -> &Name;

    //fn clone_box(&self) -> Box<dyn Stat>;

    fn is_dir(&self) -> bool {
        self.mode() == DIRECTORY_MODE
    }
}

/*
impl Clone for Box<dyn Stat> {
    fn clone(&self) -> Box<dyn Stat> {
        self.clone_box()
    }
}
*/
