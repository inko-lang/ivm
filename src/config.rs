use crate::error::Error;
use std::env;
use std::path::PathBuf;

#[cfg(windows)]
pub const INKO_EXE: &str = "inko.exe";

#[cfg(not(windows))]
pub const INKO_EXE: &str = "inko";

const BASE_DIR: &str = "ivm";

fn home_dir() -> Option<PathBuf> {
    let var = if cfg!(windows) {
        env::var_os("USERPROFILE")
    } else {
        env::var_os("HOME")
    };

    var.filter(|v| !v.is_empty()).map(PathBuf::from)
}

fn windows_local_appdata() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join("AppData").join("Local")))
}

pub fn cache_directory() -> Result<PathBuf, Error> {
    let base = if cfg!(windows) {
        windows_local_appdata()
    } else if cfg!(macos) {
        home_dir().map(|h| h.join("Library").join("Caches"))
    } else {
        env::var_os("XDG_CACHE_HOME")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".cache")))
    };

    base.map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The cache directory couldn't be determined")
    })
}

pub fn data_directory() -> Result<PathBuf, Error> {
    let base = if cfg!(windows) {
        windows_local_appdata()
    } else if cfg!(macos) {
        home_dir().map(|h| h.join("Library").join("Application Support"))
    } else {
        env::var_os("XDG_DATA_HOME")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".local").join("share")))
    };

    base.map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The local data directory couldn't be determined")
    })
}

pub fn install_directory() -> Result<PathBuf, Error> {
    data_directory().map(|d| d.join("installed"))
}

pub fn config_directory() -> Result<PathBuf, Error> {
    let base = if cfg!(windows) {
        env::var_os("ROAMINGAPPDATA")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join("AppData").join("Roaming")))
    } else if cfg!(macos) {
        home_dir().map(|h| h.join("Library").join("Application Support"))
    } else {
        env::var_os("XDG_DATA_HOME")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".config")))
    };

    base.map(|d| d.join(BASE_DIR)).ok_or_else(|| {
        Error::generic("The configuration directory couldn't be determined")
    })
}

pub fn bin_directory() -> Result<PathBuf, Error> {
    data_directory().map(|d| d.join("bin"))
}

pub fn downloads_directory() -> Result<PathBuf, Error> {
    cache_directory().map(|d| d.join("downloads"))
}

pub fn default_version_file() -> Result<PathBuf, Error> {
    config_directory().map(|d| d.join("version"))
}

pub fn manifest_file() -> Result<PathBuf, Error> {
    downloads_directory().map(|d| d.join("manifest.txt"))
}
