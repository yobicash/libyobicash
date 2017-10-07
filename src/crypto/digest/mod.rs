use typenum::consts::U64;
use generic_array::GenericArray;
use serialize::hex::{FromHex, ToHex};
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YDigest(pub GenericArray<u8, U64>);

impl YDigest {
  pub fn from_bytes(b: &[u8]) -> YResult<YDigest> {
    if b.len() != 64 {
      return Err(YErrorKind::InvalidLength.into());
    }
    Ok(YDigest(*GenericArray::from_slice(&b[..])))
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(self.0.as_slice());
    b
  }

  pub fn from_hex(s: &str) -> YResult<YDigest> {
    let buf = s.from_hex()?;
    YDigest::from_bytes(buf.as_slice())
  }

  pub fn to_hex(&self) -> String {
    self.to_bytes()[..].to_hex()
  }
}

