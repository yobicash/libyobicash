use ::VERSION;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::digest::YDigest;
use input::{YPartialInput, YInput};
use output::YOutput;

#[derive(Clone)]
pub struct YPartialTransaction {
  pub inputs: Vec<YPartialInput>,
  pub outputs: Vec<YOutput>,
}

impl YPartialTransaction {
  pub fn new(inputs: Vec<YPartialInput>, outputs: Vec<YOutput>) -> Option<YPartialTransaction> {
    // TODO: check unique partial inputs
    // TODO: check unique outputs
    Some(YPartialTransaction {
      inputs: inputs,
      outputs: outputs,
    })
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YPartialTransaction> { unreachable!() }

  pub fn complete(self) -> Option<YTransaction> {
    unreachable!()
  }
}

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
  pub fn new(inputs: Vec<YInput>, outputs: Vec<YOutput>) -> Option<YTransaction> {
    // TODO: check unique inputs
    // TODO: check unique outputs
    let inputs_len = inputs.len() as u32;
    let outputs_len = outputs.len() as u32;
    let now = YTime::now();
    let version = YVersion::parse(VERSION).unwrap();
    let id = YDigest::default();
    let tx = YTransaction {
      id: id,
      version: version,
      time: now,
      inputs_len: inputs_len,
      inputs: inputs,
      outputs_len: outputs_len,
      outputs: outputs,
    };
    // TODO: id
    // TODO: check inputs `c`
    Some(tx)
  }

  pub fn from_partial(p: YPartialTransaction) -> Option<YTransaction> {
    p.complete()
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YTransaction> { unreachable!() }

  pub fn verify_input(&self, idx: u32, output: &YOutput) -> Option<bool> {
    if self.inputs_len - 1 < idx {
      return None;
    }
    Some(self.inputs[idx as usize].verify(output))
  }

  pub fn verify(&self, outputs: Vec<YOutput>) -> Option<bool> {
    let len = self.inputs_len;
    if outputs.len() as u32 != len {
      return None;
    }
    for idx in 0..len {
      if let Some(verified) = self.verify_input(idx, &outputs[idx as usize]) {
        if !verified {
          return Some(false)
        }
      } else {
        return None;
      }
    }
    Some(true)
  }
}
