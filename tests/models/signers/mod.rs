use libyobicash::models::signers::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::sign::PUBLICKEY_SIZE;
use libyobicash::crypto::sign::sign;
use libyobicash::crypto::utils::randombytes;

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
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 0;
    signers.add_signer(&pk, weight).unwrap();
    signers.set_address().unwrap();
    let res = signers.check();
    assert!(res.is_ok())
}

#[test]
fn signers_add_signer_fail() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE + 1).unwrap();
    let weight = 0;
    let res = signers.add_signer(&pk, weight);
    assert!(res.is_err())
}

#[test]
fn signers_lookup_signer_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 0;
    signers.add_signer(&pk, weight).unwrap();
    let res = signers.lookup_signer(&pk).unwrap();
    assert!(res)
}

#[test]
fn signers_lookup_signer_fail() {
    let mut signers = Signers::new().unwrap();
    let mut pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 0;
    signers.add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLICKEY_SIZE + 1).unwrap();
    let res = signers.lookup_signer(&pk);
    assert!(res.is_err())
}

#[test]
fn signers_find_signer_idx_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 0;
    signers.add_signer(&pk, weight).unwrap();
    let res = signers.find_signer_idx(&pk).unwrap();
    assert_eq!(res, 0)
}

#[test]
fn signers_find_signer_idx_fail() {
    let mut signers = Signers::new().unwrap();
    let mut pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 0;
    signers.add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLICKEY_SIZE + 1).unwrap();
    let res = signers.find_signer_idx(&pk);
    assert!(res.is_err())
}

#[test]
fn signers_find_signer_weight_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    let res = signers.find_signer_weight(&pk).unwrap().unwrap();
    assert_eq!(res, 10)
}

#[test]
fn signers_find_signer_weight_fail() {
    let mut signers = Signers::new().unwrap();
    let mut pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    pk = randombytes(PUBLICKEY_SIZE + 1).unwrap();
    let res = signers.find_signer_weight(&pk);
    assert!(res.is_err())
}

#[test]
fn set_threshold_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    let res = signers.set_threshold(weight-1);
    assert!(res.is_ok())
}

#[test]
fn set_threshold_fail() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    let res = signers.set_threshold(weight+1);
    assert!(res.is_err())
}

#[test]
fn set_address_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    let res = signers.set_address();
    assert!(res.is_ok())
}

#[test]
fn check_succ() {
    let mut signers = Signers::new().unwrap();
    let pk = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight = 10;
    signers.add_signer(&pk, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    signers.set_address().unwrap();
    let res = signers.check();
    assert!(res.is_ok())
}

#[test]
fn verify_signatures_succ() {
    let mut signers = Signers::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    signers.add_signer(&wallet.public_key, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    signers.set_address().unwrap();
    let msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    let res = signers.verify_signatures(&msg, &sigs).unwrap();
    assert!(res)
}

#[test]
fn verify_signatures_fail() {
    let mut signers = Signers::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    signers.add_signer(&wallet.public_key, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    signers.set_address().unwrap();
    let mut msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    msg[0] = msg[0] % 2 + 1;
    let res = signers.verify_signatures(&msg, &sigs).unwrap();
    assert!(!res)
}

#[test]
fn check_signatures_succ() {
    let mut signers = Signers::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    signers.add_signer(&wallet.public_key, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    signers.set_address().unwrap();
    let msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    let res = signers.check_signatures(&msg, &sigs);
    assert!(res.is_ok())
}

#[test]
fn check_signatures_fail() {
    let mut signers = Signers::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let weight = 10;
    signers.add_signer(&wallet.public_key, weight).unwrap();
    signers.set_threshold(weight-1).unwrap();
    signers.set_address().unwrap();
    let mut msg = randombytes(HASH_SIZE).unwrap();
    let sig = sign(&msg, &wallet.secret_key).unwrap();
    let sigs = vec![sig];
    msg[0] = msg[0] % 2 + 1;
    let res = signers.check_signatures(&msg, &sigs);
    assert!(res.is_err())
}
