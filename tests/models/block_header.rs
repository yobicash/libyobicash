// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `block_header` module tests.

use libyobicash::constants::TESTWITNESS;
use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Digest, Scalar, ZKPWitness};
use libyobicash::crypto::HexSerialize as CryptoHexSerialize;
use libyobicash::models::output::Output;
use libyobicash::models::coin::Coin;
use libyobicash::models::transaction::Transaction;
use libyobicash::models::block::Block;
use libyobicash::models::block_header::BlockHeader;

#[test]
fn block_header_new_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let res = BlockHeader::new(&block, &prev_block_header, witness);
    assert!(res.is_ok())
}

#[test]
fn block_header_new_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let witness = ZKPWitness::from_hex(TESTWITNESS).unwrap();

    let res = BlockHeader::new(&block, &prev_block_header, witness);
    assert!(res.is_err())
}

#[test]
fn block_header_new_regtest_genesis_succ() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let block_header = BlockHeader::new_regtest_genesis(regtest_witness).unwrap();
    
    let res = block_header.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_header_new_regtest_genesis_fail() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let mut block_header = BlockHeader::new_regtest_genesis(regtest_witness).unwrap();
    block_header.transactions_length += 1;

    let res = block_header.is_genesis();
    assert!(res.is_err())
}

#[test]
fn block_header_new_testnet_genesis_succ() {
    let block_header = BlockHeader::new_testnet_genesis().unwrap();
    
    let res = block_header.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_header_new_testnet_genesis_fail() {
    let mut block_header = BlockHeader::new_testnet_genesis().unwrap();
    block_header.transactions_length += 1;
    
    let res = block_header.is_genesis();
    assert!(res.is_err())
}

#[test]
fn block_header_new_mainnet_genesis_succ() {
    let block_header = BlockHeader::new_mainnet_genesis().unwrap();
    
    let res = block_header.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_header_new_mainnet_genesis_fail() {
    let mut block_header = BlockHeader::new_mainnet_genesis().unwrap();
    block_header.transactions_length += 1;
    
    let res = block_header.is_genesis();
    assert!(res.is_err())
}

#[test]
fn block_header_validate_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header = BlockHeader::new(&block, &prev_block_header, witness).unwrap();

    let res = block_header.validate();
    assert!(res.is_ok())
}

#[test]
fn block_header_validate_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let mut block_header = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    block_header.prev_id = Digest::default();

    let res = block_header.validate();
    assert!(res.is_err())
}

#[test]
fn block_header_verify_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header = BlockHeader::new(&block, &prev_block_header, witness).unwrap();

    let verified = block_header.verify(&block, Some(&prev_block_header)).unwrap();
    assert!(verified)
}

#[test]
fn block_header_verify_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs_a = vec![tx_a.clone(), tx_b.clone()];

    let block_a = Block::new(network_type, &txs_a).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header = BlockHeader::new(&block_a, &prev_block_header, witness).unwrap();

    let txs_b = vec![tx_b, tx_a];

    let block_b = Block::new(network_type, &txs_b).unwrap();

    let verified = block_header.verify(&block_b, Some(&prev_block_header)).unwrap();
    assert!(!verified)
}

#[test]
fn block_header_to_json_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let block_header_str = block_header_a.to_json().unwrap();
    let block_header_b = BlockHeader::from_json(&block_header_str).unwrap();
    
    assert_eq!(block_header_a, block_header_b)
}

#[test]
fn block_header_to_json_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let mut block_header_str = block_header_a.to_json().unwrap();
    block_header_str.pop();

    let res = BlockHeader::from_json(&block_header_str);
    assert!(res.is_err())
}

#[test]
fn block_header_to_bytes_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let block_header_buf = block_header_a.to_bytes().unwrap();
    let block_header_b = BlockHeader::from_bytes(&block_header_buf).unwrap();
    
    assert_eq!(block_header_a, block_header_b)
}

#[test]
fn block_header_to_bytes_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let mut block_header_buf = block_header_a.to_bytes().unwrap();
    block_header_buf[0] ^= block_header_buf[0];
    
    let res = BlockHeader::from_bytes(&block_header_buf);
    assert!(res.is_err())
}

#[test]
fn block_header_to_hex_succ() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let block_header_str = block_header_a.to_hex().unwrap();
    let block_header_b = BlockHeader::from_hex(&block_header_str).unwrap();
    
    assert_eq!(block_header_a, block_header_b)
}

#[test]
fn block_header_to_hex_fail() {
    let prev_block_header = BlockHeader::new_testnet_genesis().unwrap();

    let network_type = NetworkType::TestNet;

    let in_amount_a = Amount::from(10.0);
    let in_instance_a = Scalar::random();
    let in_witness_a = ZKPWitness::new(in_instance_a).unwrap();
    let in_output_a = Output::new(&in_amount_a, in_witness_a).unwrap();
    let in_coin_a = Coin::new(&in_output_a, in_instance_a).unwrap();

    let out_amount_a = Amount::from(8.0);
    let out_instance_a = Scalar::random();
    let out_witness_a = ZKPWitness::new(out_instance_a).unwrap();
    let out_output_a = Output::new(&out_amount_a, out_witness_a).unwrap();
    
    let fee_a = Amount::from(2.0);
    
    let coins_a = vec![in_coin_a];
    let outputs_a = vec![out_output_a];
    let ds_a = vec![];

    let tx_a = Transaction::new(network_type, &coins_a, &outputs_a, &ds_a, &fee_a).unwrap();

    let in_amount_b = Amount::from(10.0);
    let in_instance_b = Scalar::random();
    let in_witness_b = ZKPWitness::new(in_instance_b).unwrap();
    let in_output_b = Output::new(&in_amount_b, in_witness_b).unwrap();
    let in_coin_b = Coin::new(&in_output_b, in_instance_b).unwrap();

    let out_amount_b = Amount::from(8.0);
    let out_instance_b = Scalar::random();
    let out_witness_b = ZKPWitness::new(out_instance_b).unwrap();
    let out_output_b = Output::new(&out_amount_b, out_witness_b).unwrap();
    
    let fee_b = Amount::from(2.0);
    
    let coins_b = vec![in_coin_b];
    let outputs_b = vec![out_output_b];
    let ds_b = vec![];

    let tx_b = Transaction::new(network_type, &coins_b, &outputs_b, &ds_b, &fee_b).unwrap();

    let txs = vec![tx_a, tx_b];

    let block = Block::new(network_type, &txs).unwrap();
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();

    let block_header_a = BlockHeader::new(&block, &prev_block_header, witness).unwrap();
    let mut block_header_str = block_header_a.to_hex().unwrap();
    block_header_str.pop();

    let res = BlockHeader::from_hex(&block_header_str);
    assert!(res.is_err())
}
