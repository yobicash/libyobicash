use libyobicash::crypto::elliptic::scalar::*;
use libyobicash::crypto::elliptic::point::*;
use libyobicash::utils::random::Random;

#[test]
fn point_from_bytes_succ() {
    let p = YPoint::random().to_bytes();
    let res = YPoint::from_bytes(p.as_slice());
    assert!(res.is_ok())
}

#[test]
fn point_from_bytes_fail() {
    let mut b = [0u8; 64];
    Random::bytes_mut(&mut b);
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

#[test]
fn diffie_hellman_succ() {
    let g = YPoint::default();
    let sk_1 = YScalar::random();
    let pk_1 = &g*&sk_1;
    let sk_2 = YScalar::random();
    let pk_2 = &g*&sk_2;
    let dh_1 = diffie_hellman(&sk_1, &pk_2);
    let dh_2 = diffie_hellman(&sk_2, &pk_1);
    assert_eq!(dh_1, dh_2)
}

#[test]
fn diffie_hellman_fail() {
    let g_1 = YPoint::default();
    let sk_1 = YScalar::random();
    let pk_1 = &g_1*&sk_1;
    let sk_g_2 = YScalar::random();
    let g_2 = &g_1*&sk_g_2;
    let sk_2 = YScalar::random();
    let pk_2 = &g_2*&sk_2;
    let dh_1 = diffie_hellman(&sk_1, &pk_2);
    let dh_2 = diffie_hellman(&sk_2, &pk_1);
    assert_ne!(dh_1, dh_2)
}
