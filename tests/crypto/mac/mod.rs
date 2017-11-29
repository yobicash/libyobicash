use serialize::hex::FromHex;
use libyobicash::crypto::key::YKey64;
use libyobicash::crypto::mac::{YMAC, YMACCode};
use libyobicash::utils::random::Random;

#[test]
fn mac_from_bytes_succ() {
    let mut b = [0u8; 64];
    Random::bytes_mut(&mut b);
    let res = YMACCode::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn mac_from_bytes_fail() {
    let mut b = [0u8; 32];
    Random::bytes_mut(&mut b);
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
    let s = "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3ef";
    let res = YMACCode::from_hex(s);
    assert!(res.is_err())
}

fn test_vectors() -> Vec<(String, String, String)> {
    vec![
    ("71b30e3f2e63828dbf750fd38ff75ae88e031b6955ac71432671230813563b9324b88dd3fc93846130f98f217d920084a2341b7d872f664b7a740dc785c2059b".to_string(),
     "627d1a6002fdcac8ef4241c5d6f8d9991eeea96687c73adee2d8c37b0e99f7901b3cb6e68d6f4de6d40cedf5bc51baff54874f9641660d492e4144e6497dbd4ff11ef3a8110c31682fb9a8389dbb2ea2718c45deabf14bd4a2ad493a7880a77194e370c6".to_string(),
     "d7fb99755b61985b589e5f8993a269806a5ee6a9ee105a654286ed92e8313d0b8e7be1ea10272c9ed701980b459a42cd0a72308a14681456213b9abf281c3086".to_string())]
}

#[test]
fn mac_test_vectors_succ() {
    for v in test_vectors() {
        let key = YKey64::from_hex(v.0.as_str()).unwrap();
        let msg = v.1.as_str().from_hex().unwrap();
        let res = YMAC::mac(key, msg.as_slice());
        let test = YMACCode::from_hex(v.2.as_str()).unwrap();
        assert_eq!(res, test)
    }
}
