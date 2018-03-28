// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `output` module tests.

use libyobicash::constants::{MAINWITNESS, TESTWITNESS};
use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::Amount;
use libyobicash::crypto::{Random, Scalar, ZKPWitness, ZKPProof};
use libyobicash::crypto::HexSerialize;
use libyobicash::models::output::Output;
use libyobicash::models::coin::Coin;
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
    let amount = Amount::from(1.0);
    let witness = ZKPWitness::from_hex(MAINWITNESS).unwrap();
    
    let res = Output::new(&amount, witness);
    assert!(res.is_err())
}

#[test]
fn output_new_regtest_genesis_succ() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let output = Output::new_regtest_genesis(regtest_witness).unwrap();
    let res = output.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn output_new_regtest_genesis_fail() {
    let regtest_instance = Scalar::random();
    let regtest_witness = ZKPWitness::new(regtest_instance).unwrap();

    let mut output = Output::new_regtest_genesis(regtest_witness).unwrap();
    output.amount = Amount::new();
    let is_genesis = output.is_genesis().unwrap();
    assert!(!is_genesis)
}

#[test]
fn output_new_testnet_genesis_succ() {
    let output = Output::new_testnet_genesis().unwrap();

    let res = output.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn output_new_testnet_genesis_fail() {
    let mut output = Output::new_testnet_genesis().unwrap();
    output.amount = Amount::new();

    let res = output.is_genesis();
    assert!(res.is_err())
}

#[test]
fn output_new_mainnet_genesis_succ() {
    let output = Output::new_testnet_genesis().unwrap();

    let res = output.is_genesis();
    assert!(res.is_ok())
}

#[test]
fn output_new_mainnet_genesis_fail() {
    let mut output = Output::new_mainnet_genesis().unwrap();
    output.amount = Amount::new();

    let res = output.is_genesis();
    assert!(res.is_err())
}

#[test]
fn output_verify_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
    
    let verified = output.verify(&input).unwrap();
    assert!(verified)
}

#[test]
fn output_verify_fail() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    let len = 10;
    let message = Random::bytes(len);
    
    let mut input = Input::new(&coin, &message).unwrap();
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
    output.witness = ZKPWitness::from_hex(TESTWITNESS).unwrap();
    
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
