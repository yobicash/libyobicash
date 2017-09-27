use utils::biguint::YBigUint;
use crypto::digest::YDigest;
use crypto::mac::{YMAC, YMACResult};
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;

pub struct YOutput {
  sender: YPoint,
  receiver: YPoint,
  amount: YBigUint,
  data: Vec<u8>,
  tag: YMACResult,
  custom: [u8; 32],
}
