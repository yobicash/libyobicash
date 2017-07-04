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
fn set_time_succ() {}

#[test]
fn set_time_fail() {}

#[test]
fn set_version_succ() {}

#[test]
fn set_version_fail() {}

#[test]
fn from_prev_succ() {}

#[test]
fn from_prev_fail() {}

#[test]
fn check_prev_succ() {}

#[test]
fn check_prev_fail() {}

#[test]
fn set_s_cost_succ() {}

#[test]
fn set_s_cost_fail() {}

#[test]
fn set_t_cost_succ() {}

#[test]
fn set_t_cost_fail() {}

#[test]
fn set_delta_succ() {}

#[test]
fn set_delta_fail() {}

#[test]
fn set_coinbase_succ() {}

#[test]
fn set_coinbase_fail() {}

#[test]
fn add_tx_id_succ() {}

#[test]
fn add_tx_id_fail() {}

#[test]
fn set_segments_root_succ() {}

#[test]
fn set_segments_root_fail() {}

#[test]
fn check_segments_root_succ() {}

#[test]
fn check_segments_root_fail() {}

#[test]
fn mine_succ() {}

#[test]
fn mine_fail() {}

#[test]
fn check_pow_succ() {}
