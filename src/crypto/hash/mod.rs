use sodiumoxide::crypto::hash as _hash;
use itertools::Itertools;
use errors::*;
use length::check_length;
use size::check_size;
use crypto::utils::init;
use crypto::utils::check_binary_size;
use std::ops::Index;
use std::iter::Iterator;

pub const HASH_SIZE: usize = 32;

pub type Hash = Vec<u8>;

pub fn check_hash_size(h: &Hash) -> Result<()> {
   check_binary_size(h.as_slice(), HASH_SIZE as u32) 
}

pub fn hash(msg: &[u8]) -> Result<Hash> {
    init()?;
    check_size(msg)?;
    Ok(_hash::sha256::hash(msg).as_ref().to_vec())
}

#[derive(Clone, Debug)]
pub struct Hashes {
    length: u32,
    idx: u32,
    items: Vec<Hash>,
}

impl Hashes {
    pub fn new(items: &Vec<Hash>) -> Result<Hashes> {
        check_length(items)?;
        let len = items.len();
        Ok(Hashes {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Hash) {
        self.items.push(item)
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
    Ok(Hashes::new(hashes)?.unique().collect())
}

pub fn check_unique_hashes(hashes: &Vec<Hash>) -> Result<()> {
    let uniques: Vec<Hash> = Hashes::new(hashes)?.unique().collect();
    if uniques.len() != hashes.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
