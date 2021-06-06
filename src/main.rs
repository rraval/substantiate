use anyhow::Result;
use std::{collections::HashMap, fs::read_to_string, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    paths: Vec<ConfigPath>,
}

#[derive(Deserialize, Debug)]
struct ConfigPath {
    target: String,
    #[serde(default="default_target_env_var")]
    target_env_var: String,
    command: String,
    #[serde(default="default_tags")]
    tags: Vec<String>,
    env: HashMap<String, String>,
}

fn default_target_env_var() -> String {
    "SUBSTANTIATE_TARGET".to_string()
}

fn default_tags() -> Vec<String> {
    Vec::new()
}

fn main() -> Result<()> {
    let path = Path::new("config.toml");
    let contents = read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    println!("{:?}", config);
    Ok(())
}
