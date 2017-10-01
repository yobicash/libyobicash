use semver::{Version, SemVerError};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct YVersion(pub Version);

impl YVersion {
  pub fn new(major: u64, minor: u64, patch: u64) -> YVersion {
    YVersion(Version::new(major, minor, patch))
  }

  pub fn parse(s: &str) -> Result<YVersion, SemVerError> {
    let version = Version::parse(s)?;
    Ok(YVersion(version))
  }

  pub fn to_string(&self) -> String {
    format!("{}", self.0) 
  }
}
