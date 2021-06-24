use std::env::set_current_dir;

use anyhow::{Context, Result};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};
use directories::ProjectDirs;

use crate::{config::ConfigFinder, tangible::TangiblePath};

mod config;
mod default;
mod permanence;
mod tangible;

fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .name(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .long("config")
                .takes_value(true)
                .help("Path to configuration file"),
        )
        .subcommand(SubCommand::with_name("show").about("Print the parsed configuration file"))
        .subcommand(
            SubCommand::with_name("up").about("Invoke commands to bring paths into existence"),
        )
        .get_matches();

    let config_finder = matches
        .value_of("config")
        .map(|config| ConfigFinder::Explicit(config.to_string()))
        .unwrap_or(ConfigFinder::Implicit);

    let project_dirs =
        ProjectDirs::from("", "", crate_name!()).context("Resolving project directories")?;
    let path = config_finder.path(&project_dirs)?;
    let config = config_finder.read(&path)?;
    set_current_dir(path.parent().context("Config file parent")?)?;

    if matches.subcommand_matches("show").is_some() {
        println!("{:#?}", config);
        return Ok(());
    }

    if matches.subcommand_matches("up").is_some() {
        config
            .paths
            .iter()
            .try_for_each(|p| TangiblePath::up(p).map(|_| ()))?;
    }

    Ok(())
}
