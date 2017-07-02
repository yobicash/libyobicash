use libyobicash::crypto::hash::*;
use libyobicash::crypto::utils::randombytes;

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
