//use ::VERSION;
//use utils::version::YVersion;
//use utils::time::YTime;
//use crypto::digest::YDigest;
use input::YPartialInput;
use output::YOutput;

pub struct YPartialTransaction {
  pub inputs: Vec<YPartialInput>,
  pub outputs: Vec<YOutput>,
}

impl YPartialTransaction {
  pub fn new(inputs: Vec<YPartialInput>, outputs: Vec<YOutput>) -> YPartialTransaction {
    unreachable!()
  }

  pub fn verify_input(&self, idx: usize, output: &Vec<YOutput>) -> bool {
    unreachable!()
  }

  pub fn verify(&self, outputs: Vec<YOutput>) -> bool {
    unreachable!()
  }
}
