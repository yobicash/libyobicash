// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `data` module tests.

use libyobicash::traits::{Validate, Serialize};
use libyobicash::crypto::{Random, SecretKey};
use libyobicash::utils::NetworkType;
use libyobicash::models::data::Data;

#[test]
fn data_new_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let pk_b = SecretKey::random().to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let res = Data::new(network_type, sk_a, pk_b, &plain);
    assert!(res.is_ok())
}

#[test]
fn data_new_fail() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let pk_b = sk_a.to_public();
    let len = 10;
    let plain = Random::bytes(len);
    
    let res = Data::new(network_type, sk_a, pk_b, &plain);
    assert!(res.is_err())
}

#[test]
fn data_decrypt_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    
    let plain_a = Random::bytes(len);
    let data = Data::new(network_type, sk_a, pk_b, &plain_a).unwrap();
    let plain_b = data.decrypt(sk_b).unwrap();
    
    assert_eq!(plain_a, plain_b)
}

#[test]
fn data_decrypt_fail() {
    let network_type = NetworkType::default();
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    
    let plain_a = Random::bytes(len);
    let mut data = Data::new(network_type, sk_a, pk_b, &plain_a).unwrap();
    data.plain_size -= 1;
    let plain_b = data.decrypt(sk_b).unwrap();
    
    assert_ne!(plain_a, plain_b)
}

#[test]
fn data_validate_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let data = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    
    let res = data.validate();
    assert!(res.is_ok())
}

#[test]
fn data_validate_fail() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let mut data = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    data.cyph_size += 16;
    
    let res = data.validate();
    assert!(res.is_err())
}

#[test]
fn data_to_json_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let data_str = data_a.to_json().unwrap();
    let data_b = Data::from_json(&data_str).unwrap();
    
    assert_eq!(data_a, data_b)
}

#[test]
fn data_to_json_fail() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let mut data_str = data_a.to_json().unwrap();
    data_str.pop();
    
    let res = Data::from_json(&data_str);
    assert!(res.is_err())
}

#[test]
fn data_to_bytes_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let data_buf = data_a.to_bytes().unwrap();
    let data_b = Data::from_bytes(&data_buf).unwrap();
    
    assert_eq!(data_a, data_b)
}

#[test]
fn data_to_bytes_fail() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);

    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let mut data_buf = data_a.to_bytes().unwrap();
    data_buf[0] ^= data_buf[0];
    
    let res = Data::from_bytes(&data_buf);
    assert!(res.is_err())
}

#[test]
fn data_to_hex_succ() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);
    
    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let data_str = data_a.to_hex().unwrap();
    let data_b = Data::from_hex(&data_str).unwrap();
    
    assert_eq!(data_a, data_b)
}

#[test]
fn data_to_hex_fail() {
    let network_type = NetworkType::default();
    
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    let len = 10;
    let plain = Random::bytes(len);
    
    let data_a = Data::new(network_type, sk_a, pk_b, &plain).unwrap();
    let mut data_str = data_a.to_hex().unwrap();
    data_str.pop();
    
    let res = Data::from_hex(&data_str);
    assert!(res.is_err())
}
