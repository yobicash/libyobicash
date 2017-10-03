use sha2::Sha512;
use hmac::{Mac, Hmac};

pub struct YMAC(pub Hmac<Sha512>);

impl YMAC {
  pub fn new(key: &[u8]) -> YMAC {
    YMAC(Hmac::<Sha512>::new(key))
  }

  pub fn update(&mut self, msg: &[u8]) {
    self.0.input(msg)
  }

  pub fn result(self) -> [u8; 64] {
    let mut code = [0u8; 64];
    let res = self.0.result();
    let c = res.code();
    for i in 0..64 {
      code[i] = c[i];
    }
    code
  }
 
  pub fn mac(key: &[u8], msg: &[u8]) -> [u8; 64] {
    let mut m = YMAC::new(key);
    m.update(msg);
    m.result()
  }

  pub fn verify(self, code: &[u8]) -> bool {
    self.0.verify(code)
  }
}
