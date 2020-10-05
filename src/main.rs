// This must come first so other modules can use the macros.
mod macros;

mod command;
mod config;
mod error;
mod http;
mod manifest;
mod version;

use command::main;
use std::process::exit;

fn main() {
    if let Err(e) = main::run() {
        error!("{}", e);
        exit(1);
    }
}
