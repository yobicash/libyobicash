use semver::Version;
use byteorder::{LittleEndian, BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use VERSION;
use errors::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
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

    pub fn major(&self) -> u64 {
        self.0.major
    }

    pub fn minor(&self) -> u64 {
        self.0.minor
    }

    pub fn patch(&self) -> u64 {
        self.0.patch
    }

    pub fn parse(s: &str) -> YResult<YVersion> {
        let version = Version::parse(s)?;
        Ok(YVersion(version))
    }

    pub fn from_str(s: &str) -> YResult<YVersion> {
        YVersion::parse(s)
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }

    pub fn to_little_endian(&self) -> YResult<[u8; 24]> {
        let mut res = [0; 24];
        let mut buf = Vec::new();
        buf.write_u64::<LittleEndian>(self.0.major)?;
        buf.write_u64::<LittleEndian>(self.0.minor)?;
        buf.write_u64::<LittleEndian>(self.0.patch)?;
        for i in 0..24 {
            res[i] = buf[i]
        }
        Ok(res)
    }

    pub fn from_little_endian(b: &[u8]) -> YResult<YVersion> {
        let mut reader = Cursor::new(b);
        let major = reader.read_u64::<LittleEndian>()?;
        let minor = reader.read_u64::<LittleEndian>()?;
        let patch = reader.read_u64::<LittleEndian>()?;
        Ok(YVersion::new(major, minor, patch))
    }

    pub fn to_big_endian(&self) -> YResult<[u8; 24]> {
        let mut res = [0; 24];
        let mut buf = Vec::new();
        buf.write_u64::<BigEndian>(self.0.major)?;
        buf.write_u64::<BigEndian>(self.0.minor)?;
        buf.write_u64::<BigEndian>(self.0.patch)?;
        for i in 0..24 {
            res[i] = buf[i]
        }
        Ok(res)
    }

    pub fn from_big_endian(b: &[u8]) -> YResult<YVersion> {
        let mut reader = Cursor::new(b);
        let major = reader.read_u64::<BigEndian>()?;
        let minor = reader.read_u64::<BigEndian>()?;
        let patch = reader.read_u64::<BigEndian>()?;
        Ok(YVersion::new(major, minor, patch))
    }

    pub fn to_bytes(&self) -> YResult<[u8; 24]> {
        self.to_big_endian()
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YVersion> {
        YVersion::from_big_endian(b)
    }
}
