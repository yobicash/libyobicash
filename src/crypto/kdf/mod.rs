use sha2::Sha512;
use hkdf::Hkdf;

pub struct YKDF(pub Hkdf<Sha512>);

impl YKDF {
  const LENGTH: usize = 64;

  pub fn extract(salt: &[u8], ikm: &[u8]) -> YKDF {
    YKDF(Hkdf::<Sha512>::new(ikm, salt))
  }

  pub fn expand(&mut self, info: &[u8], len: usize) -> Vec<u8> {
    // TODO
    assert!(len <= 255*YKDF::LENGTH);
    self.0.derive(info, len)
  }

  pub fn kdf(salt: &[u8], ikm: &[u8], info: &[u8], len: usize) -> Vec<u8> {
    let mut kdf = YKDF::extract(salt, ikm);
    kdf.expand(info, len)
  }
}

