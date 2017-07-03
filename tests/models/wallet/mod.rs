use libyobicash::models::wallet::YWallet;
use libyobicash::crypto::sign::SEED_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_wallet_succ() {
    let res = YWallet::new();
    assert!(res.is_ok())
}

#[test]
fn new_wallet_from_seed_succ() {
    let seed = randombytes(SEED_SIZE).unwrap();
    let res = YWallet::from_seed(&seed);
    assert!(res.is_ok())
}

#[test]
fn new_wallet_from_seed_fail() {
    let seed = randombytes(SEED_SIZE+1).unwrap();
    let res = YWallet::from_seed(&seed);
    assert!(res.is_err())
}
