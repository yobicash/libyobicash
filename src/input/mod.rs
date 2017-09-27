use curve25519_dalek::edwards::ValidityCheck;
use crypto::digest::YDigest;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::zkp::schnorr_protocol::SchnorrProtocolPublic;
use output::YOutput;

#[derive(Copy, Clone, Debug)]
pub struct YInput {
  pub id: YDigest,
  pub idx: u64,
  pub height: u64,
  pub g: YPoint,
  pub t: YPoint,
  pub c: YScalar,
  pub r: YScalar,
}

impl YInput {
  pub fn new(
      id: YDigest,
      idx: u64,
      height: u64,
      g: YPoint,
      t: YPoint,
      c: YScalar,
      r: YScalar) -> Option<YInput> {
      if !g.is_valid() || !t.is_valid() {
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

  // TODO: verify g is H(tx') where tx' has id == id and output idx == out, and tx' = partial transaction with - schnorr protocol stuff, and no other outputs [NB: check this thing, ain't so binding]
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
