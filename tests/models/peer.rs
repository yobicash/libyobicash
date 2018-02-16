// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `peer` model tests.

use libyobicash::traits::{Serialize, Validate};
use libyobicash::utils::Timestamp;
use libyobicash::models::peer::Peer;

#[test]
fn peer_new_succ() {
    let addr = String::new();
    
    let res = Peer::new(&addr);
    assert!(res.is_ok())
}

#[test]
fn peer_validate_succ() {
    let addr = String::new();
    
    let peer = Peer::new(&addr).unwrap();
    
    let res = peer.validate();
    assert!(res.is_ok())
}

#[test]
fn peer_seen_succ() {
    let addr = String::new();
    
    let mut peer = Peer::new(&addr).unwrap();
    
    let res = peer.seen();
    assert!(res.is_ok())
}

#[test]
fn peer_seen_fail() {
    let addr = String::new();
    
    let mut peer = Peer::new(&addr).unwrap();
    peer.created_at = Timestamp::now();
    peer.updated_at = Timestamp::min_value();
    
    let res = peer.seen();
    assert!(res.is_err())
}

#[test]
fn peer_to_json_succ() {
    let addr = String::new();
    
    let peer_a = Peer::new(&addr).unwrap();
    let peer_str = peer_a.to_json().unwrap();
    let peer_b = Peer::from_json(&peer_str).unwrap();
    
    assert_eq!(peer_a, peer_b)
}

#[test]
fn peer_to_json_fail() {
    let addr = String::new();
    
    let peer = Peer::new(&addr).unwrap();
    let mut peer_str = peer.to_json().unwrap();
    peer_str.pop();
    
    let res = Peer::from_json(&peer_str);
    assert!(res.is_err())
}

#[test]
fn peer_to_bytes_succ() {
    let addr = String::new();
    
    let peer_a = Peer::new(&addr).unwrap();
    let peer_buf = peer_a.to_bytes().unwrap();
    let peer_b = Peer::from_bytes(&peer_buf).unwrap();
    
    assert_eq!(peer_a, peer_b)
}

#[test]
fn peer_to_bytes_fail() {
    let addr = String::new();
    
    let peer = Peer::new(&addr).unwrap();
    let mut peer_buf = peer.to_bytes().unwrap();
    peer_buf[0] ^= peer_buf[0];
    
    let res = Peer::from_bytes(&peer_buf);
    assert!(res.is_err())
}

#[test]
fn peer_to_hex_succ() {
    let addr = String::new();
    
    let peer_a = Peer::new(&addr).unwrap();
    let peer_str = peer_a.to_hex().unwrap();
    let peer_b = Peer::from_hex(&peer_str).unwrap();
    
    assert_eq!(peer_a, peer_b)
}

#[test]
fn peer_to_hex_fail() {
    let addr = String::new();
    
    let peer = Peer::new(&addr).unwrap();
    let mut peer_str = peer.to_hex().unwrap();
    peer_str.pop();
    
    let res = Peer::from_hex(&peer_str);
    assert!(res.is_err())
}
