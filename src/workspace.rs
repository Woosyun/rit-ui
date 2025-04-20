use std::{
    collections::HashMap,
    path::PathBuf,
};

pub struct Dir {
    name: String,
    status: EntryStatus,
    entries: HashMap<String, Entry>,
}
impl Dir {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: EntryStatus::NotChanged,
            entries: HashMap::new(),
        }
    }
    pub fn add_entry(&mut self, path: PathBuf, entry_status: EntryStatus) {
        let mut path = path.into_iter()
            .map(|oss| oss.to_str().unwrap().to_string())
            .collect::<Vec<_>>();
        self.rec_add_entry(&mut path, entry_status);
    }
    fn rec_add_entry(&mut self, path: &mut Vec<String>, entry_status: EntryStatus) {
        let name = path.pop().unwrap();
        if path.len() == 0 {
            let entry = Entry::File(File::new(name.clone(), entry_status));
            let _ = self.entries.insert(name, entry);
        } else {
            if let Some(dir) = self.entries.get_mut(&name) {
                if let Entry::Dir(dir) = dir {
                    dir.rec_add_entry(path, entry_status);
                }
            } else {
                let mut new_dir = Dir::new(name.clone());
                new_dir.rec_add_entry(path, entry_status);
                let _ = self.entries.insert(name, Entry::Dir(new_dir));
            }
        }
    }
}
pub enum Entry {
    Dir(Dir),
    File(File),
}
pub struct File {
    name: String,
    status: EntryStatus,
}
impl File {
    pub fn new(name: String, status: EntryStatus) -> Self {
        Self {
            name,
            status,
        }
    }
}
pub enum EntryStatus {
    Added,
    Modified,
    NotChanged,
}
