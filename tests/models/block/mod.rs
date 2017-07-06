use chrono::Duration;
use libyobicash::models::block::*;
use libyobicash::models::tx::Tx;
use libyobicash::models::amount::Amount;
use libyobicash::models::signers::Signers;
use libyobicash::models::wallet::Wallet;
use libyobicash::mining::targetting::*;
use libyobicash::mining::por::*;
use libyobicash::mining::pow::*;
use libyobicash::crypto::hash::HASH_SIZE;
use libyobicash::crypto::utils::randombytes;

#[test]
fn new_block_succ() {
    let res = Block::new();
    assert!(res.is_ok())
}

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
fn set_coinbase_succ() {
    let wallet = Wallet::new().unwrap();
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut to = Signers::new().unwrap();
    to = to
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    to.check().unwrap();
    let mut block = Block::new().unwrap();
    block = block
        .set_s_cost(MIN_S_COST).unwrap()
        .set_t_cost(MIN_T_COST).unwrap()
        .set_delta(MIN_DELTA).unwrap();
    let data = Vec::new(); // TODO: c_amount.to_u32()?;
    let res = block.set_coinbase(&wallet, &to, &data);
    assert!(res.is_ok())
}

#[test]
fn set_coinbase_fail() {
    let wallet = Wallet::new().unwrap();
    let seed1 = randombytes(HASH_SIZE).unwrap();
    let wallet1 = Wallet::from_seed(&seed1).unwrap();
    let weight1 = 10;
    let seed2 = randombytes(HASH_SIZE).unwrap();
    let wallet2 = Wallet::from_seed(&seed2).unwrap();
    let weight2 = 50;
    let seed3 = randombytes(HASH_SIZE).unwrap();
    let wallet3 = Wallet::from_seed(&seed3).unwrap();
    let weight3 = 100;
    let threshold = weight1 + weight3;
    let mut to = Signers::new().unwrap();
    to = to
        .add_signer(&wallet1.public_key, weight1).unwrap()
        .add_signer(&wallet2.public_key, weight2).unwrap()
        .add_signer(&wallet3.public_key, weight3).unwrap()
        .set_threshold(threshold).unwrap()
        .finalize().unwrap();
    to.check().unwrap();
    let mut block = Block::new().unwrap();
    block = block
        .set_s_cost(MIN_S_COST).unwrap()
        .set_t_cost(MIN_T_COST).unwrap()
        .set_delta(MIN_DELTA).unwrap();
    // let c_amount = block.get_coinbase_amount() + Amount::new(1);
    let data = randombytes(2).unwrap(); // TODO: c_amount.to_u32()?;
    let res = block.set_coinbase(&wallet, &to, &data);
    assert!(res.is_err())
}

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
fn set_segments_root_succ() {
    let mut block = Block::new().unwrap();
    let mut segments = Vec::new();
    for _ in 0..block.get_bits() {
        let segment = randombytes(SEGMENT_SIZE).unwrap();
        segments.push(segment);
    }
    let res = block.set_segments_root(&segments);
    assert!(res.is_ok())
}

#[test]
fn set_segments_root_fail() {
    let mut block = Block::new().unwrap();
    let mut segments = Vec::new();
    for _ in 0..block.get_bits() + 1 {
        let segment = randombytes(SEGMENT_SIZE).unwrap();
        segments.push(segment);
    }
    let res = block.set_segments_root(&segments);
    assert!(res.is_err())
}

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

#[test]
fn from_prev_succ() {}

#[test]
fn from_prev_fail() {}

#[test]
fn check_prev_succ() {}

#[test]
fn check_prev_fail() {}
