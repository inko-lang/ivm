use crate::config::install_directory;
use crate::error::Error;
use crate::version::Version;
use getopts::Options;
use std::fs::read_dir;

const USAGE: &str = "ivm list [OPTIONS]

Lists all installed versions.

Examples:

    ivm list     # Lists all installed versions";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    let mut versions = Vec::new();
    let default = Version::default();

    for entry in read_dir(install_directory()?)? {
        if let Ok(version) =
            Version::parse(&entry?.file_name().to_string_lossy())
        {
            versions.push(version);
        }
    }

    versions.sort();

    for version in versions {
        match &default {
            Some(v) if v == &version => println!("{} (default)", version),
            _ => println!("{}", version),
        }
    }

    Ok(())
}
