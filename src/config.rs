use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap, fs::read_to_string, path::Path};

use crate::default;

pub enum ConfigFinder {
    Implicit,
    Explicit(String),
}

impl ConfigFinder {
    pub fn path(&self, app_name: &str) -> Result<Cow<Path>> {
        match self {
            Self::Implicit => {
                let project_dirs =
                    ProjectDirs::from("", "", app_name).context("Resolving project directories")?;
                let mut config_dir = project_dirs.config_dir().to_path_buf();
                config_dir.push(default::CONFIG_FILE_NAME);
                Ok(Cow::from(config_dir))
            }

            Self::Explicit(string) => Ok(Cow::from(Path::new(string))),
        }
    }

    pub fn read<P: AsRef<Path>>(&self, path: P) -> Result<Config> {
        let context = || format!("{}", path.as_ref().display());
        let contents = read_to_string(path.as_ref()).with_context(context)?;
        toml::from_str(&contents).with_context(context)
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub paths: Vec<ConceptualPath>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConceptualPath {
    pub target: String,
    #[serde(default = "default_target_env_var")]
    pub target_env_var: Cow<'static, str>,
    pub command: String,
    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,
    pub env: HashMap<String, String>,
}

fn default_target_env_var() -> Cow<'static, str> {
    Cow::from(default::TARGET_ENV_VAR)
}
