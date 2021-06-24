use std::{path::PathBuf, process::Command};

use anyhow::{bail, Result};

use crate::config::ConceptualPath;

pub struct TangiblePath<'a> {
    pub conceptual_path: &'a ConceptualPath,
}

impl<'a> TangiblePath<'a> {
    pub fn up(conceptual_path: &ConceptualPath) -> Result<TangiblePath> {
        let mut command = Command::new(&conceptual_path.command);

        command
            .env(
                conceptual_path.target_env_var.as_ref(),
                &conceptual_path.target,
            )
            .envs(&conceptual_path.env);

        let output = command.output()?;
        if !output.status.success() {
            bail!("{:?}: {:?}", output.status, command);
        }

        let target_path = PathBuf::from(&conceptual_path.target);
        if !target_path.exists() {
            bail!("{} does not exist", conceptual_path.target);
        }

        Ok(TangiblePath { conceptual_path })
    }
}
