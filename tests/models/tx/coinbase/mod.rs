use chrono::Duration;
use libyobicash::models::tx::coinbase::*;
use libyobicash::models::wallet::Wallet;
use libyobicash::models::address::hash_to_address;
use libyobicash::models::height::*;
use libyobicash::models::amount::Amount;
use libyobicash::models::signers::Signers;
use libyobicash::models::output::Output;
use libyobicash::models::outpoint::*;
use libyobicash::crypto::sign::PUBLIC_KEY_SIZE;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::hash::random_u32_from_seed;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_tx_succ() {
    let res = CoinbaseTx::new();
    assert!(res.is_ok())
}

#[test]
fn set_time_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let mut time = tx.get_time();
    let d = Duration::hours(1);
    time = time.checked_sub_signed(d).unwrap();
    let res = tx.set_time(&time);
    assert!(res.is_ok())
}

#[test]
fn set_time_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let mut time = tx.get_time();
    let d = Duration::hours(1);
    time = time.checked_add_signed(d).unwrap();
    let res = tx.set_time(&time);
    assert!(res.is_err())
}

#[test]
fn set_version_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let mut version = tx.get_version();
    if version.major > 0 {
        version.major = version.major -1;
    } else if version.minor > 0 {
        version.minor = version.minor -1;
    } else if version.patch > 0 {
        version.patch = version.patch -1;
    } else {
        panic!("Invalid default version")
    }
    let res = tx.set_version(&version);
    assert!(res.is_ok())
}

#[test]
fn set_version_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let mut version = tx.get_version();
    version.major = version.major +1;
    let res = tx.set_version(&version);
    assert!(res.is_err())
}

#[test]
fn set_height_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let res = tx.set_height(MIN_REGULAR_HEIGHT);
    assert!(res.is_ok())
}

#[test]
fn set_height_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let res = tx.set_height(COINBASE_HEIGHT);
    assert!(res.is_err())
}

#[test]
fn set_signers_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let pk1 = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let res = tx.set_signers(&signers);
    assert!(res.is_ok())
}

#[test]
fn set_signers_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let pk1 = randombytes(PUBLIC_KEY_SIZE).unwrap();
    let weight1 = 10;
    let mut pk2 = pk1.to_owned();
    pk2[0] = pk2[0] % 2 + 1;
    let weight2 = 200;
    let threshold = weight2;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&pk1, weight1).unwrap()
        .add_signer(&pk2, weight2).unwrap()
        .set_threshold(threshold).unwrap();
    let res = tx.set_signers(&signers);
    assert!(res.is_err())
}

#[test]
fn add_output_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    let res = tx.add_output(&output);
    assert!(res.is_ok())
}

#[test]
fn get_output_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    let tx = tx.add_output(&output).unwrap();
    let res = tx.get_output(0);
    assert!(res.is_ok())
}

#[test]
fn get_output_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    let tx = tx.add_output(&output).unwrap();
    let res = tx.get_output(1);
    assert!(res.is_err())
}

#[test]
fn check_balance_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let len = 10;
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx.add_output(&output).unwrap();
    let res = tx.check_balance();
    assert!(res.is_ok())
}

#[test]
fn check_balance_fail() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let len = 10;
    let amount = tx.get_amount() + Amount::new(1);
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx.add_output(&output).unwrap();
    let res = tx.check_balance();
    assert!(res.is_err())
}

#[test]
fn sign_succ() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx = tx
        .set_signers(&signers).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet3).unwrap();
    let res = tx.check_signatures();
    assert!(res.is_ok())
}

#[test]
fn sign_fail() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx = tx
        .set_signers(&signers).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet2).unwrap();
    let res = tx.check_signatures();
    assert!(res.is_err())
}

#[test]
fn finalize_succ() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx = tx
        .set_signers(&signers).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet3).unwrap();
    let res = tx.finalize();
    assert!(res.is_ok())
}

#[test]
fn check_succ() {
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut signers = Signers::new().unwrap();
    signers = signers
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    signers.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    tx = tx
        .set_signers(&signers).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet1).unwrap()
        .sign(&wallet3).unwrap()
        .finalize().unwrap();
    let res = tx.check();
    assert!(res.is_ok())
}

#[test]
fn unique_txs_succ() {
    let len = 10;
    let mut txs: Vec<CoinbaseTx> = Vec::new();
    for i in 0..len {
        let mut tx = CoinbaseTx::new().unwrap();
        let time = tx.get_time() - Duration::hours(i);
        tx.set_time(&time).unwrap();
        txs.push(tx);
    }
    let res = check_unique_cointxs(&txs);
    assert!(res.is_ok())
}

#[test]
fn unique_txs_fail() {
    let tx = CoinbaseTx::new().unwrap();
    let len = 10;
    let txs: Vec<CoinbaseTx> = repeat(tx).take(len).collect();
    let res = check_unique_cointxs(&txs);
    assert!(res.is_err())
}

#[test]
fn get_outpoint_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap()
        .set_signers(&creators).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let outpoint = tx.get_outpoint(0).unwrap();
    let otp_tx_id = outpoint.get_tx_id();
    let otp_height = outpoint.get_height();
    let otp_idx = outpoint.get_idx();
    let otp_output = outpoint.get_output();
    assert_eq!(otp_tx_id, tx.get_id());
    assert_eq!(otp_height, tx.get_height());
    assert_eq!(otp_idx, 0);
    assert_eq!(otp_output, output)
}

#[test]
fn get_outpoints_succ() {
    let mut tx = CoinbaseTx::new().unwrap();
    let wallet = Wallet::new().unwrap();
    let creators = Signers::new().unwrap()
        .add_signer(&wallet.public_key, 1).unwrap()
        .finalize().unwrap();
    let amount = tx.get_amount();
    let seed = randombytes(HASH_SIZE).unwrap();
    let to = hash_to_address(&seed).unwrap();
    let output = Output::no_content(&amount, &to).unwrap();
    output.check().unwrap();
    let mut tx = CoinbaseTx::new().unwrap()
        .set_signers(&creators).unwrap()
        .set_amount().unwrap()
        .add_output(&output).unwrap()
        .set_coins().unwrap()
        .sign(&wallet).unwrap()
        .finalize().unwrap();
    let outpoints = tx.get_outpoints().unwrap();
    let len = outpoints.len() as u32;
    assert_eq!(len, tx.get_outputs_len());
    let outpoint = tx.get_outpoint(len-1).unwrap();
    assert_eq!(outpoints[(len-1) as usize], outpoint)
}
