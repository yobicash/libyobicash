// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `value` module tests.

use libyobicash::traits::Serialize;
use libyobicash::crypto::{SecretKey, Key};
use libyobicash::store::StoreValue;
use mocks::Unit;

#[test]
fn store_value_to_obj_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();

    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit_a = Unit {};
    let store_value = StoreValue::from_object(&unit_a, key).unwrap();
    let unit_b = store_value.to_object::<Unit>(key).unwrap();
    
    assert_eq!(unit_a, unit_b)
}

#[test]
fn store_value_to_obj_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let mut store_value = StoreValue::from_object(&unit, key).unwrap();
    store_value.size -= 1;
    
    let res = store_value.to_object::<Unit>(key);
    assert!(res.is_err())
}

#[test]
fn store_value_to_json_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value_a = StoreValue::from_object(&unit, key).unwrap();
    let store_value_str = store_value_a.to_json().unwrap();
    let store_value_b = StoreValue::from_json(&store_value_str).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_json_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value = StoreValue::from_object(&unit, key).unwrap();
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
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value_a = StoreValue::from_object(&unit, key).unwrap();
    let store_value_buf = store_value_a.to_bytes().unwrap();
    let store_value_b = StoreValue::from_bytes(&store_value_buf).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_bytes_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value = StoreValue::from_object(&unit, key).unwrap();
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
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value_a = StoreValue::from_object(&unit, key).unwrap();
    let store_value_str = store_value_a.to_hex().unwrap();
    let store_value_b = StoreValue::from_hex(&store_value_str).unwrap();
    
    assert_eq!(store_value_a, store_value_b)
}

#[test]
fn store_value_to_hex_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_value = StoreValue::from_object(&unit, key).unwrap();
    let mut store_value_str = store_value.to_hex().unwrap();
    store_value_str.pop();
    
    let res = StoreValue::from_hex(&store_value_str);
    assert!(res.is_err())
}
