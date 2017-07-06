use libyobicash::models::output::*;
use libyobicash::models::content::Content;
use libyobicash::models::amount::Amount;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_output_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let res = Output::new(&Amount::new(amount as u32), &to, &content);
    assert!(res.is_ok())
}

#[test]
fn new_output_fail() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount-1).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let res = Output::new(&Amount::new(amount as u32), &to, &content);
    assert!(res.is_err())
}

#[test]
fn check_output_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
    let res = output.check();
    assert!(res.is_ok())
}
