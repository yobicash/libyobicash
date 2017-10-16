use rand::random;
use libyobicash::crypto::elliptic::point::*;

#[test]
fn point_from_bytes_succ() {
    let p = YPoint::random().to_bytes();
    let res = YPoint::from_bytes(p.as_slice());
    assert!(res.is_ok())
}

#[test]
fn point_from_bytes_fail() {
    let mut b = [0u8; 64];
    for i in 0..64 {
      b[i] = random::<u8>();
    }
    let res = YPoint::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn point_to_bytes_succ() {
    let p_a = YPoint::random();
    let p_buf = p_a.to_bytes();
    let p_b = YPoint::from_bytes(p_buf.as_slice()).unwrap();
    assert_eq!(p_a, p_b)
}

#[test]
fn point_from_hex_succ() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b796974131";
    let res = YPoint::from_hex(s);
    assert!(res.is_ok())
}

#[test]
fn point_from_hex_fail() {
    let s = "df36e1c444a5986aaa9cb0e7352617425eb439274dfb49d794df78b79697413";
    let res = YPoint::from_hex(s);
    assert!(res.is_err())
}

#[test]
fn point_to_hex_succ() {
    let point_a = YPoint::random();
    let point_a_hex = point_a.to_hex();
    let point_b = YPoint::from_hex(point_a_hex.as_str()).unwrap();
    assert_eq!(point_a, point_b)
}

#[test]
fn point_fields_succ() {
    let p_a = YPoint::random();
    let x = p_a.x_field();
    let y = p_a.y_field();
    let p_b = YPoint::from_fields(x.as_slice(), y.as_slice()).unwrap();
    assert_eq!(p_a, p_b)
}
