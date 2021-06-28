use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use data_encoding::HEXLOWER;
use directories::ProjectDirs;
use os_str_bytes::OsStrBytes;
use sha2::{Digest, Sha256};
use symlink::{remove_symlink_auto, symlink_auto};

/// State of a target path that should be managed declaratively.
pub enum TargetStatus {
    /// A target that should exist after running the specified command.
    Conceptual,

    /// A target that already exists but the specified command was not observed to have run
    /// successfully.
    Invalid,

    /// A target that used to exist where the specified command was observed to have run
    /// successfully, but now no longer exists.
    Missing,

    /// A target that exists where the specified command was observed to have run successfully.
    Tangible,
}

pub trait Permanence {
    fn target_status<P: AsRef<Path>>(&self, path: P) -> TargetStatus;

    fn observe_success<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    fn forget_about<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

pub struct SymlinkPermanence<'a> {
    dir: &'a Path,
}

impl<'a> SymlinkPermanence<'a> {
    pub fn new(project_dirs: &ProjectDirs) -> SymlinkPermanence {
        let dir = project_dirs.data_dir();
        SymlinkPermanence { dir }
    }

    fn symlink_name<P: AsRef<Path>>(&self, path: P) -> String {
        let mut hasher = Sha256::new();
        hasher.update(path.as_ref().as_os_str().to_raw_bytes());
        HEXLOWER.encode(hasher.finalize().as_ref())
    }

    fn symlink<P: AsRef<Path>>(&self, path: &P) -> PathBuf {
        let mut path_buf = self.dir.to_path_buf();
        path_buf.push(self.symlink_name(path));
        path_buf
    }
}

impl<'a> Permanence for SymlinkPermanence<'a> {
    fn target_status<P: AsRef<Path>>(&self, path: P) -> TargetStatus {
        let symlink = self.symlink(&path);
        match (symlink.symlink_metadata(), symlink.metadata()) {
            (Err(_), Err(_)) => TargetStatus::Conceptual,
            (Err(_), Ok(_)) => TargetStatus::Invalid,
            (Ok(_), Err(_)) => TargetStatus::Missing,
            (Ok(_), Ok(_)) => TargetStatus::Tangible,
        }
    }

    fn observe_success<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let symlink = self.symlink(&path);
        symlink_auto(&symlink, path.as_ref())
            .with_context(|| format!("Symlink {:?} -> {:?}", symlink, path.as_ref()))
    }

    fn forget_about<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let symlink = self.symlink(&path);
        remove_symlink_auto(&symlink).with_context(|| format!("Remove symlink {:?}", symlink))
    }
}

#[cfg(test)]
mod tests {}
