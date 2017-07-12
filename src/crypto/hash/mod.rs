use libc::{size_t, c_int, c_ulonglong};
use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use itertools::Itertools;
use errors::*;
use length::check_length;
use size::check_size;
use crypto::utils::init;
use crypto::utils::check_binary_size;
use std::io::Cursor;
use std::ops::Index;
use std::iter::Iterator;

pub const HASH_SIZE: usize =  32;

pub fn check_hash_size(h: &Hash) -> Result<()> {
   check_binary_size(h.as_slice(), HASH_SIZE as u32) 
}

pub type Hash = Vec<u8>;

extern {
    pub fn crypto_hash_sha256_bytes() -> size_t;

    pub fn crypto_hash_sha256(h: *mut [u8; HASH_SIZE],
                              m: *const u8,
                              mlen: c_ulonglong) -> c_int;
}

pub fn _hash(msg: &[u8]) -> Hash {
    unsafe {
        let mut h = [0; HASH_SIZE];
        crypto_hash_sha256(&mut h, msg.as_ptr(), msg.len() as c_ulonglong);
        let mut v = Vec::new();
        v.extend_from_slice(&h[..]);
        v
    }
}

pub fn hash(msg: &[u8]) -> Result<Hash> {
    init()?;
    check_size(msg)?;
    let h = _hash(msg);
    Ok(h)
}

pub fn nonce_from_u32(n: u32) -> Result<Hash> {
    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, n);
    hash(&buf[..])
}

pub fn random_u32_from_seed(seed: &Hash, max: u32) -> Result<u32> {
    check_hash_size(seed)?;
    let mut c = Cursor::new(seed.to_owned());
    let n = c.read_u32::<BigEndian>()? % max;
    Ok(n)
}

#[derive(Clone, Debug)]
pub struct Hashes {
    length: u32,
    idx: u32,
    items: Vec<Hash>,
}

impl Hashes {
    pub fn new() -> Hashes {
        Hashes {
            length: 0,
            idx: 0,
            items: Vec::new(),
        }
    }

    pub fn from_vec(items: &Vec<Hash>) -> Result<Hashes> {
        check_length(items)?;
        let len = items.len();
        Ok(Hashes {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn to_vec(&self) -> Vec<Hash> {
        self.items.to_owned()
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Hash) {
        self.items.push(item);
        self.length += 1;
    }

    pub fn check(&self) -> Result<()> {
        let len = self.length;
        if self.idx >= len {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        if len != self.items.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }
}

impl Index<usize> for Hashes {
    type Output = Hash;

    fn index(&self, idx: usize) -> &Hash {
        self.items.index(idx)
    }
}

impl Iterator for Hashes {
    type Item = Hash;

    fn next(&mut self) -> Option<Hash> {
        match self.check() {
            Ok(_) => {
                let item = self.items[self.idx as usize].to_owned();
                self.idx += 1;
                Some(item)
            },
            Err(_) => { None },
        }
    }
}

pub fn unique_hashes(hashes: &Vec<Hash>) -> Result<Vec<Hash>> {
    Ok(Hashes::from_vec(hashes)?.unique().collect())
}

pub fn check_unique_hashes(hashes: &Vec<Hash>) -> Result<()> {
    let uniques: Vec<Hash> = Hashes::from_vec(hashes)?.unique().collect();
    if uniques.len() != hashes.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
