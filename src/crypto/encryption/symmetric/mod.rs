use rust_crypto::aes::{KeySize, ctr};
use rust_crypto::symmetriccipher::SynchronousStreamCipher;

pub struct YSymmetricEncryption;

impl YSymmetricEncryption {
  pub fn encrypt(key: &[u8], iv: &[u8], plain: &[u8]) -> Vec<u8> {
    let mut stream_cypher = ctr(KeySize::KeySize256, key, iv);
    let mut cypher: Vec<u8> = Vec::new();
    stream_cypher.process(plain, cypher.as_mut_slice());
    cypher
  }

  pub fn decrypt(key: &[u8], iv: &[u8], cyph: &[u8]) -> Vec<u8> {
    YSymmetricEncryption::encrypt(key, iv, cyph)
  }
}
