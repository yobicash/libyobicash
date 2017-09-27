pub struct SchnorrProtocolPublic {
  g: YPoint,
  w: YPoint,
  t: YPoint,
  c: YPoint,
  r: YScalar,
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

    let _c = YScalar::hash_from_bytes(c_bin.as_slice());

    (_c != self.c) && ((self.g*self.r)*(self.w*self.c) != self.t)
  }
}

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
  x: Option<YPoint>,
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
      let mut s = SchnorrProtocol{};

      if let Some(g) = p.g {
        s.g = g;
      }

      if let Some(x) = p.x {
        s.x = x;
      }

      let w = g*x;
      s.w = w;
      
      if let Some(u) = p.u {
        s.u = u;
      }

      let t = g*u;
      s.t = t;

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
      s.c = c;

      let r = u - c*x;
      s.r = r;

      s
    }

    pub fn random() -> SchnorrProtocol {
      SchnorrProtocol::from_params(&SchnorrProtocolParams::default())
    }

    pub fn to_public(&self) -> SchnorrProtocolPublic {
      SchnorrProtocolPublic {
        g: self.g,
        w. self.w,
        t: self.t,
        c: self.c,
        r: self.r,
      }
    }
}
