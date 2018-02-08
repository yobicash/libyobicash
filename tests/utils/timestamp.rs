// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `timestamp` module tests.

use libyobicash::traits::{BinarySerialize, Validate};
use libyobicash::utils::timestamp::Timestamp;

#[test]
fn timestamp_from_date_succ() {
    let year = 2012;
    let month = 12;
    let day = 12;
    let hours = 12;
    let mins = 12;
    let secs = 12;

    let res = Timestamp::from_date(year, month, day,
                                   hours, mins, secs);
    assert!(res.is_ok())
}

#[test]
fn timestamp_from_date_fail() {
    let year = 2012;
    let month = 12;
    let day = 32;
    let hours = 12;
    let mins = 12;
    let secs = 12;
    
    let res = Timestamp::from_date(year, month, day,
                                   hours, mins, secs);
    assert!(res.is_err())
}

#[test]
fn timestamp_parse_succ() {
    let date = "2012-12-12T00:00:00Z";
    
    let res = Timestamp::parse(date);
    assert!(res.is_ok())
}

#[test]
fn timestamp_parse_fail() {
    let date = "2012-12-32T00:00:00Z";
    
    let res = Timestamp::parse(date);
    assert!(res.is_err())
}

#[test]
fn timestamp_to_string_succ() {
    let date = "2012-12-12T00:00:00Z";
    
    let timestamp_a = Timestamp::parse(date).unwrap();
    let timestamp_str = timestamp_a.to_string();
    let timestamp_b = Timestamp::from_string(&timestamp_str).unwrap();
    
    assert_eq!(timestamp_a, timestamp_b)
}

#[test]
fn timestamp_to_string_fail() {
    let date = "2012-12-12T00:00:00Z";
    
    let timestamp_a = Timestamp::parse(date).unwrap();
    let mut timestamp_str = timestamp_a.to_string();
    timestamp_str.pop();
    let timestamp_b = Timestamp::from_string(&timestamp_str).unwrap();
    
    assert_ne!(timestamp_a, timestamp_b)
}

#[test]
fn timestamp_to_bytes_succ() {
    let date = "2012-12-12T00:00:00Z";
    
    let timestamp_a = Timestamp::parse(date).unwrap();
    let timestamp_a_buf = timestamp_a.to_bytes().unwrap();
    let timestamp_b = Timestamp::from_bytes(&timestamp_a_buf).unwrap();
    
    assert_eq!(timestamp_a, timestamp_b)
}

#[test]
fn timestamp_to_bytes_fail() {
    let date = "2012-12-12T00:00:00Z";
    
    let timestamp_a = Timestamp::parse(date).unwrap();
    let mut timestamp_a_buf = timestamp_a.to_bytes().unwrap();
    timestamp_a_buf[0] ^= timestamp_a_buf[0];
    
    let res = Timestamp::from_bytes(&timestamp_a_buf);
    assert!(res.is_ok())
}

#[test]
fn timestamp_validate_succ() {
    let timestamp = Timestamp::now();
    
    let res = timestamp.validate();
    assert!(res.is_ok())
}

#[test]
fn timestamp_validate_fail() {
    let date = "2012-12-12T00:00:00Z";
    let timestamp = Timestamp::parse(date).unwrap();
    
    let res = timestamp.validate();
    assert!(res.is_err())
}
