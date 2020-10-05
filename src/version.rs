use crate::config::{default_version_file, install_directory};
use crate::error::Error;
use std::cmp::Ordering;
use std::fmt;
use std::fs::{read, read_dir};

#[derive(PartialEq, Eq, Clone)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn default() -> Option<Self> {
        default_version_file()
            .ok()
            .and_then(|path| {
                if path.is_file() {
                    read(path).ok()
                } else {
                    None
                }
            })
            .and_then(|data| {
                Self::parse(String::from_utf8_lossy(&data).trim()).ok()
            })
    }

    pub fn latest_installed() -> Option<Self> {
        let dir = install_directory().ok()?;
        let mut latest = None;

        for entry in read_dir(dir).ok()? {
            // If the version is invalid, it's probably because the entry is a
            // different (unexpected) directory. In that case we'll just ignore
            // it.
            let version =
                Version::parse(&entry.ok()?.file_name().to_string_lossy()).ok();

            if version > latest {
                latest = version;
            }
        }

        latest
    }

    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn parse(input: &str) -> Result<Version, Error> {
        let mut chunks = input
            .split('.')
            .map(|chunk| {
                chunk.parse::<u8>().map_err(|_| {
                    Error::generic(format!(
                        "The version {:?} is invalid",
                        input
                    ))
                })
            })
            .take(3);

        let major = chunks.next().unwrap_or(Ok(0))?;
        let minor = chunks.next().unwrap_or(Ok(0))?;
        let patch = chunks.next().unwrap_or(Ok(0))?;

        if major == 0 && minor == 0 && patch == 0 {
            return Err(Error::generic("The version \"0.0.0\" is invalid"));
        }

        Ok(Version::new(major, minor, patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Version({}.{}.{})", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => {}
            order => return order,
        }

        match self.minor.cmp(&other.minor) {
            Ordering::Equal => {}
            order => return order,
        }

        self.patch.cmp(&other.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Version::parse("1"), Ok(Version::new(1, 0, 0)));
        assert_eq!(Version::parse("1.1"), Ok(Version::new(1, 1, 0)));
        assert_eq!(Version::parse("1.0.0"), Ok(Version::new(1, 0, 0)));
        assert_eq!(Version::parse("1.0.1"), Ok(Version::new(1, 0, 1)));
        assert_eq!(Version::parse("1.1.1"), Ok(Version::new(1, 1, 1)));
        assert!(Version::parse("1.1.a").is_err());
        assert!(Version::parse("1.a.1").is_err());
        assert!(Version::parse("a.1.1").is_err());
        assert!(Version::parse("0.0.0").is_err());
    }

    #[test]
    fn test_cmp() {
        assert!(Version::new(1, 0, 0) == Version::new(1, 0, 0));
        assert!(Version::new(1, 0, 1) > Version::new(1, 0, 0));
        assert!(Version::new(1, 1, 0) > Version::new(1, 0, 0));
        assert!(Version::new(0, 0, 2) > Version::new(0, 0, 1));
        assert!(Version::new(0, 1, 0) > Version::new(0, 0, 1));
        assert!(Version::new(0, 0, 1) > Version::new(0, 0, 0));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Version::new(1, 2, 3)), "1.2.3");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Version::new(1, 2, 3)), "Version(1.2.3)");
    }
}
