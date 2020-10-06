use crate::config::downloads_directory;
use crate::error::Error;
use getopts::Options;
use std::fs::remove_dir_all;

const USAGE: &str = "ivm clean [OPTIONS]

Cleans up temporary data produced by ivm.

Examples:

    ivm clean     # Cleans up all temporary data";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    let dir = downloads_directory()?;

    info!("Removing {}", dir.to_string_lossy());

    remove_dir_all(dir).map_err(|error| {
        Error::generic(format!(
            "Failed to remove the temporary data: {}",
            error
        ))
    })
}
