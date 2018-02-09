// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `transaction` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Digest, Scalar, ZKPWitness};
use libyobicash::models::output::Output;
use libyobicash::models::coin::{Coin, CoinSource};
use libyobicash::models::transaction::Transaction;

#[test]
fn transaction_new_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let res = Transaction::new(network_type, &coins, &outputs, &fee_output);
    assert!(res.is_ok())
}

#[test]
fn transaction_new_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![in_output];
    
    let res = Transaction::new(network_type, &coins, &outputs, &fee_output);
    assert!(res.is_err())
}

#[test]
fn transaction_validate_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];

    let transaction = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

    let res = transaction.validate();
    assert!(res.is_ok())
}

#[test]
fn transaction_validate_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];

    let mut transaction = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

    transaction.fee.amount += Amount::max_value();

    let res = transaction.validate();
    assert!(res.is_err())
}

#[test]
fn transaction_to_json_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];

    let transaction_a = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

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
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let transaction = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

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
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let transaction_a = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

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
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let transaction = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

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
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let transaction_a = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

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
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();

    let out_amount = Amount::from(8.0);
    let out_instance = Scalar::random();
    let out_witness = ZKPWitness::new(out_instance).unwrap();
    let out_output = Output::new(&out_amount, out_witness).unwrap();
    
    let fee_amount = Amount::from(2.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let outputs = vec![out_output];
    
    let transaction = Transaction::new(network_type, &coins, &outputs, &fee_output).unwrap();

    let mut transaction_str = transaction.to_hex().unwrap();
    transaction_str.pop();

    let res = Transaction::from_hex(&transaction_str);
    assert!(res.is_err())
}
