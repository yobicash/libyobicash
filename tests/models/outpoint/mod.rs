use libyobicash::models::outpoint::*;
use libyobicash::models::output::Output;
use libyobicash::models::content::Content;
use libyobicash::models::amount::Amount;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_outpoint_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
    let tx_id = randombytes(HASH_SIZE).unwrap();
    let idx = 10;
    let res = OutPoint::new(&tx_id, idx, &output);
    assert!(res.is_ok())
}

#[test]
fn new_outpoint_fail() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let content = Content::new(&wallet, &data).unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
    let tx_id = randombytes(HASH_SIZE+1).unwrap();
    let idx = 10;
    let res = OutPoint::new(&tx_id, idx, &output);
    assert!(res.is_err())
}
