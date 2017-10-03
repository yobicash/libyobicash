use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
use utils::version::YVersion;
use utils::time::YTime;
use crypto::digest::YDigest;
use crypto::hash::YHash;
use crypto::elliptic::scalar::YScalar;
use input::{YPartialInput, YInput};
use output::YOutput;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Default)]
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

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let inputs = self.inputs.clone();
    let inputs_len = inputs.len();
    match buf.write_u32::<BigEndian>(inputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..inputs_len {
      if let Some(input_buf) = inputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(input_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(input_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    let outputs = self.outputs.clone();
    let outputs_len = outputs.len();
    match buf.write_u32::<BigEndian>(outputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..outputs_len {
      if let Some(output_buf) = outputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(output_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(output_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    Some(buf)
  }

  pub fn from_bytes(b: &[u8]) -> Option<YPartialTransaction> {
    if b.len() < 8 {
      return None;
    }
    
    let mut ptx = YPartialTransaction::default();

    let inputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..inputs_len {
      let input_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      if let Some(input) = YPartialInput::from_bytes(&b[i+8..i+8+input_len]) {
        ptx.inputs.push(input);      
      }
    }

    let outputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..outputs_len {
      let output_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      if let Some(output) = YOutput::from_bytes(&b[i+8..i+8+output_len]) {
        ptx.outputs.push(output);      
      }
    }

    Some(ptx)
  }

  pub fn complete(self) -> Option<YTransaction> {
    unreachable!()
  }
}

#[derive(Clone, Eq, PartialEq, Default)]
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
    let version = YVersion::default();
    let id = YDigest::default();
    let mut tx = YTransaction {
      id: id,
      version: version,
      time: now,
      inputs_len: inputs_len,
      inputs: inputs.clone(),
      outputs_len: outputs_len,
      outputs: outputs,
    };
    for i in 0..inputs_len as usize {
      if let Some(_c) = tx.calc_challenge(i as u32) {
        if inputs[i].c != _c {
          return None;
        }  
      } else {
        return None;
      }
    }
    if let Some(_id) = tx.calc_id() {
      tx.id = _id;
    } else {
      return None;
    }
    Some(tx)
  }

  pub fn from_partial(p: YPartialTransaction) -> Option<YTransaction> {
    p.complete()
  }

  pub fn calc_challenge(&self, idx: u32) -> Option<YScalar> {
    let mut tx_copy = self.clone();
    // NB: case where the tx is quite complete but
    // a) the id is the default id
    // b) the idx input is substituted by a default YInput
    // c) all the non-idx inputs' challenges are the default challenge YScalar(0)
    tx_copy.id = YDigest::default();
    for i in 0..tx_copy.inputs.len() {
      if i == idx as usize {
        tx_copy.inputs[i] = YInput::default();
      } else {
        tx_copy.inputs[i].c = YScalar::default();
      }
    }
    if let Some(buf) = tx_copy.to_bytes() {
      Some(YScalar::hash_from_bytes(buf.as_slice()))
    } else {
      None
    }
  }

  pub fn calc_id(&self) -> Option<YDigest> {
    let mut buf = Vec::new();
    if let Some(version_buf) = self.version.to_bytes() {
      match buf.write(&version_buf[..]) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
    } else {
      return None;
    }
    match buf.write(&self.time.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    let inputs = self.inputs.clone();
    let inputs_len = inputs.len();
    match buf.write_u32::<BigEndian>(inputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..inputs_len {
      if let Some(input_buf) = inputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(input_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(input_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    let outputs = self.outputs.clone();
    let outputs_len = outputs.len();
    match buf.write_u32::<BigEndian>(outputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..outputs_len {
      if let Some(output_buf) = outputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(output_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(output_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    Some(YHash::hash(buf.as_slice())) 
  }

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    match buf.write(&self.id.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    if let Some(version_buf) = self.version.to_bytes() {
      match buf.write(&version_buf[..]) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
    } else {
      return None;
    }
    match buf.write(&self.time.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    let inputs = self.inputs.clone();
    let inputs_len = inputs.len();
    match buf.write_u32::<BigEndian>(inputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..inputs_len {
      if let Some(input_buf) = inputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(input_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(input_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    let outputs = self.outputs.clone();
    let outputs_len = outputs.len();
    match buf.write_u32::<BigEndian>(outputs_len as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    for i in 0..outputs_len {
      if let Some(output_buf) = outputs[i].to_bytes() {
        match buf.write_u32::<BigEndian>(output_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(output_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; }
        }
      }
    }
    Some(buf)
  }

  pub fn from_bytes(b: &[u8]) -> Option<YTransaction> {
    if b.len() < 104 {
      return None;
    }
    
    let mut tx = YTransaction::default();

    if let Some(_id) = YDigest::from_bytes(&b[0..64]) {
      tx.id = _id;
    } else {
      return None;
    }

    if let Some(_version) = YVersion::from_bytes(&b[64..88]) {
      tx.version = _version;
    } else {
      return None;
    }

    tx.time = YTime::from_bytes(&b[88..96]);

    let inputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..inputs_len {
      let input_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      if let Some(input) = YInput::from_bytes(&b[i+8..i+8+input_len]) {
        tx.inputs.push(input);      
      }
    }

    let outputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..outputs_len {
      let output_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      if let Some(output) = YOutput::from_bytes(&b[i+8..i+8+output_len]) {
        tx.outputs.push(output);      
      }
    }

    for i in 0..inputs_len as usize {
      if let Some(_c) = tx.calc_challenge(i as u32) {
        if tx.inputs[i].c != _c {
          return None;
        }  
      } else {
        return None;
      }
    }
    if let Some(_id) = tx.calc_id() {
      tx.id = _id;
    } else {
      return None;
    }

    Some(tx)
  }

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
