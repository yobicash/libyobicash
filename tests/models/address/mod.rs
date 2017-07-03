use libyobicash::models::address::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn hash_to_address_succ() {
    let h = randombytes(HASH_SIZE).unwrap();
    let res = hash_to_address(&h);
    assert!(res.is_ok())
}

#[test]
fn hash_to_address_fail() {
    let h = randombytes(HASH_SIZE-1).unwrap();
    let res = hash_to_address(&h);
    assert!(res.is_err())
}

#[test]
fn check_address_size_succ() {
    let h = randombytes(HASH_SIZE).unwrap();
    let addr = hash_to_address(&h).unwrap();
    let res = check_address_size(&addr);
    assert!(res.is_ok())
}

#[test]
fn check_address_size_fail() {
    let addr = randombytes(ADDRESS_SIZE+1).unwrap();
    let res = check_address(&addr);
    assert!(res.is_err())
}

#[test]
fn check_address_succ() {
    let h = randombytes(HASH_SIZE).unwrap();
    let addr = hash_to_address(&h).unwrap();
    let res = check_address(&addr);
    assert!(res.is_ok())
}

#[test]
fn check_address_fail() {
    let mut addr = randombytes(ADDRESS_SIZE).unwrap();
    addr[0] = ADDRESS_PREFIX+1;
    let res = check_address(&addr);
    assert!(res.is_err())
}
