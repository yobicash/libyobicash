use libyobicash::utils::random::YRandom;
use libyobicash::crypto::hash::digest::YDigest64;
use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::elliptic::point::YPoint;
use libyobicash::crypto::elliptic::keys::YSecretKey;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::amount::YAmount;
use libyobicash::output::YOutput;
use libyobicash::utxo::YUTXO;
use libyobicash::transaction::YTransaction;

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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let outputs = vec![output];
    let res = YTransaction::new(&utxos, &sks, &outputs);
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
    let height = 1;
    let amount = YAmount::one();
    let output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    let outputs = vec![output];
    let res = YTransaction::new(&utxos, &sks, &outputs);
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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let outputs = vec![output];
    let tx_a = YTransaction::new(&utxos, &sks, &outputs).unwrap();
    let tx_buf = tx_a.to_bytes().unwrap();
    let tx_b = YTransaction::from_bytes(tx_buf.as_slice()).unwrap();
    assert_eq!(tx_a.to_bytes().unwrap(), tx_b.to_bytes().unwrap())
}

#[test]
fn transaction_bytes_fail() {
    let mut b = [0u8; 95];
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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let mut outputs = vec![output.clone()];
    let tx = YTransaction::new(&utxos, &sks, &outputs).unwrap();
    let mut verified = true;
    for i in 0..tx.inputs.len() {
        outputs[i].height -= 1;
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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let outputs = vec![output.clone()];
    let tx = YTransaction::new(&utxos, &sks, &outputs).unwrap();
    let mut verified = true;
    for i in 0..tx.inputs.len() {
        let mut output = outputs[i].clone();
        output.height -= 1;
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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let mut outputs = vec![output.clone()];
    let tx = YTransaction::new(&utxos, &sks, &outputs).unwrap();
    outputs[0].height -= 1;
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
    let height = 1;
    let amount = YAmount::one();
    let mut output = YOutput::new(&sender_sk, &recipient_pk, height, amount, None).unwrap();
    let mut _id = [0u8; 64];
    YRandom::bytes_mut(&mut _id);
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let utxo = YUTXO::from_output(&output, id, idx);
    let utxos = vec![utxo];
    let sks = vec![recipient_sk];
    output.height += 1;
    let mut outputs = vec![output.clone()];
    let tx = YTransaction::new(&utxos, &sks, &outputs).unwrap();
    for i in 0..outputs.len() {
        outputs[i].height -= 1;
        outputs[i].recipient.pk = YPoint::random();
    }
    let verified = tx.verify(&outputs).unwrap();
    assert!(!verified)
}

#[test]
fn transaction_new_coins_succ() {
    let g = YPoint::default();
    let main_sk = YSecretKey::from_g(g);
    let main_pk = main_sk.to_public();
    let change_sk = YSecretKey::from_g(g);
    let change_pk = change_sk.to_public();
    let amount = YAmount::from_u64(1000).unwrap();
    let utxo_x = YScalar::random();
    let sks = vec![main_sk];
    let utxo_sk = YSecretKey::new(g, utxo_x);
    let utxo_tx_id = YDigest64::from_bytes(&YRandom::bytes(64)).unwrap();
    let utxo_amount = YAmount::from_u64(10000).unwrap();
    let utxo_output = YOutput::new(&utxo_sk, &main_pk, 0, utxo_amount, None).unwrap();
    let utxo = YUTXO::from_output(&utxo_output, utxo_tx_id, 0);
    let utxos = vec![utxo];
    let res = YTransaction::new_coins(&main_sk, &change_sk,
                                      &main_pk, &change_pk,
                                      amount, &utxos, &sks,
                                      None);
    assert!(res.is_ok())
}

#[test]
fn transaction_new_coins_fail() {
    let g = YPoint::default();
    let main_sk = YSecretKey::from_g(g);
    let main_pk = main_sk.to_public();
    let change_sk = YSecretKey::from_g(g);
    let change_pk = change_sk.to_public();
    let amount = YAmount::from_u64(1000).unwrap();
    let utxo_x = YScalar::random();
    let sks = vec![main_sk];
    let utxo_sk = YSecretKey::new(g, utxo_x);
    let utxo_tx_id = YDigest64::from_bytes(&YRandom::bytes(64)).unwrap();
    let utxo_amount = YAmount::from_u64(100).unwrap();
    let utxo_output = YOutput::new(&utxo_sk, &main_pk, 0, utxo_amount, None).unwrap();
    let utxo = YUTXO::from_output(&utxo_output, utxo_tx_id, 0);
    let utxos = vec![utxo];
    let res = YTransaction::new_coins(&main_sk, &change_sk,
                                      &main_pk, &change_pk,
                                      amount, &utxos, &sks,
                                      None);
    assert!(res.is_err())
}

#[test]
fn transaction_new_data_succ() {
    let g = YPoint::default();
    let main_sk = YSecretKey::from_g(g);
    let main_pk = main_sk.to_public();
    let change_sk = YSecretKey::from_g(g);
    let change_pk = change_sk.to_public();
    let data_buf = YRandom::bytes(1000);
    let utxo_x = YScalar::random();
    let sks = vec![main_sk];
    let utxo_sk = YSecretKey::new(g, utxo_x);
    let utxo_tx_id = YDigest64::from_bytes(&YRandom::bytes(64)).unwrap();
    let utxo_amount = YAmount::from_u64(10000).unwrap();
    let utxo_output = YOutput::new(&utxo_sk, &main_pk, 0, utxo_amount, None).unwrap();
    let utxo = YUTXO::from_output(&utxo_output, utxo_tx_id, 0);
    let utxos = vec![utxo];
    let res = YTransaction::new_data(&main_sk, &change_sk,
                                     &main_pk, &change_pk,
                                     &data_buf, &utxos, &sks,
                                     None);
    assert!(res.is_ok())
}

#[test]
fn transaction_new_data_fail() {
    let g = YPoint::default();
    let main_sk = YSecretKey::from_g(g);
    let main_pk = main_sk.to_public();
    let change_sk = YSecretKey::from_g(g);
    let change_pk = change_sk.to_public();
    let data_buf = YRandom::bytes(1000);
    let utxo_x = YScalar::random();
    let sks = vec![main_sk];
    let utxo_sk = YSecretKey::new(g, utxo_x);
    let utxo_tx_id = YDigest64::from_bytes(&YRandom::bytes(64)).unwrap();
    let utxo_amount = YAmount::from_u64(100).unwrap();
    let utxo_output = YOutput::new(&utxo_sk, &main_pk, 0, utxo_amount, None).unwrap();
    let utxo = YUTXO::from_output(&utxo_output, utxo_tx_id, 0);
    let utxos = vec![utxo];
    let res = YTransaction::new_data(&main_sk, &change_sk,
                                     &main_pk, &change_pk,
                                     &data_buf, &utxos, &sks,
                                     None);
    assert!(res.is_err())
}

#[test]
fn transaction_genesys_succ() {
    let tx = YTransaction::new_genesys().unwrap();
    let res = tx.check();
    assert!(res.is_ok())
}
