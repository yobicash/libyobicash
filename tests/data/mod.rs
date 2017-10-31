use rand::random;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::data::YData;

#[test]
fn data_new_succ() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain = [0u8; 32];
    for i in 0..32 {
        plain[i] = random();
    }
    let res = YData::new(&sk_a, &pk_b, &plain[..]);
    assert!(res.is_ok())
}

#[test]
fn data_new_fail() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain = [0u8; 31];
    for i in 0..31 {
        plain[i] = random();
    }
    let res = YData::new(&sk_a, &pk_b, &plain[..]);
    assert!(res.is_err())
}

#[test]
fn data_bytes_succ() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain = [0u8; 32];
    for i in 0..32 {
        plain[i] = random();
    }
    let data_a = YData::new(&sk_a, &pk_b, &plain[..]).unwrap();
    let data_buf = data_a.to_bytes().unwrap();
    let data_b = YData::from_bytes(data_buf.as_slice()).unwrap();
    assert_eq!(data_a, data_b)
}

#[test]
fn data_bytes_fail() {
    let mut b = [0u8; 99];
    for i in 0..99 {
        b[i] = random();
    }
    let res = YData::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn data_verify_succ() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain = [0u8; 32];
    for i in 0..32 {
        plain[i] = random();
    }
    let data = YData::new(&sk_a, &pk_b, &plain[..]).unwrap();
    let verified = data.verify(&sk_a, &pk_b).unwrap();
    assert!(verified)
}

#[test]
fn data_verify_fail() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain = [0u8; 32];
    for i in 0..32 {
        plain[i] = random();
    }
    let data = YData::new(&sk_a, &pk_b, &plain[..]).unwrap();
    let sk_c = YSecretKey::from_g(g);
    let verified = data.verify(&sk_c, &pk_b).unwrap();
    assert!(!verified)
}

#[test]
fn data_verify_and_decrypt_succ() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain_a = [0u8; 32];
    for i in 0..32 {
        plain_a[i] = random();
    }
    let data = YData::new(&sk_a, &pk_b, &plain_a[..]).unwrap();
    let plain_b = data.verify_and_decrypt(&sk_a, &pk_b).unwrap();
    assert_eq!(&plain_a[..], plain_b.as_slice())
}

#[test]
fn data_verify_and_decrypt_fail() {
    let g = YPoint::default();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let mut plain_a = [0u8; 32];
    for i in 0..32 {
        plain_a[i] = random();
    }
    let data = YData::new(&sk_a, &pk_b, &plain_a[..]).unwrap();
    let sk_c = YSecretKey::from_g(g);
    let res = data.verify_and_decrypt(&sk_c, &pk_b);
    assert!(res.is_err())
}
