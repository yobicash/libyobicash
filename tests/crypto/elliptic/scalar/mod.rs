use rand::random;
use libyobicash::crypto::elliptic::scalar::*;

#[test]
fn scalar_from_bytes_succ() {
    let mut b = [0u8; 32];
    for i in 0..32 {
      b[i] = random::<u8>();
    }
    let res = YScalar::from_bytes(&b[..]);
    assert!(res.is_ok())
}

#[test]
fn scalar_from_bytes_fail() {
    let mut b = [0u8; 64];
    for i in 0..64 {
      b[i] = random::<u8>();
    }
    let res = YScalar::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn scalar_to_bytes_succ() {
    let mut b = [0u8; 32];
    for i in 0..32 {
        b[i] = random::<u8>();
    }
    let scalar = YScalar::from_bytes(&b[..]).unwrap();
    let mut c = [0u8; 32];
    let scalar_buf = scalar.to_bytes();
    for i in 0..32 {
        c[i] = scalar_buf[i];
    }
    assert_eq!(b, c)
}

#[test]
fn scalar_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YScalar::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn scalar_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YScalar::from_hex(s);
    assert!(res.is_err())
}

#[test]
fn scalar_to_hex_succ() {
    let scalar_a = YScalar::random();
    let scalar_a_hex = scalar_a.to_hex();
    let scalar_b = YScalar::from_hex(scalar_a_hex.as_str()).unwrap();
    assert_eq!(scalar_a, scalar_b)
}

#[test]
fn scalar_from_biguint_succ() {
    let scalar_a = YScalar::random();
    let scalar_a_bg = scalar_a.to_biguint();
    let scalar_b = YScalar::from_biguint(&scalar_a_bg).unwrap();
    assert_eq!(scalar_a, scalar_b)
}

#[test]
fn scalar_multiply_add_succ() {
    let a = YScalar::from_u64(1).unwrap();
    let b = YScalar::from_u64(2).unwrap();
    let c = YScalar::from_u64(3).unwrap();
    let d = YScalar::from_u64(5).unwrap();
    let e = YScalar::multiply_add(&a, &b, &c);
    assert_eq!(e, d)
}

#[test]
fn scalar_add_succ() {
    let a = YScalar::from_u64(1).unwrap();
    let b = YScalar::from_u64(2).unwrap();
    let c = YScalar::from_u64(3).unwrap();
    assert_eq!(c, (&a+&b))
}

#[test]
fn scalar_sub_succ() {
    let a = YScalar::from_u64(3).unwrap();
    let b = YScalar::from_u64(2).unwrap();
    let c = YScalar::from_u64(1).unwrap();
    assert_eq!(c, (&a-&b))
}

#[test]
fn scalar_mul_succ() {
    let a = YScalar::from_u64(2).unwrap();
    let b = YScalar::from_u64(3).unwrap();
    let c = YScalar::from_u64(6).unwrap();
    assert_eq!(c, (&a*&b))
}
