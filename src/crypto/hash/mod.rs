use typenum::consts::{U32, U64};
use generic_array::GenericArray;
use sha2::{Digest, Sha256, Sha512};
use serialize::hex::{FromHex, ToHex};
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YDigest32(pub GenericArray<u8, U32>);

impl YDigest32 {
    pub fn from_bytes(b: &[u8]) -> YResult<YDigest32> {
        if b.len() != 32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YDigest32(*GenericArray::from_slice(&b[..])))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.0.as_slice());
        b
    }

    pub fn from_hex(s: &str) -> YResult<YDigest32> {
        let buf = s.from_hex()?;
        YDigest32::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

#[derive(Default)]
pub struct YHash32(pub Sha256);

impl YHash32 {
    pub fn new() -> YHash32 {
        YHash32::default()
    }

    pub fn update(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    pub fn digest(self) -> YDigest32 {
        YDigest32(self.0.result())
    }

    pub fn hash(msg: &[u8]) -> YDigest32 {
        YDigest32(Sha256::digest(msg))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YDigest64(pub GenericArray<u8, U64>);

impl YDigest64 {
    pub fn from_bytes(b: &[u8]) -> YResult<YDigest64> {
        if b.len() != 64 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YDigest64(*GenericArray::from_slice(&b[..])))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.0.as_slice());
        b
    }

    pub fn from_hex(s: &str) -> YResult<YDigest64> {
        let buf = s.from_hex()?;
        YDigest64::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

#[derive(Default)]
pub struct YHash64(pub Sha512);

impl YHash64 {
    pub fn new() -> YHash64 {
        YHash64::default()
    }

    pub fn update(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    pub fn digest(self) -> YDigest64 {
        YDigest64(self.0.result())
    }

    pub fn hash(msg: &[u8]) -> YDigest64 {
        YDigest64(Sha512::digest(msg))
    }
}
