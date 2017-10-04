use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
use errors::*;
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
    r: YScalar) -> YResult<YInput> {
    if height == 0 {
      Err(YErrorKind::InvalidHeight.into())
    } else {
      Ok(YInput {
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

  pub fn to_bytes(&self) -> YResult<Vec<u8>> {
    let mut buf = Vec::new();
    buf.write(&self.id.to_bytes()[..])?;
    buf.write_u32::<BigEndian>(self.idx)?;
    buf.write_u64::<BigEndian>(self.height)?;
    buf.write(&self.g.to_bytes()[..])?;
    buf.write(&self.t.to_bytes()[..])?;
    buf.write(&self.c.to_bytes()[..])?;
    buf.write(&self.r.to_bytes()[..])?;
    Ok(buf)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YInput> {
    if b.len() != 204 {
      return Err(YErrorKind::InvalidLength(204, b.len()).into());
    }

    let mut input = YInput::default();

    if let Some(_id) = YDigest::from_bytes(&b[0..64]) {
      input.id = _id;
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    input.idx = BigEndian::read_u32(&b[64..68]);

    input.height = BigEndian::read_u64(&b[68..76]);

    if let Some(_g) = YPoint::from_bytes(&b[76..108]) {
      input.g = _g; 
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    if let Some(_t) = YPoint::from_bytes(&b[108..140]) {
      input.t = _t; 
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    if let Some(_c) = YScalar::from_bytes(&b[140..172]) {
      input.c = _c; 
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    if let Some(_r) = YScalar::from_bytes(&b[172..204]) {
      input.r = _r; 
    } else {
      return Err(YErrorKind::Unknown.into());
    }

    Ok(input)
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
