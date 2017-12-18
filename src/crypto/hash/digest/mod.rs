use typenum::consts::{U32, U64};
use generic_array::GenericArray;
use serialize::hex::{FromHex, ToHex};
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
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

    pub fn to_u32(&self) -> u32 {
        let mut n = self.0[0] as u32;
        for i in 1..32 {
            n ^= self.0[i] as u32;
        }
        n
    }

    pub fn to_u64(&self) -> u64 {
        let mut n = self.0[0] as u64;
        for i in 1..64 {
            n ^= self.0[i] as u64;
        }
        n
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
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

    pub fn to_u32(&self) -> u32 {
        let mut n = self.0[0] as u32;
        for i in 1..32 {
            n ^= self.0[i] as u32;
        }
        n
    }

    pub fn to_u64(&self) -> u64 {
        let mut n = self.0[0] as u64;
        for i in 1..64 {
            n ^= self.0[i] as u64;
        }
        n
    }
}
