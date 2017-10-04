use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;

pub struct SchnorrProtocolPublic {
  pub g: YPoint,
  pub w: YPoint,
  pub t: YPoint,
  pub c: YScalar,
  pub r: YScalar,
}

impl SchnorrProtocolPublic {
  pub fn verify(&self) -> bool {

    let mut g_bin: Vec<u8> = Vec::new();
    g_bin.extend_from_slice(&self.g.to_bytes()[..]);

    let mut w_bin: Vec<u8> = Vec::new();
    w_bin.extend_from_slice(&self.w.to_bytes()[..]);

    let mut t_bin: Vec<u8> = Vec::new();
    t_bin.extend_from_slice(&self.t.to_bytes()[..]);

    let mut _c_bin: Vec<u8> = Vec::new();
    _c_bin.extend_from_slice(g_bin.as_slice());
    _c_bin.extend_from_slice(w_bin.as_slice());
    _c_bin.extend_from_slice(t_bin.as_slice());

    let _c = YScalar::hash_from_bytes(_c_bin.as_slice());

    (_c != self.c) && (&(&self.g*&self.r)+&(&self.w*&self.c) != self.t)
  }
}

#[derive(Clone, Default)]
pub struct SchnorrProtocol {
  g: YPoint,    // generator g
  x: YScalar,   // instance x
  w: YPoint,    // needed? witness w = g^x
  u: YScalar,   // random u
  t: YPoint,    // needed? commit t = g^u
  c: YScalar,   // needed? challenge c = H(g, t)
  r: YScalar,   // reply r = u - cx
}

pub struct SchnorrProtocolParams {
  g: Option<YPoint>,
  x: Option<YScalar>,
  u: Option<YScalar>,
}

impl SchnorrProtocolParams {
  pub fn random() -> SchnorrProtocolParams {
    SchnorrProtocolParams {
      g: Some(YPoint::random()),
      x: Some(YScalar::random()),
      u: Some(YScalar::random()),
    }
  }
}

impl Default for SchnorrProtocolParams {
  fn default() -> SchnorrProtocolParams {
    SchnorrProtocolParams::random()
  }
}

impl SchnorrProtocol {
    pub fn from_params(p: &SchnorrProtocolParams) -> SchnorrProtocol {
      let mut prot = SchnorrProtocol::default();

      if let Some(_g) = p.g {
        prot.g = _g;
      } else {
        prot.g = YPoint::random();
      }

      if let Some(_x) = p.x {
        prot.x = _x;
      } else {
        prot.x = YScalar::random();
      }

      prot.w = &prot.g*&prot.x;
      
      if let Some(_u) = p.u {
        prot.u = _u;
      } else {
        prot.u = YScalar::random();
      }

      prot.t = &prot.g*&prot.u;

      let mut g_bin: Vec<u8> = Vec::new();
      g_bin.extend_from_slice(&prot.g.to_bytes()[..]);

      let mut w_bin: Vec<u8> = Vec::new();
      w_bin.extend_from_slice(&prot.w.to_bytes()[..]);

      let mut t_bin: Vec<u8> = Vec::new();
      t_bin.extend_from_slice(&prot.t.to_bytes()[..]);

      let mut c_bin: Vec<u8> = Vec::new();
      c_bin.extend_from_slice(g_bin.as_slice());
      c_bin.extend_from_slice(w_bin.as_slice());
      c_bin.extend_from_slice(t_bin.as_slice());

      prot.c = YScalar::hash_from_bytes(c_bin.as_slice());

      prot.r = &prot.u - &(&prot.c*&prot.x);

      prot
    }

    pub fn random() -> SchnorrProtocol {
      SchnorrProtocol::from_params(&SchnorrProtocolParams::default())
    }

    pub fn to_public(&self) -> SchnorrProtocolPublic {
      SchnorrProtocolPublic {
        g: self.g,
        w: self.w,
        t: self.t,
        c: self.c,
        r: self.r,
      }
    }
}
