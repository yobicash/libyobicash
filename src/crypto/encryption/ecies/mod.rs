use curve25519_dalek::edwards::IsIdentity;
use crypto::elliptic::point::YPoint;
use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
use crypto::key::YKey;
use crypto::encryption::symmetric::{YSymmetricEncryption, YIV};
use crypto::mac::{YMAC, YMACCode};

pub struct YECIES(pub YSecretKey);

impl YECIES {
  pub fn new(sk: YSecretKey) -> YECIES {
    YECIES(sk)
  }

  pub fn random() -> YECIES {
    YECIES(YSecretKey::random())
  }

  pub fn from_g(g: YPoint) -> YECIES {
    YECIES(YSecretKey::from_g(g))
  }

  pub fn public_key(&self) -> YPublicKey {
    self.0.public_key()
  }

  pub fn derive_key(&self, other: &YPublicKey) -> Option<YKey> {
    let _key = &other.pk*&self.0.sk;
    if !_key.is_identity() {
      YKey::from_bytes(&_key.to_bytes()[..])
    } else {
      None
    }
  }

  pub fn encrypt(&self, other: &YPublicKey, iv: YIV, plain: &[u8]) -> Option<Vec<u8>> {
    if let Some(key) = self.derive_key(other) {
      let cyph = YSymmetricEncryption::encrypt(key, iv, plain);
      Some(cyph)
    } else {
      None
    }
  }

  pub fn decrypt(&self, other: &YPublicKey, iv: YIV, cyph: &[u8]) -> Option<Vec<u8>> {
    if let Some(key) = self.derive_key(other) {
      let plain = YSymmetricEncryption::decrypt(key, iv, cyph);
      Some(plain)
    } else {
      None
    }
  }

  pub fn authenticate(&self, other: &YPublicKey, cyph: &[u8]) -> Option<YMACCode> {
    if let Some(key) = self.derive_key(other) {
      let tag = YMAC::mac(key, cyph);
      Some(tag)
    } else {
      None
    }
  }

  pub fn verify(&self, other: &YPublicKey, cyph: &[u8], tag: YMACCode) -> Option<bool> {
    if let Some(key) = self.derive_key(other) {
        let mut mac = YMAC::new(key);
        mac.update(cyph);
        Some(mac.verify(tag))
    } else {
      None
    }
  }

  pub fn encrypt_and_authenticate(&self, other: &YPublicKey, iv: YIV, plain: &[u8]) -> Option<(Vec<u8>, YMACCode)> {
    let _cyph = self.encrypt(other, iv, plain);
    if _cyph.is_none() { return None }
    let cyph = _cyph.unwrap();
    let _tag = self.authenticate(other, cyph.as_slice());
    if _tag.is_none() { return None }
    let tag = _tag.unwrap();
    Some((cyph, tag))
  }

  pub fn verify_and_decrypt(&self, other: &YPublicKey, iv: YIV, cyph: &[u8], tag: YMACCode) -> Option<Vec<u8>> {
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
