// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `block` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Random, Digest, Scalar, ZKPWitness, SecretKey};
use libyobicash::models::output::Output;
use libyobicash::models::data::Data;
use libyobicash::models::coin::Coin;
use libyobicash::models::transaction::Transaction;
use libyobicash::models::block::Block;

#[test]
fn block_new_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let res = Block::new(network_type, &txs);
    assert!(res.is_ok())
}

#[test]
fn block_new_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::MainNet;

    let mut tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    tx.network_type = NetworkType::TestNet;
    let txs = vec![tx];

    let res = Block::new(network_type, &txs);
    assert!(res.is_err())
}

#[test]
fn block_new_regtest_genesis_succ() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let block = Block::new_regtest_genesis(regtest_witness).unwrap();
    let res = block.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_new_regtest_genesis_fail() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let mut block = Block::new_regtest_genesis(regtest_witness).unwrap();
    block.transactions_length += 1;

    let is_genesis = block.is_genesis().unwrap();
    assert!(!is_genesis)
}

#[test]
fn block_new_testnet_genesis_succ() {
    let block = Transaction::new_testnet_genesis().unwrap();

    let res = block.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_new_testnet_genesis_fail() {
    let mut block = Block::new_testnet_genesis().unwrap();
    block.network_type = NetworkType::MainNet;

    let res = block.is_genesis();
    assert!(res.is_err())
}

#[test]
fn block_new_mainnet_genesis_succ() {
    let block = Block::new_mainnet_genesis().unwrap();

    let res = block.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn block_new_mainnet_genesis_fail() {
    let mut block = Block::new_mainnet_genesis().unwrap();
    block.transactions_ids[0] = Digest::default();

    let res = block.is_genesis();
    assert!(res.is_err())
}

#[test]
fn block_validate_succ() {
    let network_type = NetworkType::RegTest;

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

    let res = block.validate();
    assert!(res.is_ok())
}

#[test]
fn block_validate_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];

    let network_type = NetworkType::default();
    
    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let mut block = Block::new(network_type, &txs).unwrap();
    block.transactions_ids.push(Digest::default());

    let res = block.validate();
    assert!(res.is_err())
}

#[test]
fn block_to_json_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];

    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block_a = Block::new(network_type, &txs).unwrap();
    let block_str = block_a.to_json().unwrap();
    let block_b = Block::from_json(&block_str).unwrap();
    
    assert_eq!(block_a, block_b)
}

#[test]
fn block_to_json_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block = Block::new(network_type, &txs).unwrap();
    let mut block_str = block.to_json().unwrap();
    block_str.pop();
    
    let res = Block::from_json(&block_str);
    assert!(res.is_err())
}

#[test]
fn block_to_bytes_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block_a = Block::new(network_type, &txs).unwrap();
    let block_buf = block_a.to_bytes().unwrap();
    let block_b = Block::from_bytes(&block_buf).unwrap();

    assert_eq!(block_a, block_b)
}

#[test]
fn block_to_bytes_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block = Block::new(network_type, &txs).unwrap();
    let mut block_buf = block.to_bytes().unwrap();
    block_buf[0] ^= block_buf[0];
    
    let res = Block::from_bytes(&block_buf);
    assert!(res.is_err())
}

#[test]
fn block_to_hex_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block_a = Block::new(network_type, &txs).unwrap();
    let block_str = block_a.to_hex().unwrap();
    let block_b = Block::from_hex(&block_str).unwrap();
    
    assert_eq!(block_a, block_b)
}

#[test]
fn block_to_hex_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    
    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let tx = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();
    let txs = vec![tx];

    let block = Block::new(network_type, &txs).unwrap();
    let mut block_str = block.to_hex().unwrap();
    block_str.pop();
    
    let res = Block::from_hex(&block_str);
    assert!(res.is_err())
}
