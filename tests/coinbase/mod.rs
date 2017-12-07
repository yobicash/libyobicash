use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::coinbase::YCoinbase;
use libyobicash::utils::random::YRandom;

#[test]
fn coinbase_new_succ() {
    let res = YCoinbase::new();
    assert!(res.is_ok())
}

#[test]
fn coinbase_new_fail() {
}

#[test]
fn coinbase_verify_succ() {
}

#[test]
fn coinbase_verify_fail() {
}

#[test]
fn coinbase_check_succ() {
}

#[test]
fn coinbase_check_fail() {
}

#[test]
fn coinbase_bytes_succ() {
}

#[test]
fn coinbase_bytes_fail() {
    let mut b = [0u8; 103];
    YRandom::bytes_mut(&mut b);
    let res = YCoinbase::from_bytes(&b[..]);
    assert!(res.is_err())
}
