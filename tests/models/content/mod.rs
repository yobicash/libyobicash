use libyobicash::models::content::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::mining::pow::balloon_nonce_from_u32;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_content_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let wallet = Wallet::from_seed(&seed).unwrap();
    let size = 10;
    let data = randombytes(size).unwrap();
    let res = Content::new(&wallet, &data);
    assert!(res.is_ok())
}

#[test]
fn check_succ() {
    let wallet = Wallet::new().unwrap();
    let size = 10;
    let data = randombytes(size).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let res = content.check();
    assert!(res.is_ok())
}

#[test]
fn unique_contents_succ() {
    let len = 10;
    let wallet = Wallet::new().unwrap();
    let mut contents: Vec<Content> = Vec::new();
    for i in 0..len {
        let data = balloon_nonce_from_u32(i).unwrap();
        let content = Content::new(&wallet, &data).unwrap();
        contents.push(content);
    }
    let res = check_unique_contents(&contents);
    assert!(res.is_ok())
}

#[test]
fn unique_contents_fail() {
    let len = 10;
    let wallet = Wallet::new().unwrap();
    let data = randombytes(len).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let contents: Vec<Content> = repeat(content).take(len).collect();
    let res = check_unique_contents(&contents);
    assert!(res.is_err())
}
