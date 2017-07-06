use libyobicash::models::wallet::Wallet;
use libyobicash::models::wallet::check_unique_wallets;
use libyobicash::mining::pow::balloon_nonce_from_u32;
use libyobicash::crypto::sign::SEED_SIZE;
use libyobicash::crypto::utils::randombytes;
use std::iter::repeat;

#[test]
fn new_wallet_succ() {
    let res = Wallet::new();
    assert!(res.is_ok())
}

#[test]
fn new_wallet_from_seed_succ() {
    let seed = randombytes(SEED_SIZE).unwrap();
    let res = Wallet::from_seed(&seed);
    assert!(res.is_ok())
}

#[test]
fn new_wallet_from_seed_fail() {
    let seed = randombytes(SEED_SIZE+1).unwrap();
    let res = Wallet::from_seed(&seed);
    assert!(res.is_err())
}

#[test]
fn unique_wallets_succ() {
    let len = 10;
    let mut wallets: Vec<Wallet> = Vec::new();
    for i in 0..len {
        let seed = balloon_nonce_from_u32(i).unwrap();
        let wallet = Wallet::from_seed(&seed).unwrap();
        wallets.push(wallet);
    }
    let res = check_unique_wallets(&wallets);
    assert!(res.is_ok())
}

#[test]
fn unique_wallets_fail() {
    let len = 10;
    let wallet = Wallet::new().unwrap();
    let wallets: Vec<Wallet> = repeat(wallet).take(len).collect();
    let res = check_unique_wallets(&wallets);
    assert!(res.is_err())
}
