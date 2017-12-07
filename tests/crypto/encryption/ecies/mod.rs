use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::encryption::ecies::YECIES;
use libyobicash::utils::random::YRandom;

#[test]
fn ecies_shared_key_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let ecies_b = YECIES::new(sk_b);
    let key_a = ecies_a.shared_key(&pk_b).unwrap();
    let key_b = ecies_b.shared_key(&pk_a).unwrap();
    assert_eq!(key_a, key_b)
}

#[test]
fn ecies_shared_key_fail() {
    let g_a = YPoint::random();
    let sk_a = YSecretKey::from_g(g_a);
    let g_b = YPoint::random();
    let sk_b = YSecretKey::from_g(g_b);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let res = ecies_a.shared_key(&pk_b);
    assert!(res.is_err())
}

#[test]
fn ecies_encrypt_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt);
    let res = ecies_a.encrypt(&pk_b, &ptxt[..]);
    assert!(res.is_ok())
}

#[test]
fn ecies_encrypt_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt = [0u8; 17];
    YRandom::bytes_mut(&mut ptxt);
    let res = ecies_a.encrypt(&pk_b, &ptxt[..]);
    assert!(res.is_err())
}

#[test]
fn ecies_decrypt_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let ecies_b = YECIES::new(sk_b);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let ptxt_b = ecies_b.decrypt(&pk_a, cyph.as_slice()).unwrap();
    assert_eq!(&ptxt_a[..], ptxt_b.as_slice())
}

#[test]
fn ecies_decrypt_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let sk_c = YSecretKey::from_g(g);
    let ecies_c = YECIES::new(sk_c);
    let ptxt_b = ecies_c.decrypt(&pk_a, cyph.as_slice()).unwrap();
    assert_ne!(&ptxt_a[..], ptxt_b.as_slice())
}

#[test]
fn ecies_authenticate_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let res = ecies_a.authenticate(&pk_b, cyph.as_slice());
    assert!(res.is_ok())
}

#[test]
fn ecies_authenticate_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let sk_c = YSecretKey::random();
    let pk_c = sk_c.to_public();
    let res = ecies_a.authenticate(&pk_c, cyph.as_slice());
    assert!(res.is_err())
}

#[test]
fn ecies_verify_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let ecies_b = YECIES::new(sk_b);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let tag = ecies_a.authenticate(&pk_b, cyph.as_slice()).unwrap();
    let verified = ecies_b.verify(&pk_a, cyph.as_slice(), tag).unwrap();
    assert!(verified)
}

#[test]
fn ecies_verify_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let cyph = ecies_a.encrypt(&pk_b, &ptxt_a[..]).unwrap();
    let tag = ecies_a.authenticate(&pk_b, cyph.as_slice()).unwrap();
    let sk_c = YSecretKey::from_g(g);
    let ecies_c = YECIES::new(sk_c);
    let verified = ecies_c.verify(&pk_a, cyph.as_slice(), tag).unwrap();
    assert!(!verified)
}

#[test]
fn ecies_encrypt_and_authenticate_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let res = ecies_a.encrypt_and_authenticate(&pk_b, &ptxt_a[..]);
    assert!(res.is_ok())
}

#[test]
fn ecies_encrypt_and_authenticate_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let sk_c = YSecretKey::random();
    let pk_c = sk_c.to_public();
    let res = ecies_a.encrypt_and_authenticate(&pk_c, &ptxt_a[..]);
    assert!(res.is_err())
}

#[test]
fn ecies_verify_and_decrypt_succ() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let ecies_b = YECIES::new(sk_b);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let (cyph, tag) = ecies_a.encrypt_and_authenticate(&pk_b, &ptxt_a[..]).unwrap();
    let ptxt_b = ecies_b.verify_and_decrypt(&pk_a, cyph.as_slice(), tag).unwrap();
    assert_eq!(&ptxt_a[..], ptxt_b.as_slice())
}

#[test]
fn ecies_verify_and_decrypt_fail() {
    let g = YPoint::random();
    let sk_a = YSecretKey::from_g(g);
    let pk_a = sk_a.to_public();
    let sk_b = YSecretKey::from_g(g);
    let pk_b = sk_b.to_public();
    let ecies_a = YECIES::new(sk_a);
    let mut ptxt_a = [0u8; 16];
    YRandom::bytes_mut(&mut ptxt_a);
    let (cyph, tag) = ecies_a.encrypt_and_authenticate(&pk_b, &ptxt_a[..]).unwrap();
    let sk_c = YSecretKey::from_g(g);
    let ecies_c = YECIES::new(sk_c);
    let res = ecies_c.verify_and_decrypt(&pk_a, cyph.as_slice(), tag);
    assert!(res.is_err())
}
