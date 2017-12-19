use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::amount::YAmount;
use libyobicash::utxo::YUTXO;
use libyobicash::utils::random::YRandom;

#[test]
fn utxo_to_input_succ() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.to_public();
    let amount = YAmount::one();
    let utxo = YUTXO::new(id, idx, height, recipient, amount);
    let u = YScalar::random();
    let c = YScalar::random();
    let res = utxo.to_input(secret.sk, u, c);
    assert!(res.is_ok())
}

#[test]
fn utxo_to_input_fail() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.to_public();
    let amount = YAmount::one();
    let utxo = YUTXO::new(id, idx, height, recipient, amount);
    let false_x = YScalar::random();
    let u = YScalar::random();
    let c = YScalar::random();
    let res = utxo.to_input(false_x, u, c);
    assert!(res.is_err())
}

#[test]
fn utxo_bytes_succ() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.to_public();
    let amount = YAmount::one();
    let utxo_a = YUTXO::new(id, idx, height, recipient, amount);
    let utxo_buf = utxo_a.to_bytes().unwrap();
    let utxo_b = YUTXO::from_bytes(utxo_buf.as_slice()).unwrap();
    assert_eq!(utxo_a.to_bytes().unwrap(), utxo_b.to_bytes().unwrap())
}

#[test]
fn utxo_bytes_fail() {
    let mut b = [0u8; 135];
    YRandom::bytes_mut(&mut b);
    let res = YUTXO::from_bytes(&b[..]);
    assert!(res.is_err())
}
