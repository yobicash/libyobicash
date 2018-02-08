// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `output` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{Version, Timestamp, Amount};
use libyobicash::crypto::{Digest, Scalar, ZKPWitness, ZKPProof};
use libyobicash::models::output::Output;
use libyobicash::models::coin::{Coin, CoinSource};
use libyobicash::models::input::Input;

#[test]
fn output_new_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let res = Output::new(&amount, witness);
    assert!(res.is_ok())
}

#[test]
fn output_new_fail() {
    let mut amount = Amount::max_value();
    amount += Amount::from(1.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let res = Output::new(&amount, witness);
    assert!(res.is_err())
}

#[test]
fn output_verify_succ() {
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
    
    let verified = output.verify(&input).unwrap();
    assert!(verified)
}

#[test]
fn output_verify_fail() {
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
    let mut input = Input::new(&coin, &version, timestamp,
                               &outputs_ids, fee_id).unwrap();
    input.proof = ZKPProof::default();
    
    let verified = output.verify(&input).unwrap();
    assert!(!verified)
}

#[test]
fn output_validate_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output = Output::new(&amount, witness).unwrap();
    
    let res = output.validate();
    assert!(res.is_ok())
}

#[test]
fn output_validate_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let mut output = Output::new(&amount, witness).unwrap();
    output.amount += Amount::max_value();
    
    let res = output.validate();
    assert!(res.is_err())
}

#[test]
fn output_to_json_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output_a = Output::new(&amount, witness).unwrap();
    let output_str = output_a.to_json().unwrap();
    let output_b = Output::from_json(&output_str).unwrap();
    
    assert_eq!(output_a, output_b)
}

#[test]
fn output_to_json_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output = Output::new(&amount, witness).unwrap();
    let mut output_str = output.to_json().unwrap();
    output_str.pop();
    
    let res = Output::from_json(&output_str);
    assert!(res.is_err())
}

#[test]
fn output_to_bytes_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output_a = Output::new(&amount, witness).unwrap();
    let output_buf = output_a.to_bytes().unwrap();
    let output_b = Output::from_bytes(&output_buf).unwrap();
    
    assert_eq!(output_a, output_b)
}

#[test]
fn output_to_bytes_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output = Output::new(&amount, witness).unwrap();
    let mut output_buf = output.to_bytes().unwrap();
    output_buf[0] ^= output_buf[0];
    
    let res = Output::from_bytes(&output_buf);
    assert!(res.is_err())
}

#[test]
fn output_to_hex_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output_a = Output::new(&amount, witness).unwrap();
    let output_str = output_a.to_hex().unwrap();
    let output_b = Output::from_hex(&output_str).unwrap();
    
    assert_eq!(output_a, output_b)
}

#[test]
fn output_to_hex_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    
    let output = Output::new(&amount, witness).unwrap();
    let mut output_str = output.to_hex().unwrap();
    output_str.pop();
    let res = Output::from_hex(&output_str);
    
    assert!(res.is_err())
}
