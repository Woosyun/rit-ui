use std::{
    fs,
    path::Path,
};

pub fn read_to_string(path: &Path) -> crate::Result<String> {
    fs::read_to_string(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn write(path: &Path, content: &str) -> crate::Result<()> {
    fs::write(path, content)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn remove_file(path: &Path) -> crate::Result<()> {
    fs::remove_file(path)
        .map_err(|e| {
            let f = format!("{:?}: {}", path, e);
            crate::Error::Io(f)
        })
}

pub fn read_dir(path: &Path) -> crate::Result<fs::ReadDir> {
    fs::read_dir(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn metadata(path: &Path) -> crate::Result<fs::Metadata> {
    fs::metadata(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn create_dir(path: &Path) -> crate::Result<()> {
    fs::create_dir(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}
pub fn create_dir_all(path: &Path) -> crate::Result<()> {
    fs::create_dir_all(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn rename(base: &Path, target: &Path) -> crate::Result<()> {
    fs::rename(base, target)
        .map_err(|e| {
            let msg = format!("{:?}->{:?}: {}", base, target, e);
            crate::Error::Io(msg)
        })
}

pub fn get_file_name(path: &Path) -> crate::Result<String> {
    match path.file_name() {
        Some(file_name) => {
            let file_name = file_name
                .to_str().unwrap()
                .to_string();
            Ok(file_name)
        },
        None => {
            let f = format!("{:?}: cannot get file name. Maybe file name termiantes with ..", path);
            Err(crate::Error::Workspace(f))
        }
    }
}

pub fn lock_write(file: &Path, content: &str) -> crate::Result<()> {
    let mut lockfile = file.to_path_buf();
    lockfile.set_extension("lock");

    write(&lockfile, content)?;
    rename(&lockfile, file)?;

    Ok(())
}
