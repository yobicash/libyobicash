use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use errors::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPoSt {
    pub id_tx: YDigest64,
    pub difficulty: u32,
    pub nonce: u32,
    pub chunks: Vec<u8>,
    pub digest: YDigest64,
}

impl YPoSt {
    pub fn new(id_tx: YDigest64, diff: u32, nonce: u32, chunks: &Vec<u8>) -> YResult<YPoSt> {
        if chunks.len() != diff as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        let post = YPoSt {
            id_tx: id_tx,
            difficulty: diff,
            nonce: nonce,
            chunks: chunks.clone(),
            digest: YSHA512::hash(chunks),
        };
        Ok(post)
    }

    pub fn verify(&self) -> bool {
        if self.chunks.len() != self.difficulty as usize {
            return false;
        }
        if self.digest != YSHA512::hash(self.chunks.as_slice()) {
            return false;
        }
        true
    }

    pub fn check(&self) -> YResult<()> {
        if self.chunks.len() != self.difficulty as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        if self.digest != YSHA512::hash(self.chunks.as_slice()) {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        Ok(()) 
    }
}
