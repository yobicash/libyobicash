use utils::time::YTime;
use utils::version::YVersion;
use utils::biguint::YBigUint;
use crypto::digest::YDigest;
use crypto::mac::{YMAC, YMACResult};
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use input::YInput;
use output::YOutput;

pub struct YTransaction {
  id: YDigest,
  version: YVersion,
  time: YTime,
  inputs_len: u32,
  inputs: Vec<YInput>,
  outputs_len: u32,
  outputs: Vec<YOutput>,
}

impl YTransaction {
  pub fn new(inputs: Vec<YInput>, outputs: Vec<YOutput>) -> YTransaction {
    unreachable!()
  }

  pub fn verify(&self) -> bool { unreachable!() }
}
