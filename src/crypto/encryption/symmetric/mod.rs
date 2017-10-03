use typenum::consts::U64;
use generic_array::GenericArray;
use rust_crypto::aes::{KeySize, ctr};
use rust_crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::key::YKey;

pub struct YSymmetricEncryption;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YIV(pub GenericArray<u8, U64>);

impl YIV {
  pub fn from_bytes(b: &[u8]) -> Option<YIV> {
    if b.len() != 64 {
      return None;
    }
    Some(YIV(*GenericArray::from_slice(&b[..])))
  }

  pub fn to_bytes(&self) -> [u8; 64] {
    let mut b = [0u8; 64];
    for i in 0..64 {
      b[i] = self.0[i]
    }
    b
  }
}

impl YSymmetricEncryption {
  pub fn encrypt(key: YKey, iv: YIV, plain: &[u8]) -> Vec<u8> {
    let _key = &key.to_bytes()[..];
    let _iv = &iv.to_bytes()[..];
    let mut stream_cypher = ctr(KeySize::KeySize256, _key, _iv);
    let mut cypher: Vec<u8> = Vec::new();
    stream_cypher.process(plain, cypher.as_mut_slice());
    cypher
  }

  pub fn decrypt(key: YKey, iv: YIV, cyph: &[u8]) -> Vec<u8> {
    YSymmetricEncryption::encrypt(key, iv, cyph)
  }
}
