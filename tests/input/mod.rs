use rand::random;
use libyobicash::crypto::hash::YDigest64;
use libyobicash::crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use libyobicash::input::YInput;

#[test]
fn input_new_succ() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let prot = YSchnorrProtocol::random().to_public();
    let res = YInput::new(id, idx, height, prot);
    assert!(res.is_ok())
}

#[test]
fn input_new_fail() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 0;
    let prot = YSchnorrProtocol::random().to_public();
    let res = YInput::new(id, idx, height, prot);
    assert!(res.is_err())
}

#[test]
fn input_bytes_succ() {
    let mut _id = [0u8; 64];
    for i in 0..64 {
        _id[i] = random();
    }
    let id = YDigest64::from_bytes(&_id[..]).unwrap();
    let idx = 0;
    let height = 1;
    let prot = YSchnorrProtocol::random().to_public();
    let inp_a = YInput::new(id, idx, height, prot).unwrap();
    let inp_buf = inp_a.to_bytes().unwrap();
    let inp_b = YInput::from_bytes(inp_buf.as_slice()).unwrap();
    assert_eq!(inp_a, inp_b)
}

#[test]
fn input_bytes_fail() {
    let mut b = [0u8; 203];
    for i in 0..203 {
        b[i] = random();
    }
    let res = YInput::from_bytes(&b[..]);
    assert!(res.is_err())
}

#[test]
fn input_verify_succ() {
    // TODO
}

#[test]
fn input_verify_fail() {
    // TODO
}

#[test]
fn input_verify_and_decrypt_succ() {
    // TODO
}

#[test]
fn input_verify_and_decrypt_fail() {
    // TODO
}
