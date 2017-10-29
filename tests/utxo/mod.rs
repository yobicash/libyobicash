use rand::random;
use libyobicash::crypto::hash::YDigest64;
use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::utxo::YUTXO;

#[test]
fn utxo_new_succ() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.public_key();
    let amount = YAmount::one();
    let res = YUTXO::new(id, idx, height, recipient, amount);
    assert!(res.is_ok())
}

#[test]
fn utxo_new_fail() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.public_key();
    let amount = YAmount::one();
    let res = YUTXO::new(id, idx, height, recipient, amount);
    assert!(res.is_err())
}

#[test]
fn utxo_from_output_succ() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.public_key();
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let res = YUTXO::from_output(&output, id, idx, height);
    assert!(res.is_ok())
}

#[test]
fn utxo_from_output_fail() {
    let g = YPoint::default();
    let sender_sk = YSecretKey::from_g(g);
    let recipient_sk = YSecretKey::from_g(g);
    let recipient_pk = recipient_sk.public_key();
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let res = YUTXO::from_output(&output, id, idx, height);
    assert!(res.is_err())
}

#[test]
fn utxo_to_input_succ() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.public_key();
    let amount = YAmount::one();
    let utxo = YUTXO::new(id, idx, height, recipient, amount).unwrap();
    let u = YScalar::random();
    let c = YScalar::random();
    let res = utxo.to_input(secret.sk, u, c);
    assert!(res.is_ok())
}

#[test]
fn utxo_to_input_fail() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.public_key();
    let amount = YAmount::one();
    let utxo = YUTXO::new(id, idx, height, recipient, amount).unwrap();
    let false_x = YScalar::random();
    let u = YScalar::random();
    let c = YScalar::random();
    let res = utxo.to_input(false_x, u, c);
    assert!(res.is_err())
}

#[test]
fn utxo_bytes_succ() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let g = YPoint::default();
    let secret = YSecretKey::from_g(g);
    let recipient = secret.public_key();
    let amount = YAmount::one();
    let utxo_a = YUTXO::new(id, idx, height, recipient, amount).unwrap();
    let utxo_buf = utxo_a.to_bytes().unwrap();
    let utxo_b = YUTXO::from_bytes(utxo_buf.as_slice()).unwrap();
    assert_eq!(utxo_a.to_bytes().unwrap(), utxo_b.to_bytes().unwrap())
}

#[test]
fn utxo_bytes_fail() {
    let mut b = [0u8; 143];
    for i in 0..143 {
        b[i] = random();
    }
    let res = YUTXO::from_bytes(&b[..]);
    assert!(res.is_err())
}
