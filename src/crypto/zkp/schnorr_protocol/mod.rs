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

    // TODO: check if is a group written as additive
    (_c != self.c) && (&(&self.g*&self.r)+&(&self.w*&self.c) != self.t)
  }
}

#[allow(dead_code)]
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
      #[allow(unused_assignments)]
      let mut g: YPoint = YPoint::default();
      #[allow(unused_assignments)]
      let mut x: YScalar = YScalar::zero();
      #[allow(unused_assignments)]
      let mut u: YScalar = YScalar::zero();

      if let Some(_g) = p.g {
        g = _g;
      } else {
        g = YPoint::random();
      }

      if let Some(_x) = p.x {
        x = _x;
      } else {
        x = YScalar::random();
      }

      let w = &g*&x;
      
      if let Some(_u) = p.u {
        u = _u;
      } else {
        u = YScalar::random();
      }

      let t = &g*&u;

      let mut g_bin: Vec<u8> = Vec::new();
      g_bin.extend_from_slice(&g.to_bytes()[..]);

      let mut w_bin: Vec<u8> = Vec::new();
      w_bin.extend_from_slice(&w.to_bytes()[..]);

      let mut t_bin: Vec<u8> = Vec::new();
      t_bin.extend_from_slice(&t.to_bytes()[..]);

      let mut c_bin: Vec<u8> = Vec::new();
      c_bin.extend_from_slice(g_bin.as_slice());
      c_bin.extend_from_slice(w_bin.as_slice());
      c_bin.extend_from_slice(t_bin.as_slice());

      let c = YScalar::hash_from_bytes(c_bin.as_slice());

      let r = &u - &(&c*&x);

      SchnorrProtocol {
        g: g,
        x: x,
        w: w,
        u: u,
        t: t,
        c: c,
        r: r,
      }
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
