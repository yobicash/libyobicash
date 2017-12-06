use libyobicash::crypto::hash::balloon::*;
use libyobicash::crypto::hash::digest::*;
use libyobicash::utils::random::Random;

#[test]
fn balloon_params_new_succ() {
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let res = YBalloonParams::new(s_cost, t_cost, delta);
    assert!(res.is_ok())
}

#[test]
fn balloon_params_new_fail() {
    let s_cost = Random::u32_range(1, 10);
    let t_cost = 0;
    let delta = 3;
    let res = YBalloonParams::new(s_cost, t_cost, delta);
    assert!(res.is_err())
}

#[test]
fn balloon_params_check_succ() {
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let res = params.check();
    assert!(res.is_ok())
}

#[test]
fn balloon_params_check_fail() {
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let mut params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    params.s_cost = 0;
    let res = params.check();
    assert!(res.is_err())
}

#[test]
fn balloon256_new_succ() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let res = YBalloon256::new(salt, params);
    assert!(res.is_ok())
}

#[test]
fn balloon256_new_fail() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let mut params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    params.t_cost = 0;
    let res = YBalloon256::new(salt, params);
    assert!(res.is_err())
}

#[test]
fn balloon256_check_succ() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let balloon = YBalloon256::new(salt, params).unwrap();
    let res = balloon.check();
    assert!(res.is_ok())
}

#[test]
fn balloon256_check_fail() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let mut balloon = YBalloon256::new(salt, params).unwrap();
    balloon.params.delta = 2;
    let res = balloon.check();
    assert!(res.is_err())
}

#[test]
fn balloon256_hash_succ() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let balloon = YBalloon256::new(salt, params).unwrap();
    let msg = Random::bytes(100);
    let res = balloon.hash(msg.as_slice());
    assert!(res.is_ok())
}

#[test]
fn balloon256_hash_fail() {
    let salt_buf = Random::bytes(32);
    let salt = YDigest32::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let mut balloon = YBalloon256::new(salt, params).unwrap();
    balloon.params.delta = 2;
    let msg = Random::bytes(100);
    let res = balloon.hash(msg.as_slice());
    assert!(res.is_err())
}

#[test]
fn balloon512_new_succ() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let res = YBalloon512::new(salt, params);
    assert!(res.is_ok())
}

#[test]
fn balloon512_new_fail() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let mut params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    params.t_cost = 0;
    let res = YBalloon512::new(salt, params);
    assert!(res.is_err())
}

#[test]
fn balloon512_check_succ() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let balloon = YBalloon512::new(salt, params).unwrap();
    let res = balloon.check();
    assert!(res.is_ok())
}

#[test]
fn balloon512_check_fail() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let mut balloon = YBalloon512::new(salt, params).unwrap();
    balloon.params.delta = 2;
    let res = balloon.check();
    assert!(res.is_err())
}

#[test]
fn balloon512_hash_succ() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let balloon = YBalloon512::new(salt, params).unwrap();
    let msg = Random::bytes(100);
    let res = balloon.hash(msg.as_slice());
    assert!(res.is_ok())
}

#[test]
fn balloon512_hash_fail() {
    let salt_buf = Random::bytes(64);
    let salt = YDigest64::from_bytes(salt_buf.as_slice()).unwrap();
    let s_cost = Random::u32_range(1, 10);
    let t_cost = Random::u32_range(1, 10);
    let delta = 3;
    let params = YBalloonParams::new(s_cost, t_cost, delta).unwrap();
    let mut balloon = YBalloon512::new(salt, params).unwrap();
    balloon.params.delta = 2;
    let msg = Random::bytes(100);
    let res = balloon.hash(msg.as_slice());
    assert!(res.is_err())
}
