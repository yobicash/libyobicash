use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::utils::time::YTime;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::utxo::YUTXO;
use libyobicash::transaction::YTransaction;
use libyobicash::utils::random::YRandom;

#[test]
fn transaction_new_succ() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output];
    let height = 1;
    let res = YTransaction::new(&utxos, &xs, &outputs, height, None);
    assert!(res.is_ok())
}

#[test]
fn transaction_new_fail() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output];
    let activation = YTime::new(1970, 1, 1, 0, 0, 0);
    let height = 1;
    let res = YTransaction::new(&utxos, &xs, &outputs, height, Some(activation));
    assert!(res.is_err())
}

#[test]
fn transaction_bytes_succ() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output];
    let height = 1;
    let tx_a = YTransaction::new(&utxos, &xs, &outputs, height, None).unwrap();
    let tx_buf = tx_a.to_bytes().unwrap();
    let tx_b = YTransaction::from_bytes(tx_buf.as_slice()).unwrap();
    assert_eq!(tx_a.to_bytes().unwrap(), tx_b.to_bytes().unwrap())
}

#[test]
fn transaction_bytes_fail() {
    let mut b = [0u8; 103];
    YRandom::bytes_mut(&mut b);
    let res = YTransaction::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn transaction_verify_input_succ() {
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
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output.clone()];
    let height = 1;
    let tx = YTransaction::new(&utxos, &xs, &outputs, height, None).unwrap();
    let mut verified = true;
    for i in 0..tx.inputs.len() {
        verified &= tx.verify_input(i as u32, &outputs[i]).unwrap();
    }
    assert!(verified)
}

#[test]
fn transaction_verify_input_fail() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output.clone()];
    let height = 1;
    let tx = YTransaction::new(&utxos, &xs, &outputs, height, None).unwrap();
    let mut verified = true;
    for i in 0..tx.inputs.len() {
        let mut output = outputs[i].clone();
        output.recipient.pk = YPoint::random();
        verified &= tx.verify_input(i as u32, &output).unwrap();
    }
    assert!(!verified)
}

#[test]
fn transaction_verify_succ() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let outputs = vec![output.clone()];
    let height = 1;
    let tx = YTransaction::new(&utxos, &xs, &outputs, height, None).unwrap();
    let verified = tx.verify(&outputs).unwrap();
    assert!(verified)
}

#[test]
fn transaction_verify_fail() {
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
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let xs = vec![recipient_sk.sk];
    let mut outputs = vec![output.clone()];
    let height = 1;
    let tx = YTransaction::new(&utxos, &xs, &outputs, height, None).unwrap();
    for i in 0..outputs.len() {
        outputs[i].recipient.pk = YPoint::random();
    }
    let verified = tx.verify(&outputs).unwrap();
    assert!(!verified)
}
