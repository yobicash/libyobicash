use curve25519_dalek::edwards::ValidityCheck;
use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
use crypto::digest::YDigest;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::zkp::schnorr_protocol::SchnorrProtocolPublic;
use output::YOutput;
use std::io::Write;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct YInput {
  pub id: YDigest,
  pub idx: u32,
  pub height: u64,
  pub g: YPoint,
  pub t: YPoint,
  pub c: YScalar,
  pub r: YScalar,
}

impl YInput {
  pub fn new(
      id: YDigest,
      idx: u32,
      height: u64,
      g: YPoint,
      t: YPoint,
      c: YScalar,
      r: YScalar) -> Option<YInput> {
      if height == 0 || !g.is_valid() || !t.is_valid() {
        None
      } else {
          Some(YInput {
            id: id,
            idx: idx,
            height: height,
            g: g,
            t: t,
            c: c,
            r: r,
          })
      }
  }

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    match buf.write(&self.id.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write_u32::<BigEndian>(self.idx) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write_u64::<BigEndian>(self.height) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.g.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.t.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.c.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.r.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    Some(buf)
  }

  pub fn from_bytes(b: &[u8]) -> Option<YInput> {
    if b.len() != 204 {
      return None;
    }

    let mut input = YInput::default();

    if let Some(_id) = YDigest::from_bytes(&b[0..64]) {
      input.id = _id;
    } else {
      return None;
    }

    input.idx = BigEndian::read_u32(&b[64..68]);

    input.height = BigEndian::read_u64(&b[68..76]);

    if let Some(_g) = YPoint::from_bytes(&b[76..108]) {
      input.g = _g; 
    } else {
      return None;
    }

    if let Some(_t) = YPoint::from_bytes(&b[108..140]) {
      input.t = _t; 
    } else {
      return None;
    }

    if let Some(_c) = YScalar::from_bytes(&b[140..172]) {
      input.c = _c; 
    } else {
      return None;
    }

    if let Some(_r) = YScalar::from_bytes(&b[172..204]) {
      input.r = _r; 
    } else {
      return None;
    }

    Some(input)
  }

  pub fn verify(&self, out: &YOutput) -> bool {
    let prot = SchnorrProtocolPublic {
      g: self.g,
      w: out.recipient.pk,
      t: self.t,
      c: self.c,
      r: self.r,
    };
    prot.verify()
  }
}
