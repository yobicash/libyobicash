use rand::random;
use libyobicash::crypto::hash::YDigest64;
use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::amount::YAmount;
use libyobicash::input::YInput;
use libyobicash::output::YOutput;
use libyobicash::transaction::YTransaction;

#[test]
fn transaction_new_succ() {
    /*
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let prot = YSchnorrProtocol::random().to_public();
    let input = YInput::new(id, idx, height, prot).unwrap();
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.public_key();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let res = YTransaction::new(vec![input], vec![output]);
    assert!(res.is_ok())
    */
}

#[test]
fn transaction_new_fail() {}

#[test]
fn transaction_bytes_succ() {}

#[test]
fn transaction_bytes_fail() {}

#[test]
fn transaction_verify_input_succ() {}

#[test]
fn transaction_verify_input_fail() {}

#[test]
fn transaction_verify_succ() {}

#[test]
fn transaction_verify_fail() {}
