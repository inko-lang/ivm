use crate::command::clean;
use crate::command::default;
use crate::command::implode;
use crate::command::install;
use crate::command::known;
use crate::command::list;
use crate::command::run as run_cmd;
use crate::command::uninstall;
use crate::config::{
    bin_directory, cache_directory, config_directory, data_directory,
    install_directory,
};
use crate::error::Error;
use getopts::{Options, ParsingStyle};
use std::env;
use std::fs::create_dir_all;

const USAGE: &str = "ivm [OPTIONS] [COMMAND]

ivm is Inko's version manager, and can be used to install and manage different
Inko versions.

Commands:

    install      Install a new version
    uninstall    Uninstall an existing version
    list         List all installed versions
    known        List all available versions
    run          Run a command with a specific version
    default      Set the default version
    clean        Clean up temporary data
    implode      Removes all versions and temporary data

Examples:

    ivm install 0.8.0      # Install version 0.8.0
    ivm uninstall 0.8.0    # Uninstall version 0.8.0
    ivm run 0.8.0 foo      # Run the command `foo` with version 0.8.0";

pub fn run() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();
    let mut options = Options::new();

    options.parsing_style(ParsingStyle::StopAtFirstFree);
    options.optflag("h", "help", "Shows this help message");
    options.optflag("v", "version", "Prints the version number");

    let matches = options.parse(&args[1..])?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    if matches.opt_present("v") {
        println!("ivm version {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // We create all necessary directories here so we don't have to do this
    // every time some piece of code needs these to exist.
    create_dir_all(cache_directory()?)?;
    create_dir_all(data_directory()?)?;
    create_dir_all(install_directory()?)?;
    create_dir_all(config_directory()?)?;
    create_dir_all(bin_directory()?)?;

    let cmd_args = &matches.free[1..];

    match matches.free.get(0).map(|s| s.as_str()) {
        Some("install") => install::run(cmd_args),
        Some("uninstall") => uninstall::run(cmd_args),
        Some("list") => list::run(cmd_args),
        Some("known") => known::run(cmd_args),
        Some("run") => run_cmd::run(cmd_args),
        Some("default") => default::run(cmd_args),
        Some("clean") => clean::run(cmd_args),
        Some("implode") => implode::run(cmd_args),
        Some(command) => Err(Error::generic(format!(
            "The command {:?} is not valid",
            command
        ))),
        None => {
            usage!(&options, USAGE);
            Ok(())
        }
    }
}
