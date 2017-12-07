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
        if bits > 63 {
            return Err(YErrorKind::InvalidTargetBits.into())
        }
        let n = u64::max_value() >> (bits as usize);
        let mut b = Vec::new();
        b.write_u64::<BigEndian>(n)?;
        for _ in 0..56 {
            b.push(255u8);
        }
        let target = YTarget(YDigest64::from_bytes(&b[..])?);
        Ok(target)
    }

    pub fn bits(&self) -> YResult<u32> {
        let mut reader = Cursor::new(self.0.to_bytes());
        let n = reader.read_u64::<BigEndian>()?;
        let bits = n.leading_zeros() as u32;
        Ok(bits)
    }

    pub fn digest(&self) -> YDigest64 {
        self.0
    }
}


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct YPoW {
    pub post_digest: YDigest64,
    pub post_difficulty: u32,
    pub nonce: u32,
    pub params: Option<YBalloonParams>,
    pub memory: u64,
    pub seed: Vec<u8>,
    pub digest: Option<YDigest64>,
}

impl YPoW {
    pub fn new(
        post_digest: YDigest64,
        post_difficulty: u32,
        increment: u32) -> YResult<YPoW> {
        if post_difficulty < 3 || post_difficulty > 63 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        if increment + post_difficulty > 63 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        let post_params = YBalloonParams::new(post_difficulty, post_difficulty, post_difficulty)?;
        let post_balloon = YBalloon512::new(post_digest, post_params)?;
        let mut memory = 0;
        let mut params = None;
        if increment > 0 {
            let mut extra_params = post_params.clone();
            extra_params.s_cost = extra_params.s_cost + increment;
            extra_params.t_cost = extra_params.t_cost + increment;
            extra_params.delta = extra_params.delta + increment;
            extra_params.check()?;
            let extra_balloon = YBalloon512::new(post_digest, extra_params)?;
            memory = extra_balloon.memory() - post_balloon.memory();
            params = Some(extra_params);
        }
        let pow = YPoW {
            post_digest: post_digest,
            post_difficulty: post_difficulty,
            nonce: 0,
            params: params,
            memory: memory,
            seed: Vec::new(),
            digest: None,
        };
        Ok(pow)
    }

    pub fn post_params(&self) -> YResult<YBalloonParams> {
        if self.post_difficulty < 3 || self.post_difficulty > 63 {
            return Err(YErrorKind::InvalidDifficulty.into());
        }
        let diff = self.post_difficulty;
        YBalloonParams::new(diff, diff, diff)
    }

    pub fn post_balloon(&self) -> YResult<YBalloon512> {
        let post_params = self.post_params()?;
        YBalloon512::new(self.post_digest, post_params)
    }

    pub fn memory(&self) -> YResult<u64> {
        let post_balloon = self.post_balloon()?;
        let mut memory = 0;
        if let Some(params) = self.params {
            params.check()?;
            if params.s_cost > 63 || params.t_cost > 63 || params.delta > 63 {
                return Err(YErrorKind::InvalidDifficulty.into());
            }
            let extra_balloon = YBalloon512::new(self.post_digest, params)?;
            memory = extra_balloon.memory() - post_balloon.memory();
        }
        Ok(memory)
    }

    pub fn target(&self) -> YResult<YTarget> {
        YTarget::new(self.target_bits())
    }

    pub fn target_bits(&self) -> u32 {
        self.post_difficulty
    }

    pub fn params(&self) -> YResult<YBalloonParams> {
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
        let mut params = YBalloonParams::default();

        if let Some(extra_params) = self.params {
            extra_params.check()?;
            if extra_params.s_cost > 63 || extra_params.t_cost > 63 || extra_params.delta > 63 {
                return Err(YErrorKind::InvalidDifficulty.into());
            }
            params = extra_params;
        } else {
            params = self.post_params()?;
        }

        Ok(params)
    }
    
    pub fn mine(&mut self, msg: &[u8]) -> YResult<()> {
        let params = self.params()?;
        let target = self.target()?.digest();
        let mut nonce = 0;

        'mining: loop {
            let salt = YSHA512::hash(self.post_digest.to_bytes().as_slice());
            let balloon = YBalloon512::new(salt, params)?;
            let mut digest_buf = Vec::new();
            digest_buf.extend_from_slice(msg);
            digest_buf.write_u32::<BigEndian>(nonce)?;
            let digest = balloon.hash(digest_buf.as_slice())?;
            if digest < target {
                let mut seed = Vec::new();
                seed.extend_from_slice(msg);
                self.seed = seed;
                self.nonce = nonce;
                self.digest = Some(digest);
                break 'mining;
            } else {
                if nonce == u32::max_value() {
                    break 'mining;
                }
                nonce = nonce + 1;
            }
        }

        return Ok(())
    }

    pub fn verify(&self) -> YResult<bool> {
        let params = self.params()?;
        if self.memory != self.memory()? {
            return Err(YErrorKind::InvalidAmount.into());
        }
        if let Some(digest) = self.digest {
            let target = self.target()?.digest();
            if digest >= target {
                return Ok(false)
            }
            let salt = YSHA512::hash(self.post_digest.to_bytes().as_slice());
            let balloon = YBalloon512::new(salt, params)?;
            let mut digest_buf = Vec::new();
            digest_buf.extend_from_slice(self.seed.as_slice());
            digest_buf.write_u32::<BigEndian>(self.nonce)?;
            let _digest = balloon.hash(digest_buf.as_slice())?;
            if digest != _digest {
                return Ok(false);
            }
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn check(&self) -> YResult<()> {
        let params = self.params()?;
        if self.memory != self.memory()? {
            return Err(YErrorKind::InvalidAmount.into());
        }
        if let Some(digest) = self.digest {
            let target = self.target()?.digest();
            if digest >= target {
                return Err(YErrorKind::InvalidPoWSolution.into());
            }
            let salt = YSHA512::hash(self.post_digest.to_bytes().as_slice());
            let balloon = YBalloon512::new(salt, params)?;
            let mut digest_buf = Vec::new();
            digest_buf.extend_from_slice(self.seed.as_slice());
            digest_buf.write_u32::<BigEndian>(self.nonce)?;
            let _digest = balloon.hash(digest_buf.as_slice())?;
            if digest != _digest {
                return Err(YErrorKind::InvalidPoWSolution.into());
            }
            return Ok(());
        } else {
            return Err(YErrorKind::IncompletePoW.into());
        }
    }
}
