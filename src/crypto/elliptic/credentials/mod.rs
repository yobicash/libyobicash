use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct YPublicKey {
    pub g: YPoint,
    pub pk: YPoint,
}

impl YPublicKey {
  pub fn new(g: YPoint, pk: YPoint) -> YPublicKey {
    YPublicKey {
      g: g,
      pk: pk,
    }
  }

  pub fn to_bytes(&self) -> [u8; 64] {
    let mut buf = [0u8; 64];
    let g_buf = self.g.to_bytes();
    for i in 0..32 {
      buf[i] = g_buf[i];
    }
    let pk_buf = self.pk.to_bytes();
    for i in 0..32 {
      buf[i] = pk_buf[i];
    }
    buf
  }

  pub fn from_bytes(b: &[u8]) -> Option<YPublicKey> {
    if b.len() != 64 {
      return None;
    }

    let mut pk = YPublicKey::default();
    
    let g_buf = &b[0..32];
    if let Some(g) = YPoint::from_bytes(g_buf) {
      pk.g = g;
    } else {
      return None;
    }
    
    let pk_buf = &b[0..32];
    if let Some(_pk) = YPoint::from_bytes(pk_buf) {
      pk.pk = _pk;
    } else {
      return None;
    }

    Some(pk)
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct YSecretKey {
  pub g: YPoint,
  pub sk: YScalar,
}

impl YSecretKey {
  pub fn new(g: YPoint, sk: YScalar) -> YSecretKey {
    YSecretKey {
      g: g,
      sk: sk,
    }
  }

  pub fn random() -> YSecretKey {
    YSecretKey {
      g: YPoint::random(),
      sk: YScalar::random(),
    }
  }

  pub fn from_g(g: YPoint) -> YSecretKey {
    YSecretKey {
      g: g,
      sk: YScalar::random(),
    }
  }

  pub fn public_key(&self) -> YPublicKey {
    YPublicKey::new(self.g, &self.g*&self.sk)
  }

  pub fn to_bytes(&self) -> [u8; 64] {
    let mut buf = [0u8; 64];
    let g_buf = self.g.to_bytes();
    for i in 0..32 {
      buf[i] = g_buf[i];
    }
    let sk_buf = self.sk.to_bytes();
    for i in 0..32 {
      buf[i] = sk_buf[i];
    }
    buf
  }

  pub fn from_bytes(b: &[u8]) -> Option<YSecretKey> {
    if b.len() != 64 {
      return None;
    }

    let mut sk = YSecretKey::default();
    
    let g_buf = &b[0..32];
    if let Some(_g) = YPoint::from_bytes(g_buf) {
      sk.g = _g;
    } else {
      return None;
    }
    
    let sk_buf = &b[0..32];
    if let Some(_sk) = YScalar::from_bytes(sk_buf) {
      sk.sk = _sk;
    } else {
      return None;
    }

    Some(sk)
  }
}
