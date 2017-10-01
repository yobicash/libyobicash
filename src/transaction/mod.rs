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
  pub id: YDigest,
  pub version: YVersion,
  pub time: YTime,
  pub inputs_len: u32,
  pub inputs: Vec<YInput>,
  pub outputs_len: u32,
  pub outputs: Vec<YOutput>,
}

impl YTransaction {
  pub fn new(inputs: Vec<YInput>, outputs: Vec<YOutput>) -> YTransaction {
    unreachable!()
  }

  pub fn verify(&self) -> bool { unreachable!() }
}
