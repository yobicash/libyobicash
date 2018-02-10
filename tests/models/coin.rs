// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `coin` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Random, Digest, Scalar, ZKPWitness, ZKPProof};
use libyobicash::models::output::Output;
use libyobicash::models::coin::{CoinSource, Coin};

#[test]
fn coin_new_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let res = Coin::new(source, source_id, &output, instance);
    assert!(res.is_ok())
}

#[test]
fn coin_new_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness_a = ZKPWitness::new(instance_a).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness_a).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();
    let instance_b = Scalar::random();

    let res = Coin::new(source, source_id, &output, instance_b);
    assert!(res.is_err())
}

#[test]
fn coin_verify_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance).unwrap();
    
    let message = Random::bytes(64);
    let proof = ZKPProof::new(instance, &message).unwrap();
    
    let verified = coin.verify(proof).unwrap();
    assert!(verified)
}

#[test]
fn coin_verify_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness_a = ZKPWitness::new(instance_a).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness_a).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance_a).unwrap();
    
    let message = Random::bytes(64);
    let instance_b = Scalar::random();
    let proof = ZKPProof::new(instance_b, &message).unwrap();
    
    let verified = coin.verify(proof).unwrap();
    assert!(!verified)
}

#[test]
fn coin_validate_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance).unwrap();
    
    let res = coin.validate();
    assert!(res.is_ok())
}

#[test]
fn coin_validate_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let mut coin = Coin::new(source, source_id, &output, instance).unwrap();
    coin.instance = Scalar::random();
    
    let res = coin.validate();
    assert!(res.is_err())
}

#[test]
fn coin_to_json_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin_a = Coin::new(source, source_id, &output, instance).unwrap();
    let coin_str = coin_a.to_json().unwrap();
    let coin_b = Coin::from_json(&coin_str).unwrap();
    
    assert_eq!(coin_a, coin_b)
}

#[test]
fn coin_to_json_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance).unwrap();
    let mut coin_str = coin.to_json().unwrap();
    coin_str.pop();
    
    let res = Coin::from_json(&coin_str);
    assert!(res.is_err())
}

#[test]
fn coin_to_bytes_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin_a = Coin::new(source, source_id, &output, instance).unwrap();
    let coin_buf = coin_a.to_bytes().unwrap();
    let coin_b = Coin::from_bytes(&coin_buf).unwrap();
    
    assert_eq!(coin_a, coin_b)
}

#[test]
fn coin_to_bytes_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance).unwrap();
    let mut coin_buf = coin.to_bytes().unwrap();
    coin_buf[0] ^= coin_buf[0];
    
    let res = Coin::from_bytes(&coin_buf);
    assert!(res.is_err())
}

#[test]
fn coin_to_hex_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin_a = Coin::new(source, source_id, &output, instance).unwrap();
    let coin_str = coin_a.to_hex().unwrap();
    let coin_b = Coin::from_hex(&coin_str).unwrap();
    
    assert_eq!(coin_a, coin_b)
}

#[test]
fn coin_to_hex_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let network_type = NetworkType::default();
    let output = Output::new(network_type, &amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(source, source_id, &output, instance).unwrap();
    let mut coin_str = coin.to_hex().unwrap();
    coin_str.pop();
    
    let res = Coin::from_hex(&coin_str);
    assert!(res.is_err())
}
