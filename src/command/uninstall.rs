use crate::config::install_directory;
use crate::error::Error;
use crate::version::Version;
use getopts::Options;
use std::fs::remove_dir_all;

const USAGE: &str = "ivm uninstall [OPTIONS] [VERSION]

Uninstalls an existing version.

Examples:

    ivm uninstall 0.8.0     # Uninstalls version 0.8.0
    ivm uninstall latest    # Uninstalls the latest installed version";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    let version = match matches.free.get(0).map(|s| s.as_str()) {
        Some("latest") => Version::latest_installed()
            .ok_or_else(|| Error::generic("No versions are installed"))?,
        Some(version) => Version::parse(version)?,
        None => {
            return Err(Error::generic(
                "You must specify a version to uninstall",
            ));
        }
    };

    info!("Uninstalling version {}", version);

    let path = install_directory()?.join(version.to_string());

    if !path.exists() {
        return Ok(());
    }

    remove_dir_all(&path).map_err(|error| {
        Error::generic(format!(
            "Failed to remove {}: {}",
            path.to_string_lossy(),
            error
        ))
    })
}
