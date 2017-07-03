use chrono::Duration;
use libyobicash::models::tx::*;
use libyobicash::models::signers::YSigners;
use libyobicash::models::input::YInput;
use libyobicash::models::output::YOutput;
use libyobicash::mining::por::*;
use libyobicash::amount::YAmount;
use libyobicash::crypto::sign::PUBLICKEY_SIZE;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_tx_succ() {
    let res = YTx::new();
    assert!(res.is_ok())
}

#[test]
fn new_tx_check_succ() {
   let tx = YTx::new().unwrap();
   let res = tx.check_pre_id();
   assert!(res.is_ok())
}

#[test]
fn check_time_succ() {
   let mut tx = YTx::new().unwrap();
   let d = Duration::hours(1);
   tx.time = tx.time.checked_sub_signed(d).unwrap();
   let res = tx.check_time();
   assert!(res.is_ok())
}

#[test]
fn check_time_fail() {
   let mut tx = YTx::new().unwrap();
   let d = Duration::hours(1);
   tx.time = tx.time.checked_add_signed(d).unwrap();
   let res = tx.check_time();
   assert!(res.is_err())
}

#[test]
fn check_version_succ() {
    let mut tx = YTx::new().unwrap();
    if tx.version.major > 0 {
        tx.version.major = tx.version.major -1;
    } else if tx.version.minor > 0 {
        tx.version.minor = tx.version.minor -1;
    } else if tx.version.patch > 0 {
        tx.version.patch = tx.version.patch -1;
    } else {
        panic!("Invalid default version")
    }
    let res = tx.check_version();
    assert!(res.is_ok())
}

#[test]
fn check_version_fail() {
    let mut tx = YTx::new().unwrap();
    tx.version.major = tx.version.major +1;
    let res = tx.check_version();
    assert!(res.is_err())
}

#[test]
fn check_signers_succ() {
    let mut tx = YTx::new().unwrap();
    let pk1 = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = YSigners::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap()
        .set_address().unwrap();
    signers.check().unwrap();
    tx.signers = signers;
    let res = tx.check_signers();
    assert!(res.is_ok())
}

#[test]
fn check_signers_fail() {
    let mut tx = YTx::new().unwrap();
    let pk1 = randombytes(PUBLICKEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = YSigners::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap();
    tx.signers = signers;
    let res = tx.check_signers();
    assert!(res.is_err())
}

#[test]
fn check_inputs_succ() {
    let mut tx = YTx::new().unwrap();
    let len = 10;
    let max_idx = 100000;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE).unwrap();
        let idx = read_u32_from_seed(&tx_id, max_idx).unwrap();
        let input = YInput::new(&tx_id, idx).unwrap();
        input.check().unwrap();
        let res = tx.add_input(&input);
        assert!(res.is_ok());
    }
    let res = tx.check_inputs();
    assert!(res.is_ok())
}
