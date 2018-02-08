// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `version` module tests.

use libyobicash::constants::VERSION;
use libyobicash::traits::{BinarySerialize, Validate};
use libyobicash::utils::version::Version;

#[test]
fn version_parse_succ() {
    let ver_str = VERSION;

    let res = Version::parse(ver_str);
    assert!(res.is_ok())
}

#[test]
fn version_parse_fail() {
    let ver_str = "0.1.0-blablabla-blablabla";
    
    let res = Version::parse(ver_str);
    assert!(res.is_err())
}

#[test]
fn version_to_bytes_succ() {
    let ver_str = VERSION;
    
    let ver_a = Version::parse(ver_str).unwrap();
    let ver_a_buf = ver_a.to_bytes().unwrap();
    let ver_b = Version::from_bytes(&ver_a_buf).unwrap();
    
    assert_eq!(ver_a, ver_b)
}

#[test]
fn version_to_bytes_fail() {
    let ver_str = VERSION;
    
    let ver_a = Version::parse(ver_str).unwrap();
    let mut ver_a_buf = ver_a.to_bytes().unwrap();
    ver_a_buf[0] ^= ver_a_buf[0];
    
    let res = Version::from_bytes(&ver_a_buf);
    assert!(res.is_err())
}

#[test]
fn version_validate_succ() {
    let ver = Version::default();
    
    let res = ver.validate();
    assert!(res.is_ok())
}

#[test]
fn version_validate_fail() {
    let ver_str = "0.0.0";
    
    let ver = Version::parse(ver_str).unwrap();
    
    let res = ver.validate();
    assert!(res.is_err())
}
