use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::encryption::ecies::YECIES;
use amount::YAmount;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct YData {
  pub data: Vec<u8>,
  pub iv: Vec<u8>,
  pub tag: Vec<u8>,
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
        let mut _iv = Vec::new();
        _iv.extend_from_slice(iv);
        Some(YData{
          data: data,
          iv: _iv,
          tag: tag,
        })
      } else {
        None
      }   
    } else {
     None
    }
  }

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
      if let Some(data) = ecies.verify_and_decrypt(&other, self.iv.as_slice(), self.data.as_slice(), &self.tag) {
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
