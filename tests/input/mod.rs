use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::input::YInput;
use libyobicash::utils::random::Random;

#[test]
fn input_bytes_succ() {
    let mut _id = [0u8; 64];
    Random::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let prot = YSchnorrProtocol::random().to_public();
    let inp_a = YInput::new(id, idx, prot);
    let inp_buf = inp_a.to_bytes().unwrap();
    let inp_b = YInput::from_bytes(inp_buf.as_slice()).unwrap();
    assert_eq!(inp_a, inp_b)
}

#[test]
fn input_bytes_fail() {
    let mut b = [0u8; 195];
    Random::bytes_mut(&mut b);
    let res = YInput::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn input_verify_succ() {
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.to_public();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    Random::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let input = YInput::new(id, idx, public_prot);
    let verified = input.verify(&output);
    assert!(verified)
}

#[test]
fn input_verify_fail() {
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.to_public();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    Random::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let mut input = YInput::new(id, idx, public_prot);
    input.c = YScalar::random();
    let verified = input.verify(&output);
    assert!(!verified)
}
