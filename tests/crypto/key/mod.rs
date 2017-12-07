use libyobicash::crypto::key::{YKey32, YKey64};
use libyobicash::utils::random::YRandom;

#[test]
fn key32_from_bytes_succ() {
    let mut b = [0u8; 32];
    YRandom::bytes_mut(&mut b);
    let res = YKey32::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn key32_from_bytes_fail() {
    let mut b = [0u8; 64];
    YRandom::bytes_mut(&mut b);
    let res = YKey32::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn key32_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YKey32::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn key32_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YKey32::from_hex(s);
    assert!(res.is_err())
}

#[test]
fn key64_from_bytes_succ() {
    let mut b = [0u8; 64];
    YRandom::bytes_mut(&mut b);
    let res = YKey64::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn key64_from_bytes_fail() {
    let mut b = [0u8; 32];
    YRandom::bytes_mut(&mut b);
    let res = YKey64::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn key64_from_hex_succ() {
    let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
    let res = YKey64::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn key64_from_hex_fail() {
    let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3ef";
    let res = YKey64::from_hex(s);
    assert!(res.is_err())
}
