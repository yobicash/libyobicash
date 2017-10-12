use typenum::consts::{U32, U64};
use generic_array::GenericArray;
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::hash::YHash32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YKey32(pub GenericArray<u8, U32>);

impl YKey32 {
    pub fn from_bytes(b: &[u8]) -> YResult<YKey32> {
        if b.len() != 32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YKey32(*GenericArray::from_slice(&b[..])))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.0.as_slice());
        b
    }

    pub fn from_hex(s: &str) -> YResult<YKey32> {
        let buf = s.from_hex()?;
        YKey32::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YKey64(pub GenericArray<u8, U64>);

impl YKey64 {
    pub fn from_bytes(b: &[u8]) -> YResult<YKey64> {
        if b.len() != 64 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(YKey64(*GenericArray::from_slice(&b[..])))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.0.as_slice());
        b
    }

    pub fn from_hex(s: &str) -> YResult<YKey64> {
        let buf = s.from_hex()?;
        YKey64::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> String {
        self.to_bytes()[..].to_hex()
    }

    // NB: change the name, it's a little bit unproper
    pub fn reduce(self) -> YKey32 {
        YKey32(YHash32::hash(self.to_bytes().as_slice()).0)
    }
}
