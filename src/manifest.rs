use crate::config::manifest_file;
use crate::error::Error;
use crate::http;
use crate::version::Version;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime};

const URL: &str = "https://releases.inko-lang.org/manifest.txt";
const EXPIRE_AFTER: u64 = 6 * 60 * 60;

pub struct Manifest {
    versions: Vec<Version>,
}

impl Manifest {
    pub fn new(versions: Vec<Version>) -> Self {
        Manifest { versions }
    }

    pub fn parse() -> Result<Self, Error> {
        let file = manifest_file()?;
        let mut content = String::new();

        File::open(file)
            .and_then(|mut handle| handle.read_to_string(&mut content))
            .map_err(|error| {
                Error::generic(format!(
                    "Failed to read the manifest file: {}",
                    error
                ))
            })?;

        let mut versions = Vec::new();

        for line in content.lines() {
            versions.push(Version::parse(line)?);
        }

        versions.sort();

        Ok(Self::new(versions))
    }

    pub fn refresh() -> Result<(), Error> {
        let file = manifest_file()?;

        // To reduce the amount of HTTP requests, we only update the manifest
        // if deemed necessary.
        let download = if file.exists() {
            let duration = SystemTime::now()
                .duration_since(file.metadata()?.modified()?)
                .unwrap_or_else(|_| Duration::from_secs(EXPIRE_AFTER));

            duration.as_secs() >= EXPIRE_AFTER
        } else {
            true
        };

        if download {
            let mut response = http::get(URL)?;
            let body = response.body_mut().read_to_string().map_err(|e| {
                Error::generic(format!(
                    "failed to read the response body: {}",
                    e
                ))
            })?;

            File::create(&file)
                .and_then(|mut handle| handle.write_all(body.as_bytes()))
                .map_err(|error| {
                    Error::generic(format!(
                        "Failed to update the manifest file: {}",
                        error
                    ))
                })?;
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Version> {
        self.versions.iter()
    }

    pub fn latest(&self) -> Result<Version, Error> {
        self.versions
            .last()
            .cloned()
            .ok_or_else(|| Error::generic("There are no versions available"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latest() {
        let manifest1 = Manifest::new(vec![Version::new(1, 0, 0)]);
        let manifest2 = Manifest::new(Vec::new());

        assert_eq!(manifest1.latest(), Ok(Version::new(1, 0, 0)));
        assert!(manifest2.latest().is_err());
    }
}
