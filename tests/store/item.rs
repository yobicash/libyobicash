// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `item` module tests.

use libyobicash::traits::Serialize;
use libyobicash::crypto::{SecretKey, Key};
use libyobicash::store::StoreItem;
use mocks::Unit;

#[test]
fn store_item_to_obj_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();

    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit_a = Unit {};
    let store_item = StoreItem::from_object(&unit_a, key).unwrap();
    let unit_b = store_item.to_object::<Unit>(key).unwrap();
    
    assert_eq!(unit_a, unit_b)
}

#[test]
fn store_item_to_obj_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let mut store_item = StoreItem::from_object(&unit, key).unwrap();
    store_item.value.size -= 1;
    
    let res = store_item.to_object::<Unit>(key);
    assert!(res.is_err())
}

#[test]
fn store_item_to_json_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item_a = StoreItem::from_object(&unit, key).unwrap();
    let store_item_str = store_item_a.to_json().unwrap();
    let store_item_b = StoreItem::from_json(&store_item_str).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_json_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item = StoreItem::from_object(&unit, key).unwrap();
    let mut store_item_str = store_item.to_json().unwrap();
    store_item_str.pop();
    
    let res = StoreItem::from_json(&store_item_str);
    assert!(res.is_err())
}

#[test]
fn store_item_to_bytes_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item_a = StoreItem::from_object(&unit, key).unwrap();
    let store_item_buf = store_item_a.to_bytes().unwrap();
    let store_item_b = StoreItem::from_bytes(&store_item_buf).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_bytes_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item = StoreItem::from_object(&unit, key).unwrap();
    let mut store_item_buf = store_item.to_bytes().unwrap();
    store_item_buf[0] ^= store_item_buf[0];
    
    let res = StoreItem::from_bytes(&store_item_buf);
    assert!(res.is_err())
}

#[test]
fn store_item_to_hex_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item_a = StoreItem::from_object(&unit, key).unwrap();
    let store_item_str = store_item_a.to_hex().unwrap();
    let store_item_b = StoreItem::from_hex(&store_item_str).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_hex_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let key = Key::shared(sk_a, pk_b).unwrap();
    
    let unit = Unit {};
    
    let store_item = StoreItem::from_object(&unit, key).unwrap();
    let mut store_item_str = store_item.to_hex().unwrap();
    store_item_str.pop();
    
    let res = StoreItem::from_hex(&store_item_str);
    assert!(res.is_err())
}
