use crate::config::{cache_directory, config_directory, data_directory};
use crate::error::Error;
use getopts::Options;
use std::fs::remove_dir_all;
use std::path::PathBuf;

const USAGE: &str = "ivm implode [OPTIONS]

Removes all data produced by ivm, including any installed versions.

Examples:

    ivm implode     # Removes all of ivm's data";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    remove(cache_directory()?)?;
    remove(data_directory()?)?;
    remove(config_directory()?)?;

    info!("All data has been removed");

    Ok(())
}

fn remove(path: PathBuf) -> Result<(), Error> {
    if !path.exists() {
        return Ok(());
    }

    info!("Removing {}", path.to_string_lossy());

    remove_dir_all(&path).map_err(|error| {
        Error::generic(format!(
            "Failed to remove {}: {}",
            path.to_string_lossy(),
            error
        ))
    })
}
