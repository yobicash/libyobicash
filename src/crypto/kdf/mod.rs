use sha2::Sha512;
use hkdf::Hkdf;
use crypto::key::YKey;

pub struct YKDF(pub Hkdf<Sha512>);

impl YKDF {
  const LENGTH: usize = 64;

  pub fn extract(salt: &[u8], ikm: &[u8]) -> YKDF {
    YKDF(Hkdf::<Sha512>::new(ikm, salt))
  }

  pub fn expand(&mut self, info: &[u8]) -> Vec<u8> {
    self.0.derive(info, YKDF::LENGTH)
  }

  pub fn kdf(salt: &[u8], ikm: &[u8], info: &[u8]) -> YKey {
    let mut kdf = YKDF::extract(salt, ikm);
    let key_buf = kdf.expand(info);
    YKey::from_bytes(key_buf.as_slice()).unwrap() 
  }
}

