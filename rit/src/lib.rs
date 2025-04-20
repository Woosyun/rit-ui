pub mod workspace;
pub mod repository;
pub mod revision;
pub mod utils;
pub mod commands;

pub mod fs;

pub mod error;
pub use error::*;

pub mod prelude {
    pub use super::workspace::{
        self,
        Workspace,
        Ignore,
        File,
        stat::{
            Stat,
            Mode,
            Mtime,
            Name,
        },
    };
    pub use super::repository::{
        self,
        Repository,
        Blob,
        Oid,
    };
    pub use super::revision::{
        Revision,
        IntoRev,
        Rev,
        RevDiff,
    };
    pub use super::utils::*;
    pub use super::commands;
    pub use super::error::*;
}
