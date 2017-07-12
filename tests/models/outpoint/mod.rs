use libyobicash::models::outpoint::*;
use libyobicash::models::output::Output;
use libyobicash::models::content::Content;
use libyobicash::models::signers::Signers;
use libyobicash::models::height::*;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_outpoint_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(amount as u32, &to, &content).unwrap();
    let tx_id = randombytes(HASH_SIZE).unwrap();
    let height = MIN_REGULAR_HEIGHT;
    let idx = 10;
    let res = OutPoint::new(&tx_id, height, idx, &output);
    assert!(res.is_ok())
}

#[test]
fn new_outpoint_fail() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(amount as u32, &to, &content).unwrap();
    let tx_id = randombytes(HASH_SIZE+1).unwrap();
    let height = MIN_REGULAR_HEIGHT;
    let idx = 10;
    let res = OutPoint::new(&tx_id, height, idx, &output);
    assert!(res.is_err())
}

#[test]
fn unique_outpoints_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(amount as u32, &to, &content).unwrap();
    let len = 10;
    let mut outpoints: Vec<OutPoint> = Vec::new();
    let height = MIN_REGULAR_HEIGHT;
    for i in 0..len {
        let tx_id = nonce_from_u32(i).unwrap();
        let idx = 10;
        let outpoint = OutPoint::new(&tx_id, height, idx, &output).unwrap();
        outpoints.push(outpoint);
    }
    let res = check_unique_outpoints(&outpoints);
    assert!(res.is_ok())
}

#[test]
fn unique_outpoints_fail() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(amount as u32, &to, &content).unwrap();
    let tx_id = randombytes(HASH_SIZE).unwrap();
    let height = MIN_REGULAR_HEIGHT;
    let idx = 10;
    let outpoint = OutPoint::new(&tx_id, height, idx, &output).unwrap();
    let len = 10;
    let outpoints: Vec<OutPoint> = repeat(outpoint).take(len).collect();
    let res = check_unique_outpoints(&outpoints);
    assert!(res.is_err())
}
