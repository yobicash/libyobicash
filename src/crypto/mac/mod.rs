use typenum::consts::U64;
use generic_array::GenericArray;
use sha2::Sha512;
use hmac::{Mac, Hmac};
use crypto::key::YKey;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YMACCode(pub GenericArray<u8, U64>);

impl YMACCode {
  pub fn from_bytes(b: &[u8]) -> Option<YMACCode> {
    if b.len() != 64 {
      return None;
    }
    Some(YMACCode(*GenericArray::from_slice(&b[..])))
  }

  pub fn to_bytes(&self) -> [u8; 64] {
    let mut b = [0u8; 64];
    for i in 0..64 {
      b[i] = self.0[i]
    }
    b
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
