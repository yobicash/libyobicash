// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash `amount` module tests.

use libyobicash::utils::amount::Amount;

#[test]
fn amount_from_string_succ() {
    let s = "10000000000000000000000000000";

    let res = Amount::from_string(s);
    assert!(res.is_ok())
}

#[test]
fn amount_from_string_fail() {
    let s = "blablabla";
    
    let res = Amount::from_string(s);
    assert!(res.is_err())
}

#[test]
fn amount_to_string_succ() {
    let n = 10.0;

    let amount_a: Amount = n.into();
    let s = amount_a.to_string();
    let amount_b = Amount::from_string(&s).unwrap();
    
    assert_eq!(amount_a, amount_b)
}

#[test]
fn amount_to_string_fail() {
    let n = 10.0;
    
    let amount_a: Amount = n.into();
    let mut s = amount_a.to_string();
    s.pop();
    let amount_b = Amount::from_string(&s).unwrap();
    
    assert_ne!(amount_a, amount_b)
}

#[test]
fn amount_add_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("17/3").unwrap();
    
    let sum = a + b;
    
    assert_eq!(sum, c)
}

#[test]
fn amount_add_assign_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("17/3").unwrap();
    
    let mut sum = a;
    sum += b;
    
    assert_eq!(sum, c)
}

#[test]
fn amount_sub_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("1").unwrap();
    
    let sub = a - b;
    
    assert_eq!(sub, c)
}

#[test]
fn amount_sub_assign_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("1").unwrap();
    
    let mut sub = a;
    sub -= b;
    
    assert_eq!(sub, c)
}

#[test]
fn amount_mul_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("70/9").unwrap();
    
    let sum = a * b;
    
    assert_eq!(sum, c)
}

#[test]
fn amount_mul_assign_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("7/3").unwrap();
    let c = Amount::from_string("70/9").unwrap();
    
    let mut sum = a;
    sum *= b;
    
    assert_eq!(sum, c)
}

#[test]
fn amount_div_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("10/3").unwrap();
    let c = Amount::from_string("1").unwrap();
    
    let div = a / b;
    
    assert_eq!(div, c)
}

#[test]
fn amount_div_assign_succ() {
    let a = Amount::from_string("10/3").unwrap();
    let b = Amount::from_string("10/3").unwrap();
    let c = Amount::from_string("1").unwrap();
    
    let mut div = a;
    div /= b;
    
    assert_eq!(div, c)
}
