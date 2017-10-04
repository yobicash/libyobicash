use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
use errors::*;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::digest::YDigest;
use crypto::hash::YHash;
use crypto::elliptic::scalar::YScalar;
use input::YInput;
use output::YOutput;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct YTransaction {
  id: YDigest,
  version: YVersion,
  time: YTime,
  inputs: Vec<YInput>,
  outputs: Vec<YOutput>,
}

impl YTransaction {
  pub fn new(inputs: Vec<YInput>, outputs: Vec<YOutput>) -> YResult<YTransaction> {
    let inputs_len = inputs.len();
    let mut inputs_refs = Vec::new();
    for i in 0..inputs_len {
      let inp = inputs[i];
      let refs = (inp.id, inp.idx);
      inputs_refs.push(refs);
    }
    inputs_refs.sort();
    inputs_refs.dedup();
    if inputs_refs.len() != inputs_len {
      return Err(YErrorKind::DuplicateItem.into());
    }
    let outputs_len = outputs.len();
    let mut outputs_refs = Vec::new();
    for i in 0..outputs_len {
      let out = outputs[i].clone();
      let refs = YHash::hash(&out.sender.to_bytes()[..]);
      outputs_refs.push(refs);
    }
    outputs_refs.sort();
    outputs_refs.dedup();
    if outputs_refs.len() != outputs_len {
      return Err(YErrorKind::DuplicateItem.into());
    }
    let now = YTime::now();
    let version = YVersion::default();
    let id = YDigest::default();
    let mut tx = YTransaction {
      id: id,
      version: version,
      time: now,
      inputs: inputs.clone(),
      outputs: outputs,
    };
    let inputs_len = inputs.len();
    for i in 0..inputs_len {
      let c = tx.calc_challenge(i as u32)?;
      if inputs[i].c != c {
        return Err(YErrorKind::InvalidInputChallenge(i).into());
      }  
    }
    
    tx.id = tx.calc_id()?;
    
    Ok(tx)
  }

  pub fn calc_challenge(&self, idx: u32) -> YResult<YScalar> {
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
    let buf = tx_copy.to_bytes()?;
    Ok(YScalar::hash_from_bytes(buf.as_slice()))
  }

  pub fn calc_id(&self) -> YResult<YDigest> {
    let mut buf = Vec::new();
    if let Some(version_buf) = self.version.to_bytes() {
      buf.write(&version_buf[..])?;
    } else {
      return Err(YErrorKind::Unknown.into());
    }
    
    buf.write(&self.time.to_bytes()[..])?;

    let inputs = self.inputs.clone();
    let inputs_len = inputs.len();

    buf.write_u32::<BigEndian>(inputs_len as u32)?;
    
    for i in 0..inputs_len {
      if let Some(input_buf) = inputs[i].to_bytes() {
        buf.write_u32::<BigEndian>(input_buf.len() as u32)?;
        buf.write(input_buf.as_slice())?;
      } else {
        return Err(YErrorKind::Unknown.into());
      }
    }

    let outputs = self.outputs.clone();
    let outputs_len = outputs.len();

    buf.write_u32::<BigEndian>(outputs_len as u32)?;
    
    for i in 0..outputs_len {
      let output_buf = outputs[i].to_bytes()?;
      buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
      buf.write(output_buf.as_slice())?;
    }
    Ok(YHash::hash(buf.as_slice())) 
  }

  pub fn to_bytes(&self) -> YResult<Vec<u8>> {
    let mut buf = Vec::new();
    buf.write(&self.id.to_bytes()[..])?;
    if let Some(version_buf) = self.version.to_bytes() {
      buf.write(&version_buf[..])?;
    } else {
      return Err(YErrorKind::Unknown.into());
    }
    buf.write(&self.time.to_bytes()[..])?;
    let inputs = self.inputs.clone();
    let inputs_len = inputs.len();
    buf.write_u32::<BigEndian>(inputs_len as u32)?;
    for i in 0..inputs_len {
      if let Some(input_buf) = inputs[i].to_bytes() {
        buf.write_u32::<BigEndian>(input_buf.len() as u32)?;
        buf.write(input_buf.as_slice())?;
      } else {
        return Err(YErrorKind::Unknown.into());
      }
    }
    let outputs = self.outputs.clone();
    let outputs_len = outputs.len();
    buf.write_u32::<BigEndian>(outputs_len as u32)?;
    for i in 0..outputs_len {
      let output_buf = outputs[i].to_bytes()?;
      buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
      buf.write(output_buf.as_slice())?;
    }
    Ok(buf)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YTransaction> {
    if b.len() < 104 {
      return Err(YErrorKind::Unknown.into());
    }
    
    let mut tx = YTransaction::default();

    if let Some(_id) = YDigest::from_bytes(&b[0..64]) {
      tx.id = _id;
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    if let Some(_version) = YVersion::from_bytes(&b[64..88]) {
      tx.version = _version;
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    tx.time = YTime::from_bytes(&b[88..96]);

    let inputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..inputs_len {
      let input_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      if let Some(input) = YInput::from_bytes(&b[i+8..i+8+input_len]) {
        tx.inputs.push(input);      
      } else {
        return Err(YErrorKind::Unknown.into());
      }
    }

    let outputs_len = BigEndian::read_u32(&b[0..4]) as usize;

    for i in 0..outputs_len {
      let output_len = BigEndian::read_u32(&b[i+4..i+8]) as usize;
      let output = YOutput::from_bytes(&b[i+8..i+8+output_len])?;
      tx.outputs.push(output);      
    }

    for i in 0..inputs_len as usize {
      let _c = tx.calc_challenge(i as u32)?;
      if tx.inputs[i].c != _c {
        return Err(YErrorKind::Unknown.into());
      }  
    }
    
    tx.id = tx.calc_id()?;

    Ok(tx)
  }

  pub fn verify_input(&self, idx: u32, output: &YOutput) -> YResult<bool> {
    if self.inputs.len() - 1 < idx as usize {
      return Err(YErrorKind::Unknown.into());
    }
    Ok(self.inputs[idx as usize].verify(output))
  }

  pub fn verify(&self, outputs: Vec<YOutput>) -> YResult<bool> {
    let len = self.inputs.len();
    if outputs.len() != len {
      return Err(YErrorKind::InvalidLength(len, outputs.len()).into());
    }
    for idx in 0..len {
      let verified = self.verify_input(idx as u32, &outputs[idx as usize])?;
      if !verified {
        return Ok(false)
      }
    }
    Ok(true)
  }

  pub fn drop_output(mut self, idx: u32) -> YResult<YTransaction> {
    let i = idx as usize;
    if self.outputs.len() -1 < i {
      return Err(YErrorKind::IndexOutOfBound(i as usize, self.outputs.len()).into());
    }
    self.outputs[i] = self.outputs[i].clone().drop();
    Ok(self)
  }

  pub fn drop_all(mut self) -> YTransaction {
    for i in 0..self.outputs.len() {
      self.outputs[i] = self.outputs[i].clone().drop();
    }
    self
  }
}
