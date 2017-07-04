use chrono::Duration;
use libyobicash::models::block::*;
use libyobicash::models::tx::Tx;
use libyobicash::models::amount::Amount;
use libyobicash::mining::targetting::*;
use libyobicash::mining::por::*;
use libyobicash::mining::pow::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_block_succ() {}

#[test]
fn set_time_succ() {
    let mut block = Block::new().unwrap();
    let mut time = block.get_time();
    let d = Duration::hours(1);
    time = time.checked_sub_signed(d).unwrap();
    let res = block.set_time(&time);
    assert!(res.is_ok())
}

#[test]
fn set_time_fail() {
    let mut block = Block::new().unwrap();
    let mut time = block.get_time();
    let d = Duration::hours(1);
    time = time.checked_add_signed(d).unwrap();
    let res = block.set_time(&time);
    assert!(res.is_err())
}

#[test]
fn set_version_succ() {
    let mut block = Block::new().unwrap();
    let mut version = block.get_version();
    if version.major > 0 {
        version.major = version.major -1;
    } else if version.minor > 0 {
        version.minor = version.minor -1;
    } else if version.patch > 0 {
        version.patch = version.patch -1;
    } else {
        panic!("Invalid default version")
    }
    let res = block.set_version(&version);
    assert!(res.is_ok())
}

#[test]
fn set_version_fail() {
    let mut block = Block::new().unwrap();
    let mut version = block.get_version();
    version.major = version.major +1;
    let res = block.set_version(&version);
    assert!(res.is_err())
}

#[test]
fn from_prev_succ() {}

#[test]
fn from_prev_fail() {}

#[test]
fn check_prev_succ() {}

#[test]
fn check_prev_fail() {}

#[test]
fn set_s_cost_succ() {
    let mut block = Block::new().unwrap();
    let s_cost = MIN_S_COST + 1;
    let res = block.set_s_cost(s_cost);
    assert!(res.is_ok())
}

#[test]
fn set_s_cost_fail() {
    let mut block = Block::new().unwrap();
    let s_cost = MIN_S_COST - 1;
    let res = block.set_s_cost(s_cost);
    assert!(res.is_err())
}

#[test]
fn set_t_cost_succ() {
    let mut block = Block::new().unwrap();
    let t_cost = MIN_T_COST + 1;
    let res = block.set_t_cost(t_cost);
    assert!(res.is_ok())
}

#[test]
fn set_t_cost_fail() {
    let mut block = Block::new().unwrap();
    let t_cost = MIN_T_COST - 1;
    let res = block.set_t_cost(t_cost);
    assert!(res.is_err())
}

#[test]
fn set_delta_succ() {
    let mut block = Block::new().unwrap();
    let delta = MIN_DELTA + 1;
    let res = block.set_delta(delta);
    assert!(res.is_ok())
}

#[test]
fn set_delta_fail() {
    let mut block = Block::new().unwrap();
    let delta = MIN_DELTA - 1;
    let res = block.set_delta(delta);
    assert!(res.is_err())
}

#[test]
fn set_coinbase_succ() {}

#[test]
fn set_coinbase_fail() {}

#[test]
fn add_tx_id_succ() {
    let mut block = Block::new().unwrap();
    let len = 10;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE).unwrap();
        let res = block.add_tx_id(&tx_id);
        assert!(res.is_ok());
    }
    assert_eq!(len, block.get_tx_ids_len())
}

#[test]
fn add_tx_id_fail() {
    let mut block = Block::new().unwrap();
    let len = 10;
    for _ in 0..len {
        let tx_id = randombytes(HASH_SIZE+1).unwrap();
        let res = block.add_tx_id(&tx_id);
        assert!(res.is_err());
    }
    assert_eq!(0, block.get_tx_ids_len())
}

#[test]
fn set_segments_root_succ() {}

#[test]
fn set_segments_root_fail() {}

#[test]
fn check_por_succ() {}

#[test]
fn check_por_fail() {}

#[test]
fn mine_succ() {}

#[test]
fn mine_fail() {}

#[test]
fn check_pow_succ() {}
