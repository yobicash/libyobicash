// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `write_op` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Random, Digest, Scalar, ZKPWitness, SecretKey};
use libyobicash::models::output::Output;
use libyobicash::models::coin::{Coin, CoinSource};
use libyobicash::models::data::Data;
use libyobicash::models::delete_op::DeleteOp;
use libyobicash::models::write_op::WriteOp;

#[test]
fn write_op_new_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let res = WriteOp::new(network_type, &coins, &data, instance, &fee_output);
    assert!(res.is_ok())
}

#[test]
fn write_op_new_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = in_instance;
    
    let res = WriteOp::new(network_type, &coins, &data, instance, &fee_output);
    println!("res: {:?}", res);
    assert!(res.is_err())
}

#[test]
fn write_op_validate_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let res = write_op.validate();
    assert!(res.is_ok())
}

#[test]
fn write_op_validate_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let mut write_op = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    write_op.fee.amount += Amount::max_value();

    let res = write_op.validate();
    assert!(res.is_err())
}

#[test]
fn write_op_verify_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    // same fee just for readability.
    let proof = DeleteOp::proof(network_type, &write_op, instance, &fee_output).unwrap();
    
    // same coins and fee as above just for testing
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let verified = write_op.verify(&delete_op).unwrap();
    assert!(verified)
}

#[test]
fn write_op_verify_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let mut write_op = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    // same fee just for readability.
    let proof = DeleteOp::proof(network_type, &write_op, instance, &fee_output).unwrap();
    
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let instance_b = Scalar::random();
    let witness_b = ZKPWitness::new(instance_b).unwrap();
    write_op.witness = witness_b;

    let verified = write_op.verify(&delete_op).unwrap();
    assert!(!verified)
}

#[test]
fn write_op_to_json_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let write_op_str = write_op_a.to_json().unwrap();
    let write_op_b = WriteOp::from_json(&write_op_str).unwrap();
    
    assert_eq!(write_op_a, write_op_b)
}

#[test]
fn write_op_to_json_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let mut write_op_str = write_op_a.to_json().unwrap();
    write_op_str.pop();
    
    let res = WriteOp::from_json(&write_op_str);
    assert!(res.is_err())
}

#[test]
fn write_op_to_bytes_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let write_op_buf = write_op_a.to_bytes().unwrap();
    let write_op_b = WriteOp::from_bytes(&write_op_buf).unwrap();
    assert_eq!(write_op_a, write_op_b)
}

#[test]
fn write_op_to_bytes_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let mut write_op_buf = write_op_a.to_bytes().unwrap();
    write_op_buf[0] ^= write_op_buf[0];
    
    let res = WriteOp::from_bytes(&write_op_buf);
    assert!(res.is_err())
}

#[test]
fn write_op_to_hex_succ() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let write_op_str = write_op_a.to_hex().unwrap();
    let write_op_b = WriteOp::from_hex(&write_op_str).unwrap();
    assert_eq!(write_op_a, write_op_b)
}

#[test]
fn write_op_to_hex_fail() {
    let in_amount = Amount::from(10.0);
    let in_instance = Scalar::random();
    let in_witness = ZKPWitness::new(in_instance).unwrap();
    let in_output = Output::new(&in_amount, in_witness).unwrap();
    let in_source = CoinSource::default();
    let in_source_id = Digest::default();
    let network_type = NetworkType::default();
    let in_coin = Coin::new(network_type, in_source, in_source_id, &in_output, in_instance).unwrap();
    
    let plain_size = 10;
    let plain = Random::bytes(plain_size);
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let data = Data::new(sk_a, pk_b, &plain).unwrap();
    
    let fee_amount = Amount::from(10.0);
    let fee_instance = Scalar::random();
    let fee_witness = ZKPWitness::new(fee_instance).unwrap();
    let fee_output = Output::new(&fee_amount, fee_witness).unwrap();
    
    let coins = vec![in_coin];
    let instance = Scalar::random();
    
    let write_op_a = WriteOp::new(network_type, &coins, &data, instance, &fee_output).unwrap();

    let mut write_op_str = write_op_a.to_hex().unwrap();
    write_op_str.pop();

    let res = WriteOp::from_hex(&write_op_str);
    assert!(res.is_err())
}
