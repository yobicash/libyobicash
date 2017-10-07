use typenum::consts::U64;
use generic_array::GenericArray;
use serialize::hex::{FromHex, ToHex};
use errors::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YKey(pub GenericArray<u8, U64>);

impl YKey {
  pub fn from_bytes(b: &[u8]) -> YResult<YKey> {
    if b.len() != 64 {
      return Err(YErrorKind::InvalidLength.into());
    }
    Ok(YKey(*GenericArray::from_slice(&b[..])))
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(self.0.as_slice());
    b
  }

  pub fn from_hex(s: &str) -> YResult<YKey> {
    let buf = s.from_hex()?;
    YKey::from_bytes(buf.as_slice())
  }

  pub fn to_hex(&self) -> String {
    self.to_bytes()[..].to_hex()
  }
}
