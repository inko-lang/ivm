use crate::config::{
    bin_directory, cache_directory, config_directory, data_directory,
    downloads_directory, install_directory,
};
use crate::error::Error;
use getopts::Options;

const USAGE: &str = "ivm show [OPTIONS] [SETTING]

Prints the value of a setting.

Available settings:

    data         The data directory
    bin          The directory for symbolic links to executables
    cache        The directory for storing temporary data
    install      The directory containing all installed versions
    config       The directory containing configuration files
    downloads    The directory containing downloaded files";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    match matches.free.first().map(|s| s.as_str()) {
        Some("data") => {
            println!("{}", data_directory()?.to_string_lossy());
        }
        Some("bin") => {
            println!("{}", bin_directory()?.to_string_lossy());
        }
        Some("cache") => {
            println!("{}", cache_directory()?.to_string_lossy());
        }
        Some("install") => {
            println!("{}", install_directory()?.to_string_lossy());
        }
        Some("config") => {
            println!("{}", config_directory()?.to_string_lossy());
        }
        Some("downloads") => {
            println!("{}", downloads_directory()?.to_string_lossy());
        }
        Some(setting) => {
            return Err(Error::generic(format!(
                "The setting {} doesn't exist",
                setting
            )));
        }
        _ => {
            return Err(Error::generic("You must specify a setting name"));
        }
    }

    Ok(())
}
