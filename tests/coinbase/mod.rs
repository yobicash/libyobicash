use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::coinbase::YCoinbase;
use libyobicash::utils::random::YRandom;

#[test]
fn coinbase_new_succ() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.to_public();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let outputs = vec![output];
    let res = YCoinbase::new(&outputs);
    assert!(res.is_ok())
}

#[test]
fn coinbase_new_fail() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.to_public();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let outputs = vec![output.clone(), output];
    let res = YCoinbase::new(&outputs);
    assert!(res.is_err())
}

#[test]
fn coinbase_bytes_succ() {
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let secret_prot = YSchnorrProtocol::random();
    let public_prot = secret_prot.to_public();
    let g = public_prot.g;
    let recipient_sk = YSecretKey::new(g, secret_prot.x);
    let recipient_pk = recipient_sk.to_public();
    let sender_sk = YSecretKey::from_g(g);
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let outputs = vec![output];
    let cb_a = YCoinbase::new(&outputs).unwrap();
    let cb_buf = cb_a.to_bytes().unwrap();
    let cb_b = YCoinbase::from_bytes(cb_buf.as_slice()).unwrap();
    assert_eq!(cb_a.to_bytes().unwrap(), cb_b.to_bytes().unwrap())
}

#[test]
fn coinbase_bytes_fail() {
    let mut b = [0u8; 103];
    YRandom::bytes_mut(&mut b);
    let res = YCoinbase::from_bytes(&b[..]);
    assert!(res.is_err())
}
