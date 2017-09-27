use typenum::consts::U64;
use sha2::Sha512;
use hmac::{Mac, MacResult, Hmac};

pub struct YMAC(pub Hmac<Sha512>);

pub type YMACResult = MacResult<U64>;

impl YMAC {
  pub fn new(key: &[u8]) -> YMAC {
    YMAC(Hmac::<Sha512>::new(key))
  }

  pub fn update(&mut self, msg: &[u8]) {
    self.0.input(msg)
  }

  pub fn result(self) -> YMACResult {
    self.0.result() 
  }
 
  pub fn mac(key: &[u8], msg: &[u8]) -> YMACResult {
    let mut m = YMAC::new(key);
    m.update(msg);
    m.result()
  }

  pub fn verify(self, mac: &YMACResult) -> bool {
    let code = mac.code(); // &[u8]
    self.0.verify(code)
  }
}
