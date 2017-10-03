use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
use crypto::encryption::ecies::YECIES;
use crypto::encryption::symmetric::YIV;
use crypto::mac::YMACCode;
use amount::YAmount;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YData {
  pub data: Vec<u8>,
  pub iv: YIV,
  pub tag: YMACCode,
}

impl YData {
  pub fn new(
    sk: &YSecretKey,
    other: &YPublicKey,
    iv: YIV,
    plain: &[u8]) -> Option<YData> {
    let ecies = YECIES::new(sk.clone());
    if let Some((data, tag)) = ecies.encrypt_and_authenticate(other, iv, plain) {
      Some(YData{
        data: data,
        iv: iv,
        tag: tag,
      })
    } else {
      None
    }   
  }

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    let size = self.data.len() as u32;
    match buf.write_u32::<BigEndian>(size) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(self.data.as_slice()) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.iv.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.tag.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    Some(buf)
  }

  pub fn from_bytes(b: &[u8]) -> Option<YData> {
    if b.len() < 132 {
      return None;
    }

    let mut reader = Cursor::new(b);

    let mut data = YData::default();

    let mut size = 0u32;
    match reader.read_u32::<BigEndian>() {
      Ok(_size) => { size = _size },
      Err(_) => { return None },
    }

    for i in 0..size as usize {
      data.data[i] = 0;
    }
    match reader.read_exact(data.data.as_mut_slice()) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    match reader.read_exact(&mut data.iv.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    match reader.read_exact(&mut data.tag.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    Some(data)
  }

  pub fn verify(&self, sk: &YSecretKey, other: &YPublicKey) -> Option<bool> {
    let ecies = YECIES::new(sk.clone());
    if let Some(verified) = ecies.verify(other, self.data.as_slice(), self.tag) {
      Some(verified)
    } else {
      None
    }   
  }

  pub fn verify_and_decrypt(&self, sk: &YSecretKey, other: &YPublicKey) -> Option<Vec<u8>> {
    let ecies = YECIES::new(sk.clone());
    if let Some(data) = ecies.verify_and_decrypt(other, self.iv, self.data.as_slice(), self.tag) {
      Some(data)
    } else {
      None
    }
  }

  pub fn amount(&self) -> YAmount {
    YAmount::from_u64(self.data.len() as u64).unwrap()
  }
}
