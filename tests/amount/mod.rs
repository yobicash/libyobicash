use libyobicash::utils::biguint::*;

#[test]
fn biguint_parse_succ() {
    let n: u64 = 123456;
    let ns = format!("{}", n);
    let bn = YBigUint::parse(ns.as_str()).unwrap();
    let m = bn.to_u64();
    assert_eq!(n, m);
}

#[test]
fn biguint_parse_fail() {
    let res = YBigUint::parse("x");
    assert!(res.is_err());
}

#[test]
fn biguint_little_endian_succ() {
    let bu_a = YBigUint::parse("1000").unwrap();
    let bu_a_le = bu_a.to_little_endian();
    let bu_b = YBigUint::from_little_endian(bu_a_le.as_slice());
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn biguint_big_endian_succ() {
    let bu_a = YBigUint::parse("1000").unwrap();
    let bu_a_ge = bu_a.to_big_endian();
    let bu_b = YBigUint::from_big_endian(bu_a_ge.as_slice());
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn biguint_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YBigUint::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn biguint_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YBigUint::from_hex(s);
    assert!(res.is_err())
}

#[test]
fn biguint_to_hex_succ() {
    let bu_a = YBigUint::parse("1000").unwrap();
    let bu_a_hex = bu_a.to_hex();
    let bu_b = YBigUint::from_hex(bu_a_hex.as_str()).unwrap();
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn biguint_pow_succ() {
    let a = YBigUint::from_u64(2);
    let b = YBigUint::from_u64(3);
    let c = YBigUint::from_u64(8);
    assert_eq!(a.pow(b).to_hex(), c.to_hex())
}

#[test]
fn biguint_add_succ() {
    let a = YBigUint::from_u64(1);
    let b = YBigUint::from_u64(2);
    let c = YBigUint::from_u64(3);
    assert_eq!(c.to_hex(), (a+b).to_hex())
}

#[test]
fn biguint_sub_succ() {
    let a = YBigUint::from_u64(3);
    let b = YBigUint::from_u64(2);
    let c = YBigUint::from_u64(1);
    assert_eq!(c.to_hex(), (a-b).to_hex())
}

#[test]
fn biguint_mul_succ() {
    let a = YBigUint::from_u64(2);
    let b = YBigUint::from_u64(3);
    let c = YBigUint::from_u64(6);
    assert_eq!(c.to_hex(), (a*b).to_hex())
}

#[test]
fn biguint_div_succ() {
    let a = YBigUint::from_u64(6);
    let b = YBigUint::from_u64(3);
    let c = YBigUint::from_u64(2);
    assert_eq!(c.to_hex(), (a/b).to_hex())
}

#[test]
fn biguint_rem_succ() {
    let a = YBigUint::from_u64(6);
    let b = YBigUint::from_u64(4);
    let c = YBigUint::from_u64(2);
    assert_eq!(c.to_hex(), (a%b).to_hex())
}
