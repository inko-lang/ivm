use crate::config::{
    bin_directory, default_version_file, install_directory, INKO_EXE,
};
use crate::error::Error;
use crate::version::Version;
use getopts::Options;
use std::fs::{remove_file, write};

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink;

const USAGE: &str = "ivm default [OPTIONS] [VERSION]

Sets the default version to use.

Examples:

    ivm default 0.8.0     # Sets the default version to 0.8.0";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    let version = Version::parse(
        matches
            .free
            .first()
            .map(|s| s.as_str())
            .ok_or_else(|| Error::generic("You must specify a version"))?,
    )?;

    let version_dir = install_directory()?.join(version.to_string());

    if !version_dir.is_dir() {
        return Err(Error::generic(format!(
            "The version {} is not installed",
            version
        )));
    }

    info!("Storing default version");

    write(default_version_file()?, version.to_string()).map_err(|e| {
        Error::generic(format!("Failed to set the default version: {}", e))
    })?;

    info!("Creating symbolic link for {}", INKO_EXE);

    let sym_bin = bin_directory()?.join(INKO_EXE);
    let src_bin = version_dir.join("bin").join(INKO_EXE);

    if sym_bin.exists() {
        remove_file(&sym_bin)?;
    }

    symlink(src_bin, sym_bin)?;
    info!("The default version is now {}", version);
    Ok(())
}
