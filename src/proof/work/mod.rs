use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use crypto::hash::balloon::{YBalloonParams, YBalloon512};
use errors::*;
use std::io::Cursor;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YTarget(pub YDigest64);

impl YTarget {
    pub fn new(bits: u32) -> YResult<YTarget> {
        let n = u64::max_value() >> (bits as usize);
        let _b = [1u8; 64];
        let mut b = Vec::new();
        b.extend_from_slice(&_b[..]);
        b.write_u64::<BigEndian>(n)?;
        let target = YTarget(YDigest64::from_bytes(&b[..])?);
        Ok(target)
    }

    pub fn bits(&self) -> YResult<u32> {
        let mut reader = Cursor::new(self.0.to_bytes());
        let n = reader.read_u32::<BigEndian>()?;
        Ok(n.leading_zeros() as u32)
    }

    pub fn digest(&self) -> YDigest64 {
        self.0
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct YPoW {
    pub digest_post: YDigest64,
    pub difficulty_post: u32,
    pub nonce: u32,
    pub params: Option<YBalloonParams>,
    pub memory: u64,
    pub digest: Option<YDigest64>,
}

impl YPoW {
    pub fn new(
        digest_post: YDigest64,
        difficulty_post: u32,
        increment: u32) -> YResult<YPoW> {
        if difficulty_post < 3 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        let params_post = YBalloonParams::new(difficulty_post, difficulty_post, difficulty_post)?;
        let balloon_post = YBalloon512::new(digest_post, params_post)?;
        let mut memory = 0;
        if increment > 0 {
            let mut params = params_post.clone();
            params.s_cost = params.s_cost + increment;
            params.t_cost = params.t_cost + increment;
            params.delta = params.delta + increment;
            params.check()?;
            let balloon_extra = YBalloon512::new(digest_post, params)?;
            memory = balloon_extra.memory() - balloon_post.memory();
        }
        let pow = YPoW {
            digest_post: digest_post,
            difficulty_post: difficulty_post,
            nonce: 0,
            params: None,
            memory: memory,
            digest: None,
        };
        Ok(pow)
    }

    pub fn params_post(&self) -> YResult<YBalloonParams> {
        if self.difficulty_post < 3 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        let diff = self.difficulty_post;
        YBalloonParams::new(diff, diff, diff)
    }

    pub fn balloon_post(&self) -> YResult<YBalloon512> {
        let params_post = self.params_post()?;
        YBalloon512::new(self.digest_post, params_post)
    }

    pub fn memory(&self) -> YResult<u64> {
        let balloon_post = self.balloon_post()?;
        let mut memory = 0;
        if let Some(params) = self.params {
            let balloon_extra = YBalloon512::new(self.digest_post, params)?;
            memory = balloon_extra.memory() - balloon_post.memory();
        }
        Ok(memory)
    }

    pub fn target(&self) -> YResult<YTarget> {
        YTarget::new(self.target_bits())
    }

    pub fn target_bits(&self) -> u32 {
        self.difficulty_post
    }

    pub fn params(&self) -> YResult<YBalloonParams> {
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
        let mut params = YBalloonParams::default();

        if let Some(params_extra) = self.params {
            params = params_extra;
        } else {
            params = self.params_post()?;
        }

        Ok(params)
    }
    
    pub fn mine(&mut self, msg: &[u8]) -> YResult<()> {
        let params = self.params()?;
        let target = self.target()?.digest();
        let mut nonce = 0;
        let mut not_found = true; 

        while not_found {
            let mut seed_buf = Vec::new();
            seed_buf.extend_from_slice(self.digest_post.to_bytes().as_slice());
            let mut nonce_buf = Vec::new();
            nonce_buf.write_u32::<BigEndian>(nonce)?;
            seed_buf.extend_from_slice(nonce_buf.as_slice());
            let seed = YSHA512::hash(seed_buf.as_slice());
            let balloon = YBalloon512::new(seed, params)?;
            let digest = balloon.hash(msg)?;
            if digest < target {
                self.digest = Some(digest);
                not_found = false;
            } else {
                if nonce == u32::max_value() {
                    return Err(YErrorKind::PoWDigestNotFound.into());
                }
                nonce = nonce + 1;
            }
        }

        return Ok(())
    }

    pub fn verify(&self, msg: &[u8]) -> YResult<bool> {
        if self.difficulty_post < 3 {
            return Ok(false);
        }
        if self.memory != self.memory()? {
            return Ok(false);
        }
        if let Some(digest) = self.digest {
            let target = self.target()?.digest();
            if digest >= target {
                return Ok(false)
            }
            let mut seed_buf = Vec::new();
            seed_buf.extend_from_slice(self.digest_post.to_bytes().as_slice());
            let mut nonce_buf = Vec::new();
            nonce_buf.write_u32::<BigEndian>(self.nonce)?;
            seed_buf.extend_from_slice(nonce_buf.as_slice());
            let seed = YSHA512::hash(seed_buf.as_slice());
            let params = self.params()?;
            let balloon = YBalloon512::new(seed, params)?;
            let _digest = balloon.hash(msg)?;
            if digest != _digest {
                return Ok(false);
            }
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn check(&self, msg: &[u8]) -> YResult<()> {
        if self.difficulty_post < 3 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        if self.memory != self.memory()? {
            return Err(YErrorKind::InvalidAmount.into());
        }
        if let Some(digest) = self.digest {
            let target = self.target()?.digest();
            if digest >= target {
                return Err(YErrorKind::InvalidPoWSolution.into());
            }
            let mut seed_buf = Vec::new();
            seed_buf.extend_from_slice(self.digest_post.to_bytes().as_slice());
            let mut nonce_buf = Vec::new();
            nonce_buf.write_u32::<BigEndian>(self.nonce)?;
            seed_buf.extend_from_slice(nonce_buf.as_slice());
            let seed = YSHA512::hash(seed_buf.as_slice());
            let params = self.params()?;
            let balloon = YBalloon512::new(seed, params)?;
            let _digest = balloon.hash(msg)?;
            if digest != _digest {
                return Err(YErrorKind::InvalidPoWSolution.into());
            }
            return Ok(());
        } else {
            return Err(YErrorKind::IncompletePoW.into());
        }
    }
}
