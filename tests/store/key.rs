// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `key` module tests.

use libyobicash::traits::{BinarySerialize, HexSerialize};
use libyobicash::crypto::Random;
use libyobicash::store::StoreKey;

#[test]
fn store_key_to_bytes_suc() {
    let key_size = 10;
    let key = Random::bytes(key_size);
    
    let key_a = StoreKey::new(&key);
    let key_buf = key_a.to_bytes().unwrap();
    let key_b = StoreKey::from_bytes(&key_buf).unwrap();
    
    assert_eq!(key_a, key_b)
}

#[test]
fn store_key_to_bytes_fail() {
    let key_size = 10;
    let key = Random::bytes(key_size);
    
    let key_a = StoreKey::new(&key);
    let mut key_buf = key_a.to_bytes().unwrap();
    key_buf[0] ^= 1;
    let key_b = StoreKey::from_bytes(&key_buf).unwrap();
    
    assert_ne!(key_a, key_b)
}

#[test]
fn store_key_to_hex_suc() {
    let key_size = 10;
    let key = Random::bytes(key_size);
    
    let key_a = StoreKey::new(&key);
    let key_buf = key_a.to_hex().unwrap();
    let key_b = StoreKey::from_hex(&key_buf).unwrap();
    
    assert_eq!(key_a, key_b)
}

#[test]
fn store_key_to_hex_fail() {
    let key_size = 10;
    let key = Random::bytes(key_size);
    
    let key_a = StoreKey::new(&key);
    let mut key_str = key_a.to_hex().unwrap();
    key_str.pop();

    let res = StoreKey::from_hex(&key_str);
    assert!(res.is_err())
}
