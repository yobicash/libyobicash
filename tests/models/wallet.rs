// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://openout_outputurce.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://openout_outputurce.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `wallet` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Digest, Scalar, ZKPWitness};
use libyobicash::models::Output;
use libyobicash::models::{Coin, CoinSource};
use libyobicash::models::Wallet;

#[test]
fn wallet_add_ucoin_succ() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    let res = wallet.add_ucoin(&coin);
    assert!(res.is_ok())
}

#[test]
fn wallet_add_ucoin_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);
    
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();

    wallet.add_ucoin(&coin).unwrap();
    
    let res = wallet.add_ucoin(&coin);
    assert!(res.is_err())
}

#[test]
fn wallet_add_scoin_succ() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet.add_ucoin(&coin).unwrap();

    let res = wallet.add_scoin(&coin);
    assert!(res.is_ok())
}

#[test]
fn wallet_add_scoin_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();

    let res = wallet.add_scoin(&coin);
    assert!(res.is_err())
}

#[test]
fn wallet_validate_succ() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet.add_ucoin(&coin).unwrap();
    wallet.add_scoin(&coin).unwrap();

    let res = wallet.validate();
    assert!(res.is_ok())
}

#[test]
fn wallet_validate_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet.add_ucoin(&coin).unwrap();
    wallet.add_scoin(&coin).unwrap();

    wallet.ucoins_length -= 1;

    let res = wallet.validate();
    assert!(res.is_err())
}

#[test]
fn wallet_to_json_succ() {
    let name = String::from("wallet_name");
    let mut wallet_a = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet_a.add_ucoin(&coin).unwrap();

    let wallet_str = wallet_a.to_json().unwrap();
    let wallet_b = Wallet::from_json(&wallet_str).unwrap();
    assert_eq!(wallet_a, wallet_b)
}

#[test]
fn wallet_to_json_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet.add_ucoin(&coin).unwrap();

    let mut wallet_str = wallet.to_json().unwrap();
    wallet_str.pop();
    let res = Wallet::from_json(&wallet_str);
    assert!(res.is_err())
}

#[test]
fn wallet_to_bytes_succ() {
    let name = String::from("wallet_name");
    let mut wallet_a = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet_a.add_ucoin(&coin).unwrap();

    let wallet_buf = wallet_a.to_bytes().unwrap();
    let wallet_b = Wallet::from_bytes(&wallet_buf).unwrap();
    assert_eq!(wallet_a, wallet_b)
}

#[test]
fn wallet_to_bytes_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet.add_ucoin(&coin).unwrap();

    let mut wallet_buf = wallet.to_bytes().unwrap();
    wallet_buf[0] ^= wallet_buf[0];
    let res = Wallet::from_bytes(&wallet_buf);
    assert!(res.is_err())
}

#[test]
fn wallet_to_hex_succ() {
    let name = String::from("wallet_name");
    let mut wallet_a = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    wallet_a.add_ucoin(&coin).unwrap();

    let wallet_str = wallet_a.to_hex().unwrap();
    let wallet_b = Wallet::from_hex(&wallet_str).unwrap();
    assert_eq!(wallet_a, wallet_b)
}

#[test]
fn wallet_to_hex_fail() {
    let name = String::from("wallet_name");
    let mut wallet = Wallet::new(&name);

    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let network_type = NetworkType::default();

    let coin = Coin::new(network_type, source, source_id, &output, instance).unwrap();
    
    
    wallet.add_ucoin(&coin).unwrap();

    let mut wallet_str = wallet.to_hex().unwrap();
    wallet_str.pop();
    let res = Wallet::from_hex(&wallet_str);
    assert!(res.is_err())
}
