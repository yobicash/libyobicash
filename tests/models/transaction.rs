// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `transaction` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Random, Scalar, ZKPWitness, SecretKey};
use libyobicash::models::output::Output;
use libyobicash::models::data::Data;
use libyobicash::models::coin::Coin;
use libyobicash::models::transaction::Transaction;

#[test]
fn transaction_new_succ() {
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

    let res = Transaction::new(network_type, &coins, &outputs, &ds, &fee);
    assert!(res.is_ok())
}

#[test]
fn transaction_new_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_coin = Coin::new(&in_output, in_instance).unwrap();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let dur = 10;
    let plain = Random::bytes(len);
    let mut data = Data::new(sk_a, pk_b, dur, &plain).unwrap();
    data.duration = 0;

    let fee = Amount::from(2.0);
    
    let coins = vec![in_coin];
    let outputs = vec![in_output];
    let ds = vec![data];
    
    let network_type = NetworkType::default();

    let res = Transaction::new(network_type, &coins, &outputs, &ds, &fee);
    assert!(res.is_err())
}

#[test]
fn transaction_new_regtest_genesis_succ() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let transaction = Transaction::new_regtest_genesis(regtest_witness).unwrap();
    let res = transaction.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn transaction_new_regtest_genesis_fail() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let mut transaction = Transaction::new_regtest_genesis(regtest_witness).unwrap();
    transaction.outputs_length += 1;
    let amount: Amount = 10f32.into();
    transaction.outputs_amount += amount;

    let res = transaction.is_genesis();
    assert!(res.is_err())
}

#[test]
fn transaction_new_testnet_genesis_succ() {
    let transaction = Transaction::new_testnet_genesis().unwrap();

    let res = transaction.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn transaction_new_testnet_genesis_fail() {
    let mut transaction = Transaction::new_testnet_genesis().unwrap();
    transaction.outputs_length += 1;
    let amount: Amount = 10f32.into();
    transaction.outputs_amount += amount;

    let res = transaction.is_genesis();
    assert!(res.is_err())
}

#[test]
fn transaction_new_mainnet_genesis_new_succ() {
    let transaction = Transaction::new_mainnet_genesis().unwrap();

    let res = transaction.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn transaction_new_mainnet_genesis_fail() {
    let mut transaction = Transaction::new_mainnet_genesis().unwrap();
    transaction.outputs_length += 1;
    let amount: Amount = 10f32.into();
    transaction.outputs_amount += amount;

    let res = transaction.is_genesis();
    assert!(res.is_err())
}

#[test]
fn transaction_validate_succ() {
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

    let transaction = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let res = transaction.validate();
    assert!(res.is_ok())
}

#[test]
fn transaction_validate_fail() {
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
    
    let mut transaction = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    transaction.fee += Amount::one();

    let res = transaction.validate();
    assert!(res.is_err())
}

#[test]
fn transaction_to_json_succ() {
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

    let transaction_a = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let transaction_str = transaction_a.to_json().unwrap();
    let transaction_b = Transaction::from_json(&transaction_str).unwrap();
    
    assert_eq!(transaction_a, transaction_b)
}

#[test]
fn transaction_to_json_fail() {
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

    let transaction = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let mut transaction_str = transaction.to_json().unwrap();
    transaction_str.pop();
    
    let res = Transaction::from_json(&transaction_str);
    assert!(res.is_err())
}

#[test]
fn transaction_to_bytes_succ() {
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

    let transaction_a = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let transaction_buf = transaction_a.to_bytes().unwrap();
    let transaction_b = Transaction::from_bytes(&transaction_buf).unwrap();
    assert_eq!(transaction_a, transaction_b)
}

#[test]
fn transaction_to_bytes_fail() {
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
    
    let transaction = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let mut transaction_buf = transaction.to_bytes().unwrap();
    transaction_buf[0] ^= transaction_buf[0];
    
    let res = Transaction::from_bytes(&transaction_buf);
    assert!(res.is_err())
}

#[test]
fn transaction_to_hex_succ() {
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

    let transaction_a = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let transaction_str = transaction_a.to_hex().unwrap();
    let transaction_b = Transaction::from_hex(&transaction_str).unwrap();
    assert_eq!(transaction_a, transaction_b)
}

#[test]
fn transaction_to_hex_fail() {
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

    let transaction = Transaction::new(network_type, &coins, &outputs, &ds, &fee).unwrap();

    let mut transaction_str = transaction.to_hex().unwrap();
    transaction_str.pop();

    let res = Transaction::from_hex(&transaction_str);
    assert!(res.is_err())
}
