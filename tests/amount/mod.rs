use libyobicash::amount::*;

#[test]
fn amount_parse_succ() {
    let n: u64 = 123456;
    let ns = format!("{}", n);
    let bn = YAmount::parse(ns.as_str()).unwrap();
    let m = bn.to_u64().unwrap();
    assert_eq!(n, m);
}

#[test]
fn amount_parse_fail() {
    let res = YAmount::parse("x");
    assert!(res.is_err());
}

#[test]
fn amount_little_endian_succ() {
    let bu_a = YAmount::parse("1000").unwrap();
    let bu_a_le = bu_a.to_little_endian();
    let bu_b = YAmount::from_little_endian(bu_a_le.as_slice());
    assert_eq!(bu_a, bu_b)
}

#[test]
fn amount_big_endian_succ() {
    let bu_a = YAmount::parse("1000").unwrap();
    let bu_a_ge = bu_a.to_big_endian();
    let bu_b = YAmount::from_big_endian(bu_a_ge.as_slice());
    assert_eq!(bu_a, bu_b)
}

#[test]
fn amount_hex_succ() {
    let amount_a = YAmount::parse("100000").unwrap();
    let amount_buf = amount_a.to_bytes();
    let amount_b = YAmount::from_bytes(amount_buf.as_slice());
    assert_eq!(amount_a, amount_b)
}

#[test]
fn amount_to_hex_succ() {
    let bu_a = YAmount::parse("1000").unwrap();
    let bu_a_hex = bu_a.to_hex();
    let bu_b = YAmount::from_hex(bu_a_hex.as_str()).unwrap();
    assert_eq!(bu_a, bu_b)
}

#[test]
fn amount_add_succ() {
    let a = YAmount::from_u64(1).unwrap();
    let b = YAmount::from_u64(2).unwrap();
    let c = YAmount::from_u64(3).unwrap();
    assert_eq!(c, (a+b))
}

#[test]
fn amount_sub_succ() {
    let a = YAmount::from_u64(3).unwrap();
    let b = YAmount::from_u64(2).unwrap();
    let c = YAmount::from_u64(1).unwrap();
    assert_eq!(c, (a-b))
}

#[test]
fn amount_mul_succ() {
    let a = YAmount::from_u64(2).unwrap();
    let b = YAmount::from_u64(3).unwrap();
    let c = YAmount::from_u64(6).unwrap();
    assert_eq!(c, (a*b))
}

#[test]
fn amount_div_succ() {
    let a = YAmount::from_u64(6).unwrap();
    let b = YAmount::from_u64(3).unwrap();
    let c = YAmount::from_u64(2).unwrap();
    assert_eq!(c, (a/b))
}

#[test]
fn amount_rem_succ() {
    let a = YAmount::from_u64(6).unwrap();
    let b = YAmount::from_u64(4).unwrap();
    let c = YAmount::from_u64(2).unwrap();
    assert_eq!(c, (a%b))
}
