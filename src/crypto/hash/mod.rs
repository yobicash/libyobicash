use sha2::{Digest, Sha512};
use crypto::digest::YDigest;

#[derive(Default)]
pub struct YHash(pub Sha512);

impl YHash {
  pub fn new() -> YHash {
    YHash::default()
  }

  pub fn update(&mut self, msg: &[u8]) {
    self.0.input(msg)
  }

  pub fn digest(self) -> YDigest {
    self.0.result() 
  }

  pub fn hash(msg: &[u8]) -> YDigest {
    Sha512::digest(msg)
  }
} 
