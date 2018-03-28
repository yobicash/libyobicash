// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `input` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::Amount;
use libyobicash::crypto::{Random, Scalar, ZKPWitness};
use libyobicash::models::input::Input;
use libyobicash::models::output::Output;
use libyobicash::models::coin::Coin;

#[test]
fn input_new_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);

    let res = Input::new(&coin, &message);
    assert!(res.is_ok())
}

#[test]
fn input_new_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness = ZKPWitness::new(instance_a).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let mut coin = Coin::new(&output, instance_a).unwrap();
    let instance_b = Scalar::random();
    coin.instance = instance_b;
    
    let len = 10;
    let message = Random::bytes(len);
    
    let res = Input::new(&coin, &message);
    assert!(res.is_err())
}

#[test]
fn input_verify_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
    
    let verified = input.verify(&output).unwrap();
    assert!(verified)
}

#[test]
fn input_verify_fail() {
    let amount = Amount::from(10.0);
    let instance_a = Scalar::random();
    let witness_a = ZKPWitness::new(instance_a).unwrap();
    let mut output = Output::new(&amount, witness_a).unwrap();

    let coin = Coin::new(&output, instance_a).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
    let instance_b = Scalar::random();
    let witness_b = ZKPWitness::new(instance_b).unwrap();
    output.witness = witness_b; 
    
    let res = input.verify(&output);
    assert!(res.is_err())
}

#[test]
fn input_validate_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
    
    let res = input.validate();
    assert!(res.is_ok())
}

#[test]
fn input_to_json_succ() {
    let amount = Amount::from(10.0);
    let instance = Scalar::random();
    let witness = ZKPWitness::new(instance).unwrap();
    let output = Output::new(&amount, witness).unwrap();

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input_a = Input::new(&coin, &message).unwrap();
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

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
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

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input_a = Input::new(&coin, &message).unwrap();
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

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
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

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input_a = Input::new(&coin, &message).unwrap();
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

    let coin = Coin::new(&output, instance).unwrap();
    
    let len = 10;
    let message = Random::bytes(len);
    
    let input = Input::new(&coin, &message).unwrap();
    let mut input_str = input.to_hex().unwrap();
    input_str.pop();
    
    let res = Input::from_hex(&input_str);
    assert!(res.is_err())
}
