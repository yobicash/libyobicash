use rand::random;
use serialize::hex::{FromHex, ToHex};
use libyobicash::crypto::mac::YMACCode;

fn test_vectors<'a>() -> &'a [(&'a str, &'a str)] {
  unreachable!()
}

#[test]
fn mac_from_bytes_succ() {
  let mut b = [0u8; 64];
  for i in 0..64 {
    b[i] = random::<u8>();
  }
  let res = YMACCode::from_bytes(&b[..]);
  assert!(res.is_ok())
}

#[test]
fn mac_from_bytes_fail() {
  let mut b = [0u8; 32];
  for i in 0..32 {
    b[i] = random::<u8>();
  }
  let res = YMACCode::from_bytes(&b[..]);
  assert!(res.is_err())
}

#[test]
fn mac_from_hex_succ() {
  let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e";
  let res = YMACCode::from_hex(s);
  assert!(res.is_ok())
}

#[test]
fn mac_from_hex_fail() {
  let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3";
  let res = YMACCode::from_hex(s);
  assert!(res.is_err())
}

#[test]
fn mac_test_vectors_succ() {
  assert!(true)
}
