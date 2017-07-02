use libyobicash::crypto::merkle::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::check_hash_size;
use libyobicash::crypto::utils::randombytes;

#[test]
fn merkle_succ() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE).unwrap();
        leafs.push(leaf);
    }
    let res = merkle_root(&leafs);
    assert!(res.is_ok())
}

#[test]
fn merkle_empty_leafs_fail() {
    let leafs = Vec::new();
    let res = merkle_root(&leafs);
    assert!(res.is_err())
}

#[test]
fn merkle_faulty_leafs_fail() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE+1).unwrap();
        leafs.push(leaf);
    }
    let res = merkle_root(&leafs);
    assert!(res.is_err())
}

#[test]
fn merkle_is_hash_succ() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE).unwrap();
        leafs.push(leaf);
    }
    let mr = merkle_root(&leafs).unwrap();
    let res = check_hash_size(&mr);
    assert!(res.is_ok())
}

#[test]
fn merkle_verify_succ() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE).unwrap();
        leafs.push(leaf);
    }
    let mr = merkle_root(&leafs).unwrap();
    let res = verify_merkle_root(&leafs, &mr).unwrap();
    assert!(res)
}

#[test]
fn merkle_verify_wrong_root_fail() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE).unwrap();
        leafs.push(leaf);
    }
    let mut mr = merkle_root(&leafs).unwrap();
    mr[0] = (mr[0] % 2) + 1;
    let res = verify_merkle_root(&leafs, &mr).unwrap();
    assert!(!res)
}

#[test]
fn merkle_faulty_root_fail() {
    let len = 10;
    let mut leafs = Vec::new();
    for _ in 0..len {
        let leaf = randombytes(HASH_SIZE).unwrap();
        leafs.push(leaf);
    }
    let mr = randombytes(HASH_SIZE + 1).unwrap();
    let res = verify_merkle_root(&leafs, &mr);
    assert!(res.is_err())
}
