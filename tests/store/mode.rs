// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `store_mode` module tests.

use libyobicash::traits::{BinarySerialize, HexSerialize};
use libyobicash::store::StoreMode;

#[test]
fn store_mode_to_bytes_succ() {
    let store_mode_a = StoreMode::default();
    let store_mode_buf = store_mode_a.to_bytes().unwrap();
    let store_mode_b = StoreMode::from_bytes(&store_mode_buf).unwrap();

    assert_eq!(store_mode_a, store_mode_b)
}

#[test]
fn store_mode_to_bytes_fail() {
    let store_mode_a = StoreMode::default();
    let mut store_mode_buf = store_mode_a.to_bytes().unwrap();
    store_mode_buf[3] ^= 1;
    let store_mode_b = StoreMode::from_bytes(&store_mode_buf).unwrap();

    assert_ne!(store_mode_a, store_mode_b)
}

#[test]
fn store_mode_to_hex_succ() {
    let store_mode_a = StoreMode::default();
    let store_mode_str = store_mode_a.to_hex().unwrap();
    let store_mode_b = StoreMode::from_hex(&store_mode_str).unwrap();

    assert_eq!(store_mode_a, store_mode_b)
}

#[test]
fn store_mode_to_hex_fail() {
    let store_mode = StoreMode::default();

    let mut store_mode_str = store_mode.to_hex().unwrap();
    store_mode_str.pop();
    
    let res = StoreMode::from_hex(&store_mode_str);
    assert!(res.is_err())
}
