use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::encryption::ecies::YECIES;
use amount::YAmount;

#[derive(Clone)]
pub struct YData {
  pub data: Vec<u8>,
  pub iv: [u8; 64],
  pub tag: [u8; 64],
}

impl YData {
  pub fn new(
    g: YPoint,
    sk: YScalar,
    other: YPoint,
    iv: &[u8],
    plain: &[u8]) -> Option<YData> {
    if let Some(ecies) = YECIES::new(&g, &sk) {
      if let Some((data, tag)) = ecies.encrypt_and_authenticate(&other, iv, plain) {
        let mut _iv = [0u8; 64];
        for i in 0.._iv.len() {
          _iv[i] = iv[i]
        }
        let mut _tag = [0u8; 64];
        for i in 0.._tag.len() {
          _tag[i] = tag[i]
        }
        Some(YData{
          data: data,
          iv: _iv,
          tag: _tag,
        })
      } else {
        None
      }   
    } else {
     None
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> { unreachable!() }

  pub fn from_bytes(b: &[u8]) -> Option<YData> { unreachable!() }

  pub fn verify(&self, g: YPoint, sk: YScalar, other: YPoint) -> Option<bool> {
    if let Some(ecies) = YECIES::new(&g, &sk) {
      if let Some(verified) = ecies.verify(&other, self.data.as_slice(), &self.tag) {
        Some(verified)
      } else {
        None
      }   
    } else {
     None
    }
  }

  pub fn verify_and_decrypt(&self, g: YPoint, sk: YScalar, other: YPoint) -> Option<Vec<u8>> {
    if let Some(ecies) = YECIES::new(&g, &sk) {
      if let Some(data) = ecies.verify_and_decrypt(&other, &self.iv[..], self.data.as_slice(), &self.tag) {
        Some(data)
      } else {
        None
      }   
    } else {
     None
    }
  }

  pub fn amount(&self) -> YAmount {
    YAmount::from_u64(self.data.len() as u64).unwrap()
  }
}
