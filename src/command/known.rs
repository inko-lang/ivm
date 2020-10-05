use crate::error::Error;
use crate::manifest::Manifest;
use crate::version::Version;
use getopts::Options;

const USAGE: &str = "ivm known [OPTIONS]

Lists all available versions.

Examples:

    ivm known     # Lists all available versions";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(&options, USAGE);
        return Ok(());
    }

    Manifest::refresh()?;

    let default = Version::default();

    for version in Manifest::parse()?.iter() {
        match &default {
            Some(v) if v == version => println!("{} (default)", version),
            _ => println!("{}", version),
        }
    }

    Ok(())
}
