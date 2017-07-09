use libyobicash::mining::pow::*;
use libyobicash::mining::targetting::MIN_BITS;
use libyobicash::mining::targetting::MAX_BITS;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::hash::check_hash_size;
use libyobicash::crypto::hash::nonce_from_u32;
use libyobicash::crypto::utils::randombytes;

#[test]
fn check_s_cost_succ() {
    let s_cost = MIN_S_COST*2;
    let res = check_s_cost(s_cost);
    assert!(res.is_ok())
}

#[test]
fn check_s_cost_fail() {
    let s_cost = MIN_S_COST - 1;
    let res = check_s_cost(s_cost);
    assert!(res.is_err())
}

#[test]
fn check_t_cost_succ() {
    let t_cost = MIN_T_COST*2;
    let res = check_t_cost(t_cost);
    assert!(res.is_ok())
}

#[test]
fn check_t_cost_fail() {
    let t_cost = MIN_T_COST - 1;
    let res = check_t_cost(t_cost);
    assert!(res.is_err())
}

#[test]
fn check_delta_succ() {
    let delta = MIN_DELTA*2;
    let res = check_delta(delta);
    assert!(res.is_ok())
}

#[test]
fn check_delta_fail() {
    let delta = MIN_DELTA - 1;
    let res = check_delta(delta);
    assert!(res.is_err())
}

// NB: failure conditions for hash are a little difficult to test...

#[test]
fn balloon_hash_succ() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let n = 10;
    let nonce = nonce_from_u32(n).unwrap();
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let h = balloon_hash(&seed, &nonce, s_cost, t_cost, delta).unwrap();
    let res = check_hash_size(&h);
    assert!(res.is_ok())
}

#[test]
fn balloon_hash_fail() {
    let seed = randombytes(HASH_SIZE).unwrap();
    let nonce = randombytes(HASH_SIZE+1).unwrap();
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let res = balloon_hash(&seed, &nonce, s_cost, t_cost, delta);
    assert!(res.is_err())
}

#[test]
fn balloon_mine_succ() {
    let t_bits = MIN_BITS;
    let seed = vec![49, 176, 0, 43, 26, 145, 13, 169, 24, 216, 205, 138, 26, 113, 192, 173, 231, 126, 71, 89, 109, 249, 220, 192, 128, 207, 205, 78, 132, 83, 21, 44];
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let nonce: u32 = 1;
    let res = balloon_mine(t_bits, &seed, s_cost, t_cost, delta);
    assert_eq!(res.unwrap().unwrap(), nonce)
}

#[test]
fn balloon_mine_fail() {
    let t_bits = MIN_BITS-1;
    let seed = vec![49, 176, 0, 43, 26, 145, 13, 169, 24, 216, 205, 138, 26, 113, 192, 173, 231, 126, 71, 89, 109, 249, 220, 192, 128, 207, 205, 78, 132, 83, 21, 44];
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let res = balloon_mine(t_bits, &seed, s_cost, t_cost, delta);
    assert!(res.is_err())
}

#[test]
fn balloon_verify_succ() {
    let t_bits = MIN_BITS;
    let seed = vec![49, 176, 0, 43, 26, 145, 13, 169, 24, 216, 205, 138, 26, 113, 192, 173, 231, 126, 71, 89, 109, 249, 220, 192, 128, 207, 205, 78, 132, 83, 21, 44];
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let nonce = balloon_mine(t_bits, &seed, s_cost, t_cost, delta).unwrap().unwrap();
    let res = balloon_verify(t_bits, &seed, nonce, s_cost, t_cost, delta).unwrap();
    assert!(res)
}

#[test]
fn balloon_verify_fail() {
    let t_bits = MIN_BITS;
    let seed = vec![49, 176, 0, 43, 26, 145, 13, 169, 24, 216, 205, 138, 26, 113, 192, 173, 231, 126, 71, 89, 109, 249, 220, 192, 128, 207, 205, 78, 132, 83, 21, 44];
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let nonce = balloon_mine(t_bits, &seed, s_cost, t_cost, delta).unwrap().unwrap();
    let res = balloon_verify(t_bits+MAX_BITS, &seed, nonce, s_cost, t_cost, delta);
    assert!(res.is_err())
}

#[test]
fn balloon_memory_succ() {
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA;
    let res = balloon_memory(s_cost, t_cost, delta);
    assert!(res.is_ok())
}

#[test]
fn balloon_memory_fail() {
    let s_cost = MIN_S_COST;
    let t_cost = MIN_T_COST;
    let delta = MIN_DELTA-1;
    let res = balloon_memory(s_cost, t_cost, delta);
    assert!(res.is_err())
}
