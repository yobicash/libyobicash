use typenum::consts::U64;
use generic_array::GenericArray;
use errors::*;
use sha2::Sha512;
use hmac::{Mac, Hmac};
use serialize::hex::{FromHex, ToHex};
use crypto::key::YKey;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YMACCode(pub GenericArray<u8, U64>);

impl YMACCode {
  pub fn from_bytes(b: &[u8]) -> YResult<YMACCode> {
    if b.len() != 64 {
      return Err(YErrorKind::InvalidLength.into());
    }
    Ok(YMACCode(*GenericArray::from_slice(&b[..])))
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(self.0.as_slice());
    b
  }

  pub fn from_hex(s: &str) -> YResult<YMACCode> {
    let buf = s.from_hex()?;
    YMACCode::from_bytes(buf.as_slice())
  }

  pub fn to_hex(&self) -> String {
    self.to_bytes()[..].to_hex()
  }
}

pub struct YMAC(pub Hmac<Sha512>);

impl YMAC {
  pub fn new(key: YKey) -> YMAC {
    YMAC(Hmac::<Sha512>::new(&key.to_bytes()[..]))
  }

  pub fn update(&mut self, msg: &[u8]) {
    self.0.input(msg)
  }

  pub fn result(self) -> YMACCode {
    YMACCode::from_bytes(self.0.result().code()).unwrap()
  }
 
  pub fn mac(key: YKey, msg: &[u8]) -> YMACCode {
    let mut m = YMAC::new(key);
    m.update(msg);
    m.result()
  }

  pub fn verify(self, code: YMACCode) -> bool {
    self.0.verify(&code.to_bytes()[..])
  }
}
