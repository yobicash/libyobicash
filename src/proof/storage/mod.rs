use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use errors::*;
use std::io::{Write, Cursor, Read};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
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

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = Vec::new();
        buf.write(&self.id_tx.to_bytes()[..])?;
        buf.write_u32::<BigEndian>(self.difficulty)?;
        buf.write_u32::<BigEndian>(self.nonce)?;
        buf.write(self.chunks.as_slice())?;
        buf.write(&self.digest.to_bytes()[..])?;
        Ok(buf)
    }

    pub fn from_bytes(buf: &[u8]) -> YResult<YPoSt> {
        if buf.len() < 136 {
            return Err(YErrorKind::InvalidLength.into());
        }
        let mut reader = Cursor::new(buf);
        let mut post = YPoSt::default();
        let mut id_tx_buf = [0u8; 64];
        reader.read_exact(&mut id_tx_buf[..])?;
        post.id_tx = YDigest64::from_bytes(&id_tx_buf[..])?;
        post.difficulty = reader.read_u32::<BigEndian>()?;
        post.nonce = reader.read_u32::<BigEndian>()?;
        let mut chunks = Vec::new();
        for _ in 0..post.difficulty {
            chunks.push(0);
        }
        reader.read_exact(&mut chunks.as_mut_slice())?;
        post.chunks = chunks;
        let mut digest_buf = [0u8; 64];
        reader.read_exact(&mut digest_buf[..])?;
        post.digest = YDigest64::from_bytes(&digest_buf[..])?;
        Ok(post)
    }

    pub fn from_hex(s: &str) -> YResult<YPoSt> {
        let buf = s.from_hex()?;
        YPoSt::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
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
