use libyobicash::utils::version::YVersion;
use libyobicash::utils::random::YRandom;

#[test]
fn version_parse_succ() {
    let ver_str = "0.1.0";
    let res = YVersion::parse(ver_str);
    assert!(res.is_ok())
}

#[test]
fn version_parse_fail() {
    let ver_str = "a.b.c";
    let res = YVersion::parse(ver_str);
    assert!(res.is_err())
}

#[test]
fn version_to_string_succ() {
    let ver_str = "0.1.0";
    let ver = YVersion::parse(ver_str).unwrap();
    let ver_string = ver.to_string();
    assert_eq!(ver_str, ver_string.as_str())
}

#[test]
fn version_from_little_endian_succ() {
    let mut le_ver = [0u8; 24];
    YRandom::bytes_mut(&mut le_ver);
    let res = YVersion::from_little_endian(&le_ver[..]);
    assert!(res.is_ok())
}

#[test]
fn version_from_little_endian_fail() {
    let mut le_ver = [0u8; 23];
    YRandom::bytes_mut(&mut le_ver);
    let res = YVersion::from_little_endian(&le_ver[..]);
    assert!(res.is_err())
}

#[test]
fn version_to_little_endian_succ() {
    let mut le_ver_a = [0u8; 24];
    YRandom::bytes_mut(&mut le_ver_a);
    let ver = YVersion::from_little_endian(&le_ver_a[..]).unwrap();
    let le_ver_b = ver.to_little_endian().unwrap();
    assert_eq!(le_ver_a, le_ver_b)
}

#[test]
fn version_from_big_endian_succ() {
    let mut be_ver = [0u8; 24];
    YRandom::bytes_mut(&mut be_ver);
    let res = YVersion::from_big_endian(&be_ver[..]);
    assert!(res.is_ok())
}

#[test]
fn version_from_big_endian_fail() {
    let mut be_ver = [0u8; 23];
    YRandom::bytes_mut(&mut be_ver);
    let res = YVersion::from_big_endian(&be_ver[..]);
    assert!(res.is_err())
}

#[test]
fn version_to_big_endian_succ() {
    let mut be_ver_a = [0u8; 24];
    YRandom::bytes_mut(&mut be_ver_a);
    let ver = YVersion::from_big_endian(&be_ver_a[..]).unwrap();
    let be_ver_b = ver.to_big_endian().unwrap();
    assert_eq!(be_ver_a, be_ver_b)
}

#[test]
fn version_from_bytes_succ() {
    let mut bytes_ver = [0u8; 24];
    YRandom::bytes_mut(&mut bytes_ver);
    let res = YVersion::from_bytes(&bytes_ver[..]);
    assert!(res.is_ok())
}

#[test]
fn version_from_bytes_fail() {
    let mut bytes_ver = [0u8; 23];
    YRandom::bytes_mut(&mut bytes_ver);
    let res = YVersion::from_bytes(&bytes_ver[..]);
    assert!(res.is_err())
}

#[test]
fn version_to_bytes_succ() {
    let mut bytes_ver_a = [0u8; 24];
    YRandom::bytes_mut(&mut bytes_ver_a);
    let ver = YVersion::from_bytes(&bytes_ver_a[..]).unwrap();
    let bytes_ver_b = ver.to_bytes().unwrap();
    assert_eq!(bytes_ver_a, bytes_ver_b)
}
