use rand::random;
use libyobicash::crypto::elliptic::scalar::*;
use libyobicash::crypto::elliptic::point::*;
use libyobicash::crypto::elliptic::keys::*;

#[test]
fn public_key_from_bytes_succ() {
    let g = YPoint::default();
    let sk = YScalar::random();
    let _pk = &g*&sk;
    let pk = YPublicKey::new(g, _pk);
    let res = YPublicKey::from_bytes(pk.to_bytes().as_slice());
    assert!(res.is_ok())
}

#[test]
fn public_key_from_bytes_fail() {
    let mut b = [0u8; 32];
    for i in 0..32 {
      b[i] = random::<u8>();
    }
    let res = YPublicKey::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn public_key_to_bytes_succ() {
    let sk = YScalar::random();
    let g = YPoint::default();
    let _pk = &g*&sk;
    let pk_a = YPublicKey::new(g, _pk);
    let pk_buf = pk_a.to_bytes();
    let pk_b = YPublicKey::from_bytes(pk_buf.as_slice()).unwrap();
    assert_eq!(pk_a, pk_b)
}

#[test]
fn public_key_hex_succ() {
    let sk = YScalar::random();
    let g = YPoint::default();
    let _pk = &g*&sk;
    let pk_a = YPublicKey::new(g, _pk);
    let pk_a_hex = pk_a.to_hex();
    let pk_b = YPublicKey::from_hex(pk_a_hex.as_str()).unwrap();
    assert_eq!(pk_a, pk_b)
}

#[test]
fn secret_key_from_bytes_succ() {
    let _sk = YScalar::random();
    let g = YPoint::default();
    let sk = YSecretKey::new(g, _sk);
    let res = YSecretKey::from_bytes(sk.to_bytes().as_slice());
    assert!(res.is_ok())
}

#[test]
fn secret_key_from_bytes_fail() {
    let mut b = [0u8; 32];
    for i in 0..32 {
      b[i] = random::<u8>();
    }
    let res = YSecretKey::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn secret_key_to_bytes_succ() {
    let _sk = YScalar::random();
    let g = YPoint::default();
    let sk_a = YSecretKey::new(g, _sk);
    let sk_buf = sk_a.to_bytes();
    let sk_b = YSecretKey::from_bytes(sk_buf.as_slice()).unwrap();
    assert_eq!(sk_a, sk_b)
}

#[test]
fn secret_key_hex_succ() {
    let sk_a = YSecretKey::random();
    let sk_a_hex = sk_a.to_hex();
    let sk_b = YSecretKey::from_hex(sk_a_hex.as_str()).unwrap();
    assert_eq!(sk_a, sk_b)
}
