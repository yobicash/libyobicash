use semver::{Version, SemVerError};
use byteorder::{LittleEndian, BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use ::VERSION;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct YVersion(pub Version);

impl Default for YVersion {
  fn default() -> YVersion {
    YVersion::parse(VERSION).unwrap()
  }
}

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

  pub fn to_little_endian(&self) -> Option<[u8; 24]> {
    let mut res = [0; 24];
    let mut buf = Vec::new();
    match buf.write_u64::<LittleEndian>(self.0.major) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    match buf.write_u64::<LittleEndian>(self.0.minor) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    match buf.write_u64::<LittleEndian>(self.0.patch) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    for i in 0..24 {
      res[i] = buf[i]
    }
    Some(res)
  }

  pub fn to_big_endian(&self) -> Option<[u8; 24]> {
    let mut res = [0; 24];
    let mut buf = Vec::new();
    match buf.write_u64::<BigEndian>(self.0.major) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    match buf.write_u64::<BigEndian>(self.0.minor) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    match buf.write_u64::<BigEndian>(self.0.patch) {
      Ok(_) => {},
      Err(_) => { return None },
    }
    for i in 0..24 {
      res[i] = buf[i]
    }
    Some(res)
  }

  pub fn to_bytes(&self) -> Option<[u8; 24]> {
    self.to_big_endian()
  }

  pub fn from_little_endian(b: &[u8]) -> Option<YVersion> {
    let mut major: u64 = 0;
    let mut minor: u64 = 0;
    let mut patch: u64 = 0;
    let mut reader = Cursor::new(b);
    match reader.read_u64::<LittleEndian>() {
      Ok(_major) => { major = major },
      Err(_) => { return None },
    }
    match reader.read_u64::<LittleEndian>() {
      Ok(_minor) => { minor = minor },
      Err(_) => { return None },
    }
    match reader.read_u64::<LittleEndian>() {
      Ok(_patch) => { patch = patch },
      Err(_) => { return None },
    }
    Some(YVersion::new(major, minor, patch))
  }

  pub fn from_big_endian(b: &[u8]) -> Option<YVersion> {
    let mut major: u64 = 0;
    let mut minor: u64 = 0;
    let mut patch: u64 = 0;
    let mut reader = Cursor::new(b);
    match reader.read_u64::<BigEndian>() {
      Ok(_major) => { major = major },
      Err(_) => { return None },
    }
    match reader.read_u64::<BigEndian>() {
      Ok(_minor) => { minor = minor },
      Err(_) => { return None },
    }
    match reader.read_u64::<BigEndian>() {
      Ok(_patch) => { patch = patch },
      Err(_) => { return None },
    }
    Some(YVersion::new(major, minor, patch))
  }

  pub fn from_bytes(b: &[u8]) -> Option<YVersion> {
    YVersion::from_big_endian(b)
  }
}
