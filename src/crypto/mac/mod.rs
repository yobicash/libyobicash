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

  pub fn result(self) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(self.0.result().code());
    code
  }
 
  pub fn mac(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut m = YMAC::new(key);
    m.update(msg);
    m.result()
  }

  pub fn verify(self, code: &[u8]) -> bool {
    self.0.verify(code)
  }
}
