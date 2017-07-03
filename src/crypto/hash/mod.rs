use sodiumoxide::crypto::hash as _hash;
use errors::*;
use size::check_size;
use crypto::utils::init;
use crypto::utils::check_binary_size;

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
