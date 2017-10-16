use libyobicash::amount::*;

#[test]
fn amount_parse_succ() {
    let n: u64 = 123456;
    let ns = format!("{}", n);
    let bn = YAmount::parse(ns.as_str()).unwrap();
    let m = bn.to_u64();
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
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn amount_big_endian_succ() {
    let bu_a = YAmount::parse("1000").unwrap();
    let bu_a_ge = bu_a.to_big_endian();
    let bu_b = YAmount::from_big_endian(bu_a_ge.as_slice());
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn amount_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YAmount::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn amount_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YAmount::from_hex(s);
    assert!(res.is_err())
}

#[test]
fn amount_to_hex_succ() {
    let bu_a = YAmount::parse("1000").unwrap();
    let bu_a_hex = bu_a.to_hex();
    let bu_b = YAmount::from_hex(bu_a_hex.as_str()).unwrap();
    assert_eq!(bu_a.to_hex(), bu_b.to_hex()) // TODO: impl debug
}

#[test]
fn amount_pow_succ() {
    let a = YAmount::from_u64(2);
    let b = YAmount::from_u64(3);
    let c = YAmount::from_u64(8);
    assert_eq!(a.pow(b).to_hex(), c.to_hex())
}

#[test]
fn amount_add_succ() {
    let a = YAmount::from_u64(1);
    let b = YAmount::from_u64(2);
    let c = YAmount::from_u64(3);
    assert_eq!(c.to_hex(), (a+b).to_hex())
}

#[test]
fn amount_sub_succ() {
    let a = YAmount::from_u64(3);
    let b = YAmount::from_u64(2);
    let c = YAmount::from_u64(1);
    assert_eq!(c.to_hex(), (a-b).to_hex())
}

#[test]
fn amount_mul_succ() {
    let a = YAmount::from_u64(2);
    let b = YAmount::from_u64(3);
    let c = YAmount::from_u64(6);
    assert_eq!(c.to_hex(), (a*b).to_hex())
}

#[test]
fn amount_div_succ() {
    let a = YAmount::from_u64(6);
    let b = YAmount::from_u64(3);
    let c = YAmount::from_u64(2);
    assert_eq!(c.to_hex(), (a/b).to_hex())
}

#[test]
fn amount_rem_succ() {
    let a = YAmount::from_u64(6);
    let b = YAmount::from_u64(4);
    let c = YAmount::from_u64(2);
    assert_eq!(c.to_hex(), (a%b).to_hex())
}
