// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `version` module provides the version types and methods.

use rmp_serde as messagepack;
use hex;
use regex::{Regex, Captures};

use constants::VERSION;
use error::ErrorKind;
use result::Result;
use traits::{Validate, BinarySerialize, HexSerialize};

use std::fmt;
use std::str::FromStr;

/// A `Version` is a semver formatted version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version.
    pub patch: u32,
    /// Release version.
    pub release: String,
    /// Buildmeta version.
    pub buildmeta: String,
}

impl Version {
    /// Creates a UTC unix `Version` from a given date.
    pub fn from_parts(major: u32,
                      minor: u32,
                      patch: u32,
                      release: &str,
                      buildmeta: &str) -> Result<Version>
    {
        if release.is_empty() && !buildmeta.is_empty() {
            return Err(ErrorKind::InvalidVersion.into());
        }

        let version = Version {
            major: major,
            minor: minor,
            patch: patch,
            release: String::from(release),
            buildmeta: String::from(buildmeta),
        };

        Ok(version)
    }

    /// Creates a `Version` from a semver version string.
    pub fn parse(s: &str) -> Result<Version> {
        let re = Regex::new(r"(?x)
            ^(?P<major>\d+)
            \.(?P<minor>\d+)
            \.(?P<patch>\d+)
            (-(?P<release>[[:alnum:]]+)
              (\.(?P<buildmeta>[[:alnum:]]+))?
            )?$")?;
        let parts_res: Result<Captures> = re.captures(s).ok_or_else(|| ErrorKind::InvalidFormat.into());
        
        let parts = parts_res?;

        let major = u32::from_str(&parts["major"])?;
        let minor = u32::from_str(&parts["minor"])?;
        let patch = u32::from_str(&parts["patch"])?;
        
        let release = if parts.name("release").is_some() {
            String::from(&parts["release"])
        } else {
            String::new()
        };

        let buildmeta = if parts.name("buildmeta").is_some() {
            String::from(&parts["buildmeta"])
        } else {
            String::new()
        };

        let version = Version {
            major: major,
            minor: minor,
            patch: patch,
            release: release,
            buildmeta: buildmeta,
        };

        Ok(version)
    }

    /// Returns the current `Version`.
    pub fn current() -> Result<Version> {
        Version::parse(VERSION)
    }

    /// Returns the `Version` minimum value.
    pub fn min_value() -> Result<Version> {
        Version::from_parts(0, 1, 0, "", "")
    }

    /// Returns the `Version` maximum value.
    pub fn max_value() -> Result<Version> {
        Version::current()
    }

    /// Creates a `Version` from a string.
    pub fn from_string(s: &str) -> Result<Version> {
        Self::parse(s)
    }

    /// Converts the `Version` to string.
    pub fn to_string(&self) -> String {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);

        if !self.release.is_empty() {
            version.push_str(&format!("-{}", self.release));

            if !self.buildmeta.is_empty() {
                version.push_str(&format!(".{}", self.buildmeta));
            }
        }

        version
    }
}

impl BinarySerialize for Version {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::encode::to_vec(self)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<Version> {
        let version = messagepack::decode::from_slice(b)?;

        Ok(version)
    }
}

impl HexSerialize for Version {
    fn from_hex(s: &str) -> Result<Version> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }

        Version::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}

impl Default for Version {
    fn default() -> Version {
        Version::current().unwrap()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Validate for Version {
    fn validate(&self) -> Result<()> {
        let max = Version::max_value()?;

        let min = Version::min_value()?;

        if *self > max || *self < min {
            return Err(ErrorKind::InvalidVersion.into());
        }

        Ok(())
    }
}
