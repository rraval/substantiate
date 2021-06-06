use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

use crate::config::ConfigFinder;

mod config;
mod default;

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
        .get_matches();

    let config_finder = matches
        .value_of("config")
        .map(|config| ConfigFinder::Explicit(config.to_string()))
        .unwrap_or(ConfigFinder::Implicit);

    if matches.subcommand_matches("show").is_some() {
        let path = config_finder.path(crate_name!())?;
        let config = config_finder.read(path)?;
        println!("{:#?}", config);
    }

    Ok(())
}
