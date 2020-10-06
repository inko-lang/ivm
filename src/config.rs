use crate::error::Error;
use dirs_next::{cache_dir, config_dir, data_local_dir};
use std::path::PathBuf;

#[cfg(windows)]
pub const INKO_EXE: &str = "inko.exe";

#[cfg(not(windows))]
pub const INKO_EXE: &str = "inko";

const BASE_DIR: &str = "ivm";

pub fn cache_directory() -> Result<PathBuf, Error> {
    cache_dir().map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The cache directory couldn't be determined")
    })
}

pub fn data_directory() -> Result<PathBuf, Error> {
    data_local_dir().map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The local data directory couldn't be determined")
    })
}

pub fn install_directory() -> Result<PathBuf, Error> {
    data_directory().map(|d| d.join("installed"))
}

pub fn config_directory() -> Result<PathBuf, Error> {
    config_dir().map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The configuration directory couldn't be determined")
    })
}

pub fn bin_directory() -> Result<PathBuf, Error> {
    data_directory().map(|d| d.join("bin"))
}

pub fn default_version_file() -> Result<PathBuf, Error> {
    config_directory().map(|d| d.join("version"))
}

pub fn manifest_file() -> Result<PathBuf, Error> {
    cache_directory().map(|d| d.join("manifest.txt"))
}
