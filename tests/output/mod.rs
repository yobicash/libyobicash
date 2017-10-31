use rand::random;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::*;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;

#[test]
fn output_new_succ() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let amount = YAmount::one();
    let res = YOutput::new(&sender_sk, &recipient_pk, amount, None);
    assert!(res.is_ok())
}

#[test]
fn output_new_fail() {
    let sender_g = YPoint::default();
    let sender_sk = YSecretKey::from_g(sender_g);
    let recipient_g = YPoint::random();
    let recipient_sk = YSecretKey::from_g(recipient_g);
    let recipient_pk = recipient_sk.to_public();
    let amount = YAmount::one();
    let res = YOutput::new(&sender_sk, &recipient_pk, amount, None);
    assert!(res.is_err())
}

#[test]
fn output_new_with_data_succ() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let plain = [0u8; 32];
    let res = YOutput::with_data(&sender_sk, &recipient_pk, &plain[..], None);
    assert!(res.is_ok())
}

#[test]
fn output_new_with_data_fail() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let plain = [0u8; 31];
    let res = YOutput::with_data(&sender_sk, &recipient_pk, &plain[..], None);
    assert!(res.is_err())
}

#[test]
fn output_bytes_succ() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.to_public();
    let plain = [0u8; 32];
    let output_a = YOutput::with_data(&sender_sk, &recipient_pk, &plain[..], None).unwrap();
    let output_buf = output_a.to_bytes().unwrap();
    let output_b = YOutput::from_bytes(output_buf.as_slice()).unwrap();
    assert_eq!(output_a.to_hex().unwrap(), output_b.to_hex().unwrap())
}

#[test]
fn output_bytes_fail() {
    let mut b = [0u8; 139];
    for i in 0..139 {
        b[i] = random();
    }
    let res = YOutput::from_bytes(&b[..]);
    assert!(res.is_err())
}
