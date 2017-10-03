use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
use crypto::encryption::ecies::YECIES;
use amount::YAmount;
use std::io::{Write, Read, Cursor};

#[derive(Clone)]
pub struct YData {
  pub size: u32,
  pub data: Vec<u8>,
  pub iv: [u8; 64],
  pub tag: [u8; 64],
}

impl Default for YData {
  fn default() -> YData {
    YData {
      size: 0,
      data: Vec::new(),
      iv: [0u8; 64],
      tag: [0u8; 64],
    }
  }
}

impl YData {
  pub fn new(
    sk: &YSecretKey,
    other: &YPublicKey,
    iv: &[u8],
    plain: &[u8]) -> Option<YData> {
    let ecies = YECIES::new(sk.clone());
    if let Some((data, tag)) = ecies.encrypt_and_authenticate(other, iv, plain) {
      let mut _iv = [0u8; 64];
      for i in 0.._iv.len() {
        _iv[i] = iv[i]
      }
      let mut _tag = [0u8; 64];
      for i in 0.._tag.len() {
        _tag[i] = tag[i]
      }
      Some(YData{
        size: data.len() as u32,
        data: data,
        iv: _iv,
        tag: _tag,
      })
    } else {
      None
    }   
  }

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    match buf.write_u32::<BigEndian>(self.size) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(self.data.as_slice()) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.iv[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(&self.tag[..]) {
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

    match reader.read_u32::<BigEndian>() {
      Ok(_size) => { data.size = _size },
      Err(_) => { return None },
    }

    for i in 0..data.size as usize {
      data.data[i] = 0;
    }
    match reader.read_exact(data.data.as_mut_slice()) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    match reader.read_exact(&mut data.iv[..]) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    match reader.read_exact(&mut data.tag[..]) {
      Ok(_) => {},
      Err(_) => { return None; }
    }

    Some(data)
  }

  pub fn verify(&self, sk: &YSecretKey, other: &YPublicKey) -> Option<bool> {
    let ecies = YECIES::new(sk.clone());
    if let Some(verified) = ecies.verify(other, self.data.as_slice(), &self.tag) {
      Some(verified)
    } else {
      None
    }   
  }

  pub fn verify_and_decrypt(&self, sk: &YSecretKey, other: &YPublicKey) -> Option<Vec<u8>> {
    let ecies = YECIES::new(sk.clone());
    if let Some(data) = ecies.verify_and_decrypt(other, &self.iv[..], self.data.as_slice(), &self.tag) {
      Some(data)
    } else {
      None
    }
  }

  pub fn amount(&self) -> YAmount {
    YAmount::from_u64(self.data.len() as u64).unwrap()
  }
}
