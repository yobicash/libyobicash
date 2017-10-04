use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use errors::*;
use crypto::digest::YDigest;
use crypto::hash::YHash;
use crypto::mac::YMACCode;
use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
use crypto::encryption::ecies::YECIES;
use crypto::encryption::symmetric::YIV;
use amount::YAmount;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct YData {
  pub data: Vec<u8>,
  pub checksum: YDigest,
  pub iv: YIV,
  pub tag: YMACCode,
}

impl YData {
  pub fn new(
    sk: &YSecretKey,
    other: &YPublicKey,
    iv: YIV,
    plain: &[u8]) -> YResult<YData> {
    let ecies = YECIES::new(sk.clone());
    let (data, tag) = ecies.encrypt_and_authenticate(other, iv, plain)?;
    let digest = YHash::hash(data.as_slice());
    Ok(YData{
      data: data,
      checksum: digest,
      iv: iv,
      tag: tag,
    })
  }

  pub fn to_bytes(&self) -> YResult<Vec<u8>> {
    let mut buf = Vec::new();
    let size = self.data.len() as u32;
    buf.write_u32::<BigEndian>(size)?;
    buf.write(self.data.as_slice())?;
    buf.write(&self.checksum.to_bytes()[..])?;
    buf.write(&self.iv.to_bytes()[..])?;
    buf.write(&self.tag.to_bytes()[..])?;
    Ok(buf)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YData> {
    if b.len() < 132 {
      return Err(YErrorKind::InvalidLength.into());
    }

    let mut reader = Cursor::new(b);

    let mut data = YData::default();

    let size = reader.read_u32::<BigEndian>()?;

    for i in 0..size as usize {
      data.data[i] = 0;
    }
    reader.read_exact(data.data.as_mut_slice())?;

    let mut checksum_buf = [0u8; 64];
    reader.read_exact(&mut checksum_buf[..])?;

    data.checksum = YDigest::from_bytes(&checksum_buf[..])?;

    reader.read_exact(&mut data.iv.to_bytes()[..])?;

    reader.read_exact(&mut data.tag.to_bytes()[..])?;

    Ok(data)
  }

  pub fn verify(&self, sk: &YSecretKey, other: &YPublicKey) -> YResult<bool> {
    let ecies = YECIES::new(sk.clone());
    ecies.verify(other, self.data.as_slice(), self.tag)
  }

  pub fn verify_and_decrypt(&self, sk: &YSecretKey, other: &YPublicKey) -> YResult<Vec<u8>> {
    let ecies = YECIES::new(sk.clone());
    ecies.verify_and_decrypt(other, self.iv, self.data.as_slice(), self.tag)
  }

  pub fn amount(&self) -> YAmount {
    YAmount::from_u64(self.data.len() as u64).unwrap()
  }

  pub fn drop(mut self) -> YData {
    self.data = Vec::new();
    self
  }
}
