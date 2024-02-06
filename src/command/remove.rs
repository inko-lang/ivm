use crate::config::{
    bin_directory, default_version_file, inko_data_directory,
    install_directory, INKO_EXE,
};
use crate::error::Error;
use crate::version::Version;
use getopts::Options;
use std::fs::{read, remove_dir_all, remove_file};

const USAGE: &str = "ivm remove [OPTIONS] [VERSION]

Removes an existing version.

Examples:

    ivm remove 0.8.0     # Remove version 0.8.0
    ivm remove latest    # Remove the latest installed version";

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
            return Err(Error::generic("You must specify a version to remove"));
        }
    };

    info!("Uninstalling version {}", version);

    let path = install_directory()?.join(version.to_string());

    if !path.exists() {
        return Ok(());
    }

    let mut paths = vec![
        path,
        inko_data_directory()?
            .join("runtimes")
            .join(version.to_string()),
    ];

    // If the version we're removing is the default version, also remove the
    // version file and the corresponding symbolic link.
    if let Ok(default) = default_version_file() {
        if let Ok(data) = read(&default) {
            if String::from_utf8_lossy(&data) == version.to_string() {
                paths.push(default);
                paths.push(bin_directory()?.join(INKO_EXE));
            }
        }
    }

    for path in paths {
        let res = if path.is_dir() {
            remove_dir_all(&path)
        } else if path.is_file() || path.is_symlink() {
            remove_file(&path)
        } else {
            Ok(())
        };

        if let Err(e) = res {
            return Err(Error::generic(format!(
                "Failed to remove {}: {}",
                path.display(),
                e
            )));
        }
    }

    Ok(())
}
