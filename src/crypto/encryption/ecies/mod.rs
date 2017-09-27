use curve25519_dalek::edwards::IsIdentity;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::encryption::symmetric::YSymmetricEncryption;
use crypto::mac::{YMAC, YMACResult};

pub struct YECIES {
  g: YPoint,
  sk: YScalar,
}

impl YECIES {
  pub fn random() -> YECIES {
    YECIES {
      g: YPoint::random(),
      sk: YScalar::random(),
    }
  }

  pub fn from_g(g: &YPoint) -> YECIES {
    YECIES {
      g: *g,
      sk: YScalar::random(),
    }
  }

  pub fn public_key(&self) -> YPoint {
    &self.g*&self.sk
  }

  pub fn derive_key(&self, other: &YPoint) -> Option<Vec<u8>> {
    let _sk = other*&self.sk;
    if !_sk.is_identity() {
      let mut key: Vec<u8> = Vec::new();
      key.extend_from_slice(&_sk.to_bytes()[..]);
      Some(key)
    } else {
      None
    }
  }

  pub fn encrypt(&self, other: &YPoint, iv: &[u8], plain: &[u8]) -> Option<Vec<u8>> {
    if let Some(key) = self.derive_key(other) {
      let cyph = YSymmetricEncryption::encrypt(key.as_slice(), iv, plain);
      Some(cyph)
    } else {
      None
    }
  }

  pub fn decrypt(&self, other: &YPoint, iv: &[u8], cyph: &[u8]) -> Option<Vec<u8>> {
    if let Some(key) = self.derive_key(other) {
      let plain = YSymmetricEncryption::decrypt(key.as_slice(), iv, cyph);
      Some(plain)
    } else {
      None
    }
  }

  pub fn authenticate(&self, other: &YPoint, cyph: &[u8]) -> Option<YMACResult> {
    if let Some(key) = self.derive_key(other) {
      let tag = YMAC::mac(key.as_slice(), cyph);
      Some(tag)
    } else {
      None
    }
  }

  pub fn verify(&self, other: &YPoint, cyph: &[u8], tag: &YMACResult) -> Option<bool> {
    if let Some(key) = self.derive_key(other) {
        let mut mac = YMAC::new(key.as_slice());
        mac.update(cyph);
        Some(mac.verify(tag))
    } else {
      None
    }
  }

  pub fn encrypt_and_authenticate(&self, other: &YPoint, iv: &[u8], plain: &[u8]) -> Option<(Vec<u8>, YMACResult)> {
    let _cyph = self.encrypt(other, iv, plain);
    if _cyph.is_none() { return None }
    let cyph = _cyph.unwrap();
    let _tag = self.authenticate(other, cyph.as_slice());
    if _tag.is_none() { return None }
    let tag = _tag.unwrap();
    Some((cyph, tag))
  }

  pub fn verify_and_decrypt(&self, other: &YPoint, iv: &[u8], cyph: &[u8], tag: &YMACResult) -> Option<Vec<u8>> {
    if let Some(verified) = self.verify(other, cyph, tag) {
      if verified {
          self.decrypt(other, iv, cyph)
      } else {
        None
      }
    } else {
      None
    }
  }
}

