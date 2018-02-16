// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `key` module tests.

use libyobicash::traits::{Identify, BinarySerialize};
use libyobicash::store::StoreKey;
use mocks::Unit;

#[test]
fn store_key_to_id_succ() {
    let unit = Unit {};
    
    let unit_a_id = unit.id().unwrap();
    let key_a = StoreKey::from_id::<Unit>(unit_a_id).unwrap();
    let unit_b_id = key_a.to_id::<Unit>().unwrap();

    assert_eq!(unit_a_id, unit_b_id)
}

#[test]
fn store_key_to_bytes_suc() {
    let unit = Unit {};
    let unit_id = unit.id().unwrap();
    
    let key_a = StoreKey::from_id::<Unit>(unit_id).unwrap();
    let key_buf = key_a.to_bytes().unwrap();
    let key_b = StoreKey::from_bytes(&key_buf).unwrap();
    
    assert_eq!(key_a, key_b)
}

#[test]
fn store_key_to_bytes_fail() {
    let unit = Unit {};
    let unit_id = unit.id().unwrap();
    
    let key_a = StoreKey::from_id::<Unit>(unit_id).unwrap();
    let mut key_buf = key_a.to_bytes().unwrap();
    key_buf[0] ^= 1;
    let key_b = StoreKey::from_bytes(&key_buf).unwrap();
    
    assert_ne!(key_a, key_b)
}
