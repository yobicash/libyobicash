use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::proof::storage::YPoSt;
use libyobicash::coinbase::YCoinbase;
use libyobicash::utils::random::YRandom;

#[test]
fn coinbase_new_succ() {
    let res = YCoinbase::new();
    assert!(res.is_ok())
}

#[test]
fn coinbase_bytes_succ() {
    let mut cb_a = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb_a.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 1;
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    cb_a.set_pow(increment, miner_sk, recipient_pk, fee_pk).unwrap();
    let cb_buf = cb_a.to_bytes().unwrap();
    let cb_b = YCoinbase::from_bytes(cb_buf.as_slice()).unwrap();
    assert_eq!(cb_a, cb_b)
}

#[test]
fn coinbase_bytes_fail() {
    let mut b = [0u8; 99];
    YRandom::bytes_mut(&mut b);
    let res = YCoinbase::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn coinbase_set_post_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    let res = cb.set_post(id_tx, diff, nonce, &chunks);
    assert!(res.is_ok())
}

#[test]
fn coinbase_set_post_fail() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff-1);
    let res = cb.set_post(id_tx, diff, nonce, &chunks);
    assert!(res.is_err())
}

#[test]
fn coinbase_amount_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 63-diff;
    let res = cb.coinbase_amount(increment);
    assert!(res.is_ok())
}

#[test]
fn coinbase_amount_fail() {
    let cb = YCoinbase::new().unwrap();
    let increment = 1;
    let res = cb.coinbase_amount(increment);
    assert!(res.is_err())
}

#[test]
fn coinbase_set_pow_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 1;
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    let res = cb.set_pow(increment, miner_sk, recipient_pk, fee_pk);
    println!("res: {:?}", res);
    assert!(res.is_ok())
}

#[test]
fn coinbase_set_pow_fail() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 1;
    cb.post = Some(YPoSt::default());
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    let res = cb.set_pow(increment, miner_sk, recipient_pk, fee_pk);
    assert!(res.is_err())
}

#[test]
fn coinbase_verify_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 1;
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    cb.set_pow(increment, miner_sk, recipient_pk, fee_pk).unwrap();
    let verified = cb.verify().unwrap();
    assert!(verified)
}

#[test]
fn coinbase_verify_fail() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let verified = cb.verify().unwrap();
    assert!(!verified)
}

#[test]
fn coinbase_pre_pow_check_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let res = cb.pre_pow_check();
    assert!(res.is_ok())
}

#[test]
fn coinbase_pre_pow_check_fail() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    cb.post = Some(YPoSt::default());
    let res = cb.pre_pow_check();
    assert!(res.is_err())
}

#[test]
fn coinbase_check_succ() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let increment = 1;
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    cb.set_pow(increment, miner_sk, recipient_pk, fee_pk).unwrap();
    let res = cb.check();
    assert!(res.is_ok())
}

#[test]
fn coinbase_check_fail() {
    let mut cb = YCoinbase::new().unwrap();
    let id_tx = YDigest64::from_bytes(YRandom::bytes(64).as_slice()).unwrap();
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    cb.set_post(id_tx, diff, nonce, &chunks).unwrap();
    let res = cb.check();
    assert!(res.is_err())
}

#[test]
fn coinbase_genesys_succ() {
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff);
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    let (gen_cb, gen_tx) = YCoinbase::mine_genesys(diff,
                                                    nonce,
                                                    &chunks,
                                                    miner_sk,
                                                    recipient_pk,
                                                    fee_pk).unwrap();
    let mut res = gen_cb.check();
    assert!(res.is_ok());
    res = gen_tx.check();
    assert!(res.is_ok())
}

#[test]
fn coinbase_genesys_fail() {
    let diff = 3;
    let nonce = 0;
    let chunks = YRandom::bytes(diff-1);
    let g = YPoint::default();
    let miner_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let fee_sk = YSecretKey::from_g(g);
    let fee_pk = fee_sk.to_public();
    let res = YCoinbase::mine_genesys(diff,
                                      nonce,
                                      &chunks,
                                      miner_sk,
                                      recipient_pk,
                                      fee_pk);
    assert!(res.is_err())
}
