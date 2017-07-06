use libyobicash::models::content::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

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
