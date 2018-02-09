// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `network_type` module tests.

use libyobicash::traits::{BinarySerialize, HexSerialize};
use libyobicash::utils::NetworkType;

#[test]
fn network_type_to_bytes_succ() {
    let net_type_a = NetworkType::default();
    let net_type_buf = net_type_a.to_bytes().unwrap();
    let net_type_b = NetworkType::from_bytes(&net_type_buf).unwrap();

    assert_eq!(net_type_a, net_type_b)
}

#[test]
fn network_type_to_bytes_fail() {
    let net_type_a = NetworkType::default();
    let mut net_type_buf = net_type_a.to_bytes().unwrap();
    net_type_buf[3] ^= net_type_buf[3];
    let net_type_b = NetworkType::from_bytes(&net_type_buf).unwrap();

    assert_ne!(net_type_a, net_type_b)
}

#[test]
fn network_type_to_hex_succ() {
    let net_type_a = NetworkType::default();
    let net_type_str = net_type_a.to_hex().unwrap();
    let net_type_b = NetworkType::from_hex(&net_type_str).unwrap();

    assert_eq!(net_type_a, net_type_b)
}

#[test]
fn network_type_to_hex_fail() {
    let net_type = NetworkType::default();

    let mut net_type_str = net_type.to_hex().unwrap();
    net_type_str.pop();
    
    let res = NetworkType::from_hex(&net_type_str);
    assert!(res.is_err())
}
