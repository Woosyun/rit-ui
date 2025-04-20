use crate::{
    workspace::Stat,
    //repository::Blob,
};
use std::{
    collections::{HashSet, HashMap},
    path::{PathBuf, Path},
    fmt::{self, Write},
};
//use serde::{Serialize, Deserialize};

pub trait IntoRev {
    fn into_rev(&self) -> crate::Result<Rev>;
}

pub struct Rev(pub HashMap<PathBuf, Box<dyn Stat>>);
impl Rev {
    pub fn new(rev: HashMap<PathBuf, Box<dyn Stat>>) -> Self {
        Self(rev)
    }

    pub fn get_mut(&mut self, idx: &Path) -> Option<&mut Box<dyn Stat>> {
        self.0.get_mut(idx)
    }

    pub fn diff(&self, to: &Rev) -> crate::Result<RevDiff> {
        let mut rev_diff = RevDiff::new();
        for (path, _) in self.0.iter() {
            if to.0.get(path).is_none() {
                rev_diff.removed.insert(path.to_path_buf());
            }
        }
        for (index, to_entry) in to.0.iter() {
            if let Some(from_entry) = self.0.get(index) {
                if from_entry.mtime() != to_entry.mtime() {
                    rev_diff.modified.insert(index.to_owned());
                }
            } else {
                rev_diff.added.insert(index.to_owned());
            }
        }

        Ok(rev_diff)
    }
}

#[derive(PartialEq, Debug)]
pub struct RevDiff {
    pub added: HashSet<PathBuf>,
    pub removed: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
}
impl RevDiff {
    pub fn new() -> Self {
        Self {
            added: HashSet::new(),
            removed: HashSet::new(),
            modified: HashSet::new(),
        }
    }
    pub fn is_clean(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }
}

impl fmt::Display for RevDiff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        writeln!(&mut output, "added files:")?;
        writeln!(&mut output,
            "{}",
            self.added
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )?;
        writeln!(&mut output, "removed files:")?;
        writeln!(&mut output,
            "{}",
            self.removed
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )?;
        writeln!(&mut output, "modified files:")?;
        writeln!(&mut output,
            "{}",
            self.modified
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )?;

        write!(f, "{}", output)
    }
}
