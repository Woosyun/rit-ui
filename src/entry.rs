use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub status: EntryStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntryStatus {
    Added,
    Modified,
    NotChanged,
}
