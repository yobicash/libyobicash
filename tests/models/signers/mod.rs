use libyobicash::models::signers::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::sign::PUBLIC_KEY_SIZE;
use libyobicash::crypto::sign::sign;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_signers_succ() {
    let res = Signers::new();
    assert!(res.is_ok())
}

#[test]
fn new_signers_check_succ() {
    let signers = Signers::new().unwrap();
    let res = signers.check();
    assert!(res.is_ok())
}

#[test]
fn signers_add_signer_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 0;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap()
        .finalize().unwrap();
    let res = signers.check();
    assert!(res.is_ok())
}

#[test]
fn signers_add_signer_fail() {
    let pk = randombytes(PUBLIC_KEY_SIZE + 1).unwrap();
    let weight = 0;
    let mut signers = Signers::new().unwrap();
    let res = signers.add_signer(&pk, weight);
    assert!(res.is_err())
}

#[test]
fn signers_lookup_signer_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 0;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    let res = signers.lookup_signer(&pk).unwrap();
    assert!(res)
}

#[test]
fn signers_lookup_signer_fail() {
    let mut pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 0;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLIC_KEY_SIZE + 1).unwrap();
    let res = signers.lookup_signer(&pk);
    assert!(res.is_err())
}

#[test]
fn signers_find_signer_idx_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 0;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    let res = signers.find_signer_idx(&pk).unwrap();
    assert_eq!(res, 0)
}

#[test]
fn signers_find_signer_idx_fail() {
    let mut pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 0;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLIC_KEY_SIZE + 1).unwrap();
    let res = signers.find_signer_idx(&pk);
    assert!(res.is_err())
}

#[test]
fn signers_find_signer_weight_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    let res = signers.find_signer_weight(&pk).unwrap().unwrap();
    assert_eq!(res, 10)
}

#[test]
fn signers_find_signer_weight_fail() {
    let mut pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLIC_KEY_SIZE + 1).unwrap();
    let res = signers.find_signer_weight(&pk);
    assert!(res.is_err())
}

#[test]
fn set_threshold_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let res = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap()
        .set_threshold(weight-1);
    assert!(res.is_ok())
}

#[test]
fn set_threshold_fail() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let res = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap()
        .set_threshold(weight+1);
    assert!(res.is_err())
}

#[test]
fn finalize_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let res = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize();
    assert!(res.is_ok())
}

#[test]
fn check_succ() {
    let pk = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight = 10;
    let res = Signers::new().unwrap()
        .add_signer(&pk, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize().unwrap()
        .check();
    assert!(res.is_ok())
}

#[test]
fn verify_signatures_succ() {
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&wallet.public_key, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize().unwrap();
    let msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    let res = signers.verify_signatures(&msg, &sigs).unwrap();
    assert!(res)
}

#[test]
fn verify_signatures_fail() {
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&wallet.public_key, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize().unwrap();
    let mut msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    msg[0] = msg[0] % 2 + 1;
    let res = signers.verify_signatures(&msg, &sigs).unwrap();
    assert!(!res)
}

#[test]
fn check_signatures_succ() {
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&wallet.public_key, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize().unwrap();
    let msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    let res = signers.check_signatures(&msg, &sigs);
    assert!(res.is_ok())
}

#[test]
fn check_signatures_fail() {
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    let signers = Signers::new().unwrap()
        .add_signer(&wallet.public_key, weight).unwrap()
        .set_threshold(weight-1).unwrap()
        .finalize().unwrap();
    let mut msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    msg[0] = msg[0] % 2 + 1;
    let res = signers.check_signatures(&msg, &sigs);
    assert!(res.is_err())
}

#[test]
fn unique_signers_succ() {
    let len = 10;
    let mut signerses: Vec<Signers> = Vec::new();
    for i in 0..len {
        let seed = nonce_from_u32(i).unwrap();
        let wallet = Wallet::from_seed(&seed).unwrap();
        let weight = 10;
        let signers = Signers::new().unwrap()
            .add_signer(&wallet.public_key, weight).unwrap()
            .set_threshold(weight-1).unwrap()
            .finalize().unwrap();
        signerses.push(signers);
    }
    let res = check_unique_signerses(&signerses);
    assert!(res.is_ok())
}

#[test]
fn unique_signers_fail() {
    let signers = Signers::new().unwrap();
    let len = 10;
    let signerses: Vec<Signers> = repeat(signers).take(len).collect();
    let res = check_unique_signerses(&signerses);
    assert!(res.is_err())
}
