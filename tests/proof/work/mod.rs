use libyobicash::proof::work::*;
use libyobicash::crypto::hash::digest::*;
use libyobicash::crypto::hash::sha::YSHA512;
use libyobicash::crypto::hash::balloon::YBalloonParams;
use libyobicash::utils::random::*;

#[test]
fn target_new_succ() {
    let bits = YRandom::u32_range(0, 64);
    let res = YTarget::new(bits);
    assert!(res.is_ok())
}

#[test]
fn target_new_fail() {
    let bits = 64;
    let res = YTarget::new(bits);
    assert!(res.is_err())
}

#[test]
fn target_bits_succ() {
    let bits = YRandom::u32_range(0, 64);
    let target = YTarget::new(bits).unwrap();
    let _bits = target.bits().unwrap();
    assert_eq!(bits, _bits);
}

#[test]
fn pow_new_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let res = YPoW::new(post_digest, post_difficulty, increment);
    assert!(res.is_ok())
}

#[test]
fn pow_new_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 64;
    let res = YPoW::new(post_digest, post_difficulty, increment);
    assert!(res.is_err())
}

#[test]
fn post_pow_params_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let res = pow.post_params();
    assert!(res.is_ok())
}

#[test]
fn post_pow_params_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.post_difficulty = 64;
    let res = pow.post_params();
    assert!(res.is_err())
}

#[test]
fn pow_post_balloon_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let res = pow.post_balloon();
    assert!(res.is_ok())
}

#[test]
fn pow_post_balloon_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.post_difficulty = 64;
    let res = pow.post_balloon();
    assert!(res.is_err())
}

#[test]
fn pow_memory_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 10;
    let increment = 30;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let memory_high = pow.memory().unwrap();
    let mut extra_params = pow.params.unwrap();
    extra_params.s_cost = extra_params.s_cost -1;
    extra_params.t_cost = extra_params.t_cost -1;
    extra_params.delta = extra_params.delta -1;
    pow.params = Some(extra_params);
    let memory_low = pow.memory().unwrap();
    assert!(memory_high > memory_low)
}

#[test]
fn pow_memory_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.post_difficulty = 64;
    let res = pow.memory();
    assert!(res.is_err())
}

#[test]
fn pow_target_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let res = pow.target();
    assert!(res.is_ok())
}

#[test]
fn pow_target_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.post_difficulty = 64;
    let res = pow.target();
    assert!(res.is_err())
}

#[test]
fn pow_target_bits_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let _post_difficulty = pow.target_bits();
    assert_eq!(post_difficulty, _post_difficulty);
}

#[test]
fn pow_params_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let res = pow.params();
    assert!(res.is_ok())
}

#[test]
fn pow_params_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.params = Some(YBalloonParams {
        s_cost: 0,
        t_cost: 0,
        delta: 0,
    });
    let res = pow.params();
    assert!(res.is_err())
}

#[test]
fn pow_mine_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 3;
    let increment = 1;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let seed = YRandom::bytes(32);
    let res = pow.mine(seed.as_slice());
    assert!(res.is_ok())
}

#[test]
fn pow_mine_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = YRandom::u32_range(3, 64);
    let increment = 63-post_difficulty;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    pow.post_difficulty = 64;
    let seed = YRandom::bytes(32);
    let res = pow.mine(seed.as_slice());
    assert!(res.is_err())
}

#[test]
fn pow_verify_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 3;
    let increment = 1;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let seed = YRandom::bytes(32);
    pow.mine(seed.as_slice()).unwrap();
    if pow.digest.is_some() {
        let verified = pow.verify().unwrap();
        assert!(verified);
    }
}

#[test]
fn pow_verify_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 3;
    let increment = 1;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let seed = YRandom::bytes(32);
    pow.mine(seed.as_slice()).unwrap();
    pow.digest = Some(YSHA512::hash(YRandom::bytes(1).as_slice()));
    let verified = pow.verify().unwrap();
    assert!(!verified);
}

#[test]
fn pow_check_succ() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 3;
    let increment = 1;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let seed = YRandom::bytes(32);
    pow.mine(seed.as_slice()).unwrap();
    if pow.digest.is_some() {
        let res = pow.check();
        assert!(res.is_ok());
    }
}

#[test]
fn pow_check_fail() {
    let post_digest_buf = YRandom::bytes(64);
    let post_digest = YDigest64::from_bytes(post_digest_buf.as_slice()).unwrap(); 
    let post_difficulty = 3;
    let increment = 1;
    let mut pow = YPoW::new(post_digest, post_difficulty, increment).unwrap();
    let seed = YRandom::bytes(32);
    pow.mine(seed.as_slice()).unwrap();
    pow.digest = Some(YSHA512::hash(YRandom::bytes(1).as_slice()));
    let res = pow.check();
    assert!(res.is_err());
}
