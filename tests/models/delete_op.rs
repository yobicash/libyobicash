// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `delete_op` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::utils::{NetworkType, Amount};
use libyobicash::crypto::{Random, Digest, Scalar, ZKPWitness, SecretKey};
use libyobicash::models::output::Output;
use libyobicash::models::coin::{Coin, CoinSource};
use libyobicash::models::data::Data;
use libyobicash::models::write_op::WriteOp;
use libyobicash::models::delete_op::DeleteOp;

#[test]
fn delete_op_new_succ() {
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
   
    // same coins for testing purposes
    let res = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output);
    assert!(res.is_ok())
}

#[test]
fn delete_op_new_fail() {
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
    let instance_a = Scalar::random();
    
    let write_op = WriteOp::new(network_type, &coins, &data, instance_a, &fee_output).unwrap();

    let instance_b = Scalar::random();
    // same fee just for readability.
    let proof = DeleteOp::proof(network_type, &write_op, instance_b, &fee_output).unwrap();
   
    // same coins for testing purposes
    let res = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output);
    assert!(res.is_err())
}

#[test]
fn delete_op_validate_succ() {
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
   
    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let res = delete_op.validate();
    assert!(res.is_ok())
}

#[test]
fn delete_op_validate_fail() {
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
   
    // same coins for testing purposes
    let mut delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();
    delete_op.fee.amount += Amount::max_value();

    let res = delete_op.validate();
    assert!(res.is_err())
}

#[test]
fn delete_op_verify_succ() {
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
   
    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let verified = delete_op.verify(&write_op).unwrap();
    assert!(verified)
}

#[test]
fn delete_op_verify_fail() {
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
   
    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let instance_b = Scalar::random();
    let witness_b = ZKPWitness::new(instance_b).unwrap();
    write_op.witness = witness_b;

    let verified = delete_op.verify(&write_op).unwrap();
    assert!(!verified)
}

#[test]
fn delete_op_to_json_succ() {
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
   
    // same coins for testing purposes
    let delete_op_a = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let delete_op_str = delete_op_a.to_json().unwrap();
    let delete_op_b = DeleteOp::from_json(&delete_op_str).unwrap();
    
    assert_eq!(delete_op_a, delete_op_b)
}

#[test]
fn delete_op_to_json_fail() {
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
   
    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let mut delete_op_str = delete_op.to_json().unwrap();
    delete_op_str.pop();
    
    let res = DeleteOp::from_json(&delete_op_str);
    assert!(res.is_err())
}

#[test]
fn delete_op_to_bytes_succ() {
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

    // same coins for testing purposes
    let delete_op_a = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let delete_op_buf = delete_op_a.to_bytes().unwrap();
    let delete_op_b = DeleteOp::from_bytes(&delete_op_buf).unwrap();
    assert_eq!(delete_op_a, delete_op_b)
}

#[test]
fn delete_op_to_bytes_fail() {
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

    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let mut delete_op_buf = delete_op.to_bytes().unwrap();
    delete_op_buf[0] ^= delete_op_buf[0];
    
    let res = DeleteOp::from_bytes(&delete_op_buf);
    assert!(res.is_err())
}

#[test]
fn delete_op_to_hex_succ() {
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

    // same coins for testing purposes
    let delete_op_a = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let delete_op_str = delete_op_a.to_hex().unwrap();
    let delete_op_b = DeleteOp::from_hex(&delete_op_str).unwrap();
    assert_eq!(delete_op_a, delete_op_b)
}

#[test]
fn delete_op_to_hex_fail() {
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

    // same coins for testing purposes
    let delete_op = DeleteOp::new(network_type, &coins, &write_op, proof, &fee_output).unwrap();

    let mut delete_op_str = delete_op.to_hex().unwrap();
    delete_op_str.pop();

    let res = DeleteOp::from_hex(&delete_op_str);
    assert!(res.is_err())
}
