use crate::config::{downloads_directory, install_directory, INKO_EXE};
use crate::error::Error;
use crate::http;
use crate::manifest::Manifest;
use crate::version::Version;
use flate2::read::GzDecoder;
use getopts::Options;
use std::fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

const USAGE: &str = "ivm install [OPTIONS] [VERSION]

Installs a new version.

Examples:

    ivm install 0.8.0     # Installs version 0.8.0
    ivm install latest    # Installs the latest available version";

pub fn run(arguments: &[String]) -> Result<(), Error> {
    let mut options = Options::new();

    options.optflag("h", "help", "Shows this help message");

    let matches = options.parse(arguments)?;

    if matches.opt_present("h") {
        usage!(options, USAGE);
        return Ok(());
    }

    Manifest::refresh()?;

    let manifest = Manifest::parse()?;
    let version = match matches.free.get(0).map(|s| s.as_str()) {
        Some("latest") => manifest.latest()?,
        Some(version) => Version::parse(version)?,
        None => {
            return Err(Error::generic(
                "You must specify a version to install",
            ));
        }
    };

    info!("Downloading version {}", version);

    let source = extract(&version)?;
    let target = install_directory()?.join(version.to_string());

    info!("Installing version {}", version);

    install(&source, &target)?;

    info!("Removing source directory");

    remove_dir_all(source).map_err(|error| {
        Error::generic(format!(
            "Failed to remove the source directory: {}",
            error
        ))
    })?;

    info!("Version {} has been installed", version);

    Ok(())
}

fn extract(version: &Version) -> Result<PathBuf, Error> {
    let url = &format!("https://releases.inko-lang.org/{}.tar.gz", version);
    let extract_to = downloads_directory()?.join(version.to_string());

    if extract_to.exists() {
        return Ok(extract_to);
    }

    // We don't rely on the manifest to determine if a version exists, as the
    // manifest may be slightly out of date. This in turn would be annoying for
    // users wanting to install a version that just got released.
    if !http::exists(url) {
        return Err(Error::generic("The version does not exist"));
    }

    let response = http::get(url)?.into_reader();

    create_dir(&extract_to).map_err(|error| {
        Error::generic(format!(
            "Failed to create {}: {}",
            extract_to.to_string_lossy(),
            error
        ))
    })?;

    Archive::new(GzDecoder::new(response))
        .entries()
        .and_then(|entries| {
            for entry in entries {
                entry.and_then(|mut entry| entry.unpack_in(&extract_to))?;
            }

            Ok(())
        })
        .map_err(|error| {
            Error::generic(format!(
                "Failed to unpack the TAR archive into {}: {}",
                extract_to.to_string_lossy(),
                error
            ))
        })?;

    Ok(extract_to)
}

fn install(source: &PathBuf, target: &Path) -> Result<(), Error> {
    if target.is_dir() {
        return Err(Error::generic("The version is already installed"));
    }

    let bin_dir = target.join("bin");
    let std_dir = target.join("lib").join("inko").join("libstd");
    let license_dir = target.join("share").join("licenses").join("inko");
    let mut command = Command::new("cargo");

    // We don't use the Makefile to cut down the number of dependencies, and
    // because not all platforms may have Make installed (e.g. Windows).
    command
        .arg("build")
        .arg("--release")
        .env("RUSTFLAGS", "-C target-feature=+aes")
        .env("INKO_LIBSTD", &std_dir)
        .current_dir(source);

    if !cfg!(windows) {
        // Dynamic linking of libffi doesn't work on MSVC, and we never got it
        // to work when using MSYS2 either. As such we only dynamically link to
        // libffi on Unix systems.
        command.arg("--features").arg("libffi-system");
    }

    run_command(&mut command)?;

    mkdir_p(&bin_dir)?;
    mkdir_p(&std_dir)?;
    mkdir_p(&license_dir)?;

    cp(
        source.join("target/release").join(INKO_EXE),
        bin_dir.join(INKO_EXE),
    )?;
    cp(source.join("LICENSE"), license_dir.join("LICENSE"))?;
    cp_r(source.join("libstd").join("src"), std_dir)?;

    Ok(())
}

fn run_command(command: &mut Command) -> Result<(), Error> {
    command
        .spawn()
        .and_then(|mut child| {
            // convert the Result<Child, _> into a Result<i32, _>, with the i32
            // being the exit code.
            child.wait().map(|status| status.code().unwrap_or(0))
        })
        .map_err(|e| e.into())
        .and_then(|status| {
            if status == 0 {
                Ok(())
            } else {
                Err(Error::generic(format!(
                    "The command exited with status code {}",
                    status
                )))
            }
        })
}

fn cp_r(source: PathBuf, target: PathBuf) -> Result<(), Error> {
    create_dir_all(&target)?;

    let mut pending = vec![source.clone()];

    while let Some(path) = pending.pop() {
        let entries = read_dir(&path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                pending.push(path);
                continue;
            }

            let rel = path.strip_prefix(&source).unwrap();
            let target = target.join(rel);
            let dir = target.parent().unwrap();

            create_dir_all(dir).map_err(|e| {
                Error::generic(format!("Failed to create {:?}: {}", dir, e))
            })?;

            cp(path, target)?;
        }
    }

    Ok(())
}

fn cp(source: PathBuf, target: PathBuf) -> Result<(), Error> {
    copy(&source, &target).map_err(|error| {
        Error::generic(format!(
            "Failed to copy {} to {}: {}",
            source.to_string_lossy(),
            target.to_string_lossy(),
            error
        ))
    })?;

    Ok(())
}

fn mkdir_p(path: &PathBuf) -> Result<(), Error> {
    create_dir_all(path).map_err(|error| {
        Error::generic(format!(
            "Failed to create the directory {}: {}",
            path.to_string_lossy(),
            error
        ))
    })
}
