use crate::config::install_directory;
use crate::error::Error;
use crate::version::Version;
use getopts::{Options, ParsingStyle};
use std::env;
use std::process::{exit, Command};

const USAGE: &str = "ivm run [OPTIONS] [VERSION] [COMMAND] [ARGS...]

Runs a command using the given Inko version.

Examples:

    ivm run 0.8.0 inko --version     # Runs `inko --version` using 0.8.0
    ivm run latest inko --version    # Same, using the latest installed version";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.parsing_style(ParsingStyle::StopAtFirstFree);
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
                "You must specify a version to run the command with",
            ));
        }
    };

    let bin_path = install_directory()?.join(version.to_string()).join("bin");

    if !bin_path.is_dir() {
        return Err(Error::generic(format!(
            "Version {} is not installed",
            version
        )));
    }

    let cmd_args = &matches.free[1..];

    if cmd_args.is_empty() {
        return Err(Error::generic("You must specify a command to run"));
    }

    let mut paths = vec![bin_path];

    if let Some(path) = env::var_os("PATH") {
        paths.append(&mut env::split_paths(&path).collect());
    }

    let new_path = env::join_paths(paths).map_err(|error| {
        Error::generic(format!(
            "The PATH environment variable couldn't be updated: {}",
            error
        ))
    })?;

    let status = Command::new(&matches.free[1])
        .args(&matches.free[2..])
        .env("PATH", new_path)
        .spawn()?
        .wait()?;

    exit(status.code().unwrap_or(0));
}
