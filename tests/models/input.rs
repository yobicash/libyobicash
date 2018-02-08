// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `input` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{Version, Timestamp, Amount};
use libyobicash::crypto::{Digest, Scalar, ZKPWitness};
use libyobicash::models::input::Input;
use libyobicash::models::output::Output;
use libyobicash::models::coin::{Coin, CoinSource};

#[test]
fn input_new_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();

    let res = Input::new(&coin, &version, timestamp,
                         &outputs_ids, fee_id);
    assert!(res.is_ok())
}

#[test]
fn input_new_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness = ZKPWitness::new(instance_a).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let mut coin = Coin::new(&output, instance_a, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let instance_b = Scalar::random();
    coin.instance = instance_b;
    
    let res = Input::new(&coin, &version, timestamp,
                         &outputs_ids, fee_id);
    assert!(res.is_err())
}

#[test]
fn input_verify_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    
    let verified = input.verify(&output).unwrap();
    assert!(verified)
}

#[test]
fn input_verify_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness_a = ZKPWitness::new(instance_a).unwrap();
    let mut output = Output::new(&amount, witness_a).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance_a, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    let instance_b = Scalar::random();
    let witness_b = ZKPWitness::new(instance_b).unwrap();
    output.witness = witness_b; 
    
    let verified = input.verify(&output).unwrap();
    assert!(!verified)
}

#[test]
fn input_validate_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    
    let res = input.validate();
    assert!(res.is_ok())
}

#[test]
fn input_to_json_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input_a = Input::new(&coin, &version, timestamp,
                             &outputs_ids, fee_id).unwrap();
    let input_str = input_a.to_json().unwrap();
    let input_b = Input::from_json(&input_str).unwrap();
    
    assert_eq!(input_a, input_b)
}

#[test]
fn input_to_json_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    let mut input_str = input.to_json().unwrap();
    input_str.pop();
    
    let res = Input::from_json(&input_str);
    assert!(res.is_err())
}

#[test]
fn input_to_bytes_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input_a = Input::new(&coin, &version, timestamp,
                             &outputs_ids, fee_id).unwrap();
    let input_buf = input_a.to_bytes().unwrap();
    let input_b = Input::from_bytes(&input_buf).unwrap();
    
    assert_eq!(input_a, input_b)
}

#[test]
fn input_to_bytes_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    let mut input_buf = input.to_bytes().unwrap();
    input_buf[0] ^= input_buf[0];
    
    let res = Input::from_bytes(&input_buf);
    assert!(res.is_err())
}

#[test]
fn input_to_hex_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input_a = Input::new(&coin, &version, timestamp,
                             &outputs_ids, fee_id).unwrap();
    let input_str = input_a.to_hex().unwrap();
    let input_b = Input::from_hex(&input_str).unwrap();
    
    assert_eq!(input_a, input_b)
}

#[test]
fn input_to_hex_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();
    let source = CoinSource::default();
    let source_id = Digest::default();

    let coin = Coin::new(&output, instance, source, source_id).unwrap();
    let version = Version::default();
    let timestamp = Timestamp::now();
    let outputs_ids = vec![];
    let fee_id = Digest::default();
    let input = Input::new(&coin, &version, timestamp,
                           &outputs_ids, fee_id).unwrap();
    let mut input_str = input.to_hex().unwrap();
    input_str.pop();
    
    let res = Input::from_hex(&input_str);
    assert!(res.is_err())
}
