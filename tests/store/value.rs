// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `value` module tests.

use libyobicash::traits::Serialize;
use libyobicash::crypto::{Random, SecretKey, Key};
use libyobicash::store::StoreValue;

#[test]
fn store_value_new_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();

    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value_a = Random::bytes(value_size);
    let store_value = StoreValue::new(enc_key, &value_a).unwrap();
    let value_b = store_value.decrypt(enc_key).unwrap();
    
    assert_eq!(&value_a, &value_b)
}

#[test]
fn store_value_new_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    let mut store_value = StoreValue::new(enc_key, &value).unwrap();
    let cyph_len = store_value.to_bytes().unwrap().len() as u32;
    store_value.size = cyph_len + 1;
    
    let res = store_value.decrypt(enc_key);
    assert!(res.is_err())
}

#[test]
fn store_value_to_json_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value_a = StoreValue::new(enc_key, &value).unwrap();
    let store_value_str = store_value_a.to_json().unwrap();
    let store_value_b = StoreValue::from_json(&store_value_str).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_json_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value = StoreValue::new(enc_key, &value).unwrap();
    let mut store_value_str = store_value.to_json().unwrap();
    store_value_str.pop();
    
    let res = StoreValue::from_json(&store_value_str);
    assert!(res.is_err())
}

#[test]
fn store_value_to_bytes_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value_a = StoreValue::new(enc_key, &value).unwrap();
    let store_value_buf = store_value_a.to_bytes().unwrap();
    let store_value_b = StoreValue::from_bytes(&store_value_buf).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_bytes_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value = StoreValue::new(enc_key, &value).unwrap();
    let mut store_value_buf = store_value.to_bytes().unwrap();
    store_value_buf[0] ^= store_value_buf[0];
    
    let res = StoreValue::from_bytes(&store_value_buf);
    assert!(res.is_err())
}

#[test]
fn store_value_to_hex_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value_a = StoreValue::new(enc_key, &value).unwrap();
    let store_value_str = store_value_a.to_hex().unwrap();
    let store_value_b = StoreValue::from_hex(&store_value_str).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_hex_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_value = StoreValue::new(enc_key, &value).unwrap();
    let mut store_value_str = store_value.to_hex().unwrap();
    store_value_str.pop();
    
    let res = StoreValue::from_hex(&store_value_str);
    assert!(res.is_err())
}
