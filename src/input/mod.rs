use curve25519_dalek::edwards::ValidityCheck;
use crypto::digest::YDigest;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::zkp::schnorr_protocol::SchnorrProtocolPublic;
use output::YOutput;

#[derive(Debug, Clone)]
pub struct YPartialInput {
  pub id: YDigest,
  pub idx: u32,
  pub height: u64,
}

impl YPartialInput {
  pub fn new(id: YDigest, idx: u32, height: u64) -> Option<YPartialInput> {
    if height == 0 {
      None
    } else {
      Some(YPartialInput {
        id: id,
        idx: idx,
        height: height,
      })
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YPartialInput> { unreachable!() }

  pub fn complete(self, g: YPoint, t: YPoint, c: YScalar, r: YScalar) -> Option<YInput> {
    YInput::new(self.id, self.idx, self.height, g, t, c, r)
  }  
}

#[derive(Debug, Clone)]
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

  pub fn from_partial(
    i: YPartialInput,
    g: YPoint,
    t: YPoint,
    c: YScalar,
    r: YScalar) -> Option<YInput> {
    i.complete(g, t, c, r)
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YInput> { unreachable!() }

  pub fn verify(&self, out: &YOutput) -> bool {
    let prot = SchnorrProtocolPublic {
      g: self.g,
      w: out.receiver,
      t: self.t,
      c: self.c,
      r: self.r,
    };
    prot.verify()
  }
}
