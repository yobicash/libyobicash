use libyobicash::models::output::*;
use libyobicash::models::content::Content;
use libyobicash::models::amount::Amount;
use libyobicash::models::address::ADDRESS_SIZE;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::wallet::Wallet;
use libyobicash::models::signers::Signers;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_output_succ() {
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
    let res = Output::new(&Amount::new(amount as u32), &to, &content);
    assert!(res.is_ok())
}

#[test]
fn new_no_content_output_succ() {
    let wallet = Wallet::new().unwrap();
    let signers = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let to = signers.get_address();
    let amount = 10;
    let res = Output::no_content(&Amount::new(amount as u32), &to);
    assert!(res.is_ok())
}

#[test]
fn new_no_content_output_fail() {
    let to = randombytes(ADDRESS_SIZE+1).unwrap();
    let amount = 10;
    let res = Output::no_content(&Amount::new(amount as u32), &to);
    assert!(res.is_err())
}

#[test]
fn new_output_fail() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount-1).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
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
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let h = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&h).unwrap();
    let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
    let res = output.check();
    assert!(res.is_ok())
}

#[test]
fn unique_outputs_succ() {
    let wallet = Wallet::new().unwrap();
    let amount = 10;
    let data = randombytes(amount).unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let content = Content::new(&creators, &data).unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let len = 10;
    let mut outputs: Vec<Output> = Vec::new();
    for i in 0..len {
        let h = nonce_from_u32(i).unwrap();
        let to = hash_to_address(&h).unwrap();
        let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
        outputs.push(output);
    }
    let res = check_unique_outputs(&outputs);
    assert!(res.is_ok())
}

#[test]
fn unique_outputs_fail() {
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
    let output = Output::new(&Amount::new(amount as u32), &to, &content).unwrap();
    let len = 10;
    let outputs: Vec<Output> = repeat(output).take(len).collect();
    let res = check_unique_outputs(&outputs);
    assert!(res.is_err())
}
