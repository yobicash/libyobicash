// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `item` module tests.

use libyobicash::traits::Serialize;
use libyobicash::crypto::{Random, SecretKey, Key};
use libyobicash::store::StoreItem;

#[test]
fn store_item_new_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();

    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value_a = Random::bytes(value_size);
    let store_item = StoreItem::new(enc_key, &key, &value_a).unwrap();
    let value_b = store_item.decrypt(enc_key).unwrap();
    
    assert_eq!(&value_a, &value_b)
}

#[test]
fn store_item_new_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    let mut store_item = StoreItem::new(enc_key, &key, &value).unwrap();
    let cyph_len = store_item.value.to_bytes().unwrap().len() as u32;
    store_item.value.size = cyph_len + 1;
    
    let res = store_item.decrypt(enc_key);
    assert!(res.is_err())
}

#[test]
fn store_item_to_json_succ() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item_a = StoreItem::new(enc_key, &key, &value).unwrap();
    let store_item_str = store_item_a.to_json().unwrap();
    let store_item_b = StoreItem::from_json(&store_item_str).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_json_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item = StoreItem::new(enc_key, &key, &value).unwrap();
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
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item_a = StoreItem::new(enc_key, &key, &value).unwrap();
    let store_item_buf = store_item_a.to_bytes().unwrap();
    let store_item_b = StoreItem::from_bytes(&store_item_buf).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_bytes_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item = StoreItem::new(enc_key, &key, &value).unwrap();
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
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item_a = StoreItem::new(enc_key, &key, &value).unwrap();
    let store_item_str = store_item_a.to_hex().unwrap();
    let store_item_b = StoreItem::from_hex(&store_item_str).unwrap();
    
    assert_eq!(store_item_a, store_item_b)
}

#[test]
fn store_item_to_hex_fail() {
    let sk_a = SecretKey::random();
    let sk_b = SecretKey::random();
    let pk_b = sk_b.to_public();
    
    let enc_key = Key::shared(sk_a, pk_b).unwrap();
    
    let key_size = 10;
    let key = Random::bytes(key_size);
    let value_size = 1000;
    let value = Random::bytes(value_size);
    
    let store_item = StoreItem::new(enc_key, &key, &value).unwrap();
    let mut store_item_str = store_item.to_hex().unwrap();
    store_item_str.pop();
    
    let res = StoreItem::from_hex(&store_item_str);
    assert!(res.is_err())
}
