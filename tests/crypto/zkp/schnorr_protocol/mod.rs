use libyobicash::crypto::elliptic::scalar::YScalar;
use libyobicash::crypto::zkp::schnorr_protocol::*;

#[test]
fn schnorr_protocol_verify_succ() {
    let protocol = YSchnorrProtocol::random();
    let public_coin = protocol.to_public();
    let verified = public_coin.verify();
    assert!(verified)
}

#[test]
fn schnorr_protocol_verify_fail() {
    let protocol = YSchnorrProtocol::random();
    let mut public_coin = protocol.to_public();
    public_coin.c = YScalar::random();
    let verified = public_coin.verify();
    assert!(!verified)
}
