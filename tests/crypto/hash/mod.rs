use libyobicash::crypto::hash::*;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn hash_succ() {
    let len = 1000000;
    let msg = randombytes(len).unwrap();
    let h = hash(msg.as_slice()).unwrap();
    let res = check_hash_size(&h);
    assert!(res.is_ok()) 
}

#[test]
fn hash_fail() {
    let len = HASH_SIZE - 1;
    let h = randombytes(len).unwrap();
    let res = check_hash_size(&h);
    assert!(res.is_err()) 
}

#[test]
fn nonce_from_u32_succ() {
    let n = 10;
    let nonce = nonce_from_u32(n).unwrap();
    assert_eq!(nonce.len(), HASH_SIZE)
}

#[test]
fn unique_hashes_succ() {
    let len = 10;
    let mut hashes: Vec<Hash> = Vec::new();
    for i in 0..len {
        let hash = nonce_from_u32(i).unwrap();
        hashes.push(hash);
    }
    let res = check_unique_hashes(&hashes);
    assert!(res.is_ok())
}

#[test]
fn unique_hashes_fail() {
    let len = 10;
    let hash = randombytes(HASH_SIZE).unwrap();
    let hashes: Vec<Hash> = repeat(hash).take(len).collect();
    let res = check_unique_hashes(&hashes);
    assert!(res.is_err())
}
