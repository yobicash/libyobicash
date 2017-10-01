use utils::biguint::YBigUint;
use crypto::digest::YDigest;
use crypto::mac::{YMAC, YMACResult};
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;

pub struct YOutput {
  pub sender: YPoint,
  pub receiver: YPoint,
  pub amount: YBigUint,
  pub data: Vec<u8>,
  pub tag: YMACResult,
  pub custom: [u8; 32],
}

impl YOutput {
  pub fn new(
    sk: &YScalar,
    g: &YPoint,
    receiver: &YPoint,
    amount: YBigUint,
    custom: Option<[u8; 32]>) -> YOutput {
    unreachable!() 
  }

  pub fn verify(&self) -> bool { unreachable!() }
}
