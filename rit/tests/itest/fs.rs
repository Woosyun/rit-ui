use std::{
    path::Path,
    io::Write,
    fs::OpenOptions,
};
pub use rit::fs::*;

pub fn appendln(path: &Path, content: &str) -> rit::Result<()> {
    let mut fd = OpenOptions::new().append(true).open(path)
        .map_err(Into::<rit::Error>::into)?;
    writeln!(fd, "{}", content)
        .map_err(Into::into)
}
