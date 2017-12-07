use libyobicash::proof::storage::YPoSt;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::utils::random::YRandom;

#[test]
fn post_new_succ() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff);
    let res = YPoSt::new(id_tx, diff, nonce, &chunks);
    assert!(res.is_ok())
}

#[test]
fn post_new_fail() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff-1);
    let res = YPoSt::new(id_tx, diff, nonce, &chunks);
    assert!(res.is_err())
}

#[test]
fn post_verify_succ() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff);
    let post = YPoSt::new(id_tx, diff, nonce, &chunks).unwrap();
    assert!(post.verify())
}

#[test]
fn post_verify_fail() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff);
    let mut post = YPoSt::new(id_tx, diff, nonce, &chunks).unwrap();
    post.digest = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    assert!(!post.verify())
}

#[test]
fn post_check_succ() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff);
    let post = YPoSt::new(id_tx, diff, nonce, &chunks).unwrap();
    let res = post.check();
    assert!(res.is_ok())
}

#[test]
fn post_check_fail() {
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 10;
    let nonce = 10;
    let chunks = YRandom::bytes(diff);
    let mut post = YPoSt::new(id_tx, diff, nonce, &chunks).unwrap();
    post.digest = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let res = post.check();
    assert!(res.is_err())
}
