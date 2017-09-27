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
