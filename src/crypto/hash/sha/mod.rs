use sha2::{Digest, Sha256, Sha512};
use crypto::hash::digest::*;

#[derive(Default)]
pub struct YSHA256(pub Sha256);

impl YSHA256 {
    pub fn new() -> YSHA256 {
        YSHA256::default()
    }

    pub fn update(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    pub fn digest(self) -> YDigest32 {
        YDigest32(self.0.result())
    }

    pub fn hash(msg: &[u8]) -> YDigest32 {
        YDigest32(Sha256::digest(msg))
    }
}

#[derive(Default)]
pub struct YSHA512(pub Sha512);

impl YSHA512 {
    pub fn new() -> YSHA512 {
        YSHA512::default()
    }

    pub fn update(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    pub fn digest(self) -> YDigest64 {
        YDigest64(self.0.result())
    }

    pub fn hash(msg: &[u8]) -> YDigest64 {
        YDigest64(Sha512::digest(msg))
    }
}
