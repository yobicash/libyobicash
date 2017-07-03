use libyobicash::models::wallet::Wallet;
use libyobicash::crypto::sign::SEED_SIZE;
use libyobicash::crypto::utils::randombytes;

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
