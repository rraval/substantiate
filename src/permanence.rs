use std::path::{Path, PathBuf};

use crate::{config::ConceptualPath, tangible::TangiblePath};

use anyhow::Result;
use directories::ProjectDirs;

pub trait Permanence {
    fn is_up(conceptual_path: &ConceptualPath) -> Result<bool>;

    fn mark_as_up(tangible_path: &TangiblePath) -> Result<()>;
}

struct SymlinkPermanence<'a> {
    dir: &'a Path,
}

impl<'a> SymlinkPermanence<'a> {
    pub fn new(project_dirs: &ProjectDirs) -> SymlinkPermanence {
        let dir = project_dirs.data_dir();
        SymlinkPermanence { dir }
    }

    fn symlink_name(&self) -> String {
        todo!()
    }
}

impl<'a> Permanence for SymlinkPermanence<'a> {
    fn is_up(conceptual_path: &ConceptualPath) -> Result<bool> {
        todo!()
    }

    fn mark_as_up(tangible_path: &TangiblePath) -> Result<()> {
        todo!()
    }
}
