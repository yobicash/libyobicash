use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use errors::*;
use crypto::elliptic::keys::{YSecretKey, YPublicKey};
use crypto::encryption::symmetric::YIV;
use amount::YAmount;
use data::YData;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Default)]
pub struct YOutput {
  pub sender: YPublicKey,
  pub recipient: YPublicKey,
  pub amount: YAmount,
  pub data: Option<YData>,
  pub custom: Option<[u8; 32]>,
}

impl YOutput {
  pub fn new(
    sk: &YSecretKey,
    recipient: &YPublicKey,
    amount: YAmount,
    custom: Option<[u8; 32]>) -> YResult<YOutput> {
    let sender = sk.public_key();
    let max_amount = YAmount::max_value();
    if amount > max_amount {
      return Err(YErrorKind::AmountOutOfBound.into());
    }
    Ok(YOutput {
      sender: sender.clone(),
      recipient: recipient.clone(),
      amount: amount.clone(),
      data: None,
      custom: custom,
    })
  }

  pub fn with_data(
    sk: &YSecretKey,
    recipient: &YPublicKey,
    iv: YIV,
    plain: &[u8],
    custom: Option<[u8; 32]>) -> YResult<YOutput> {
    let sender = sk.public_key();
    let data = YData::new(sk, recipient, iv, plain)?;
    Ok(YOutput {
      sender: sender.clone(),
      recipient: recipient.clone(),
      amount: data.amount(),
      data: Some(data),
      custom: custom,
    })
  }

  pub fn to_bytes(&self) -> YResult<Vec<u8>> {
    let mut buf = Vec::new();

    buf.write(&self.sender.to_bytes()[..])?;

    buf.write(&self.recipient.to_bytes()[..])?;

    let amount_buf = self.amount.to_bytes();
    buf.write_u32::<BigEndian>(amount_buf.len() as u32)?;
    buf.write(amount_buf.as_slice())?;

    if let Some(_data) = self.data.clone() {
      let _data_buf = _data.to_bytes()?;
      buf.write_u32::<BigEndian>(_data_buf.len() as u32)?;
      buf.write(_data_buf.as_slice())?;
    } else {
      buf.write_u32::<BigEndian>(0)?;
    }

    if let Some(_custom) = self.custom {
      buf.write_u32::<BigEndian>(1)?;
      buf.write(&_custom[..])?;
    } else {
      buf.write_u32::<BigEndian>(0)?;
    }

    Ok(buf)
  }

  pub fn from_bytes(b: &[u8]) -> YResult<YOutput> {
    if b.len() < 140 {
      return Err(YErrorKind::InvalidLength.into());
    }

    let mut out = YOutput::default();

    let mut reader = Cursor::new(b);

    let mut sender_buf = [0u8; 64];
    reader.read_exact(&mut sender_buf[..])?;
    out.sender = YPublicKey::from_bytes(&sender_buf[..])?;

    let mut recipient_buf = [0u8; 64];
    reader.read_exact(&mut recipient_buf[..])?;
    out.recipient = YPublicKey::from_bytes(&recipient_buf[..])?;

    let amount_size = reader.read_u32::<BigEndian>()?;
    if amount_size > 0 {
      let mut amount = Vec::new();
      for i in 0..amount_size as usize {
        amount[i] = 0;
      }
      reader.read_exact(amount.as_mut_slice())?;
      out.amount = YAmount::from_bytes(amount.as_slice());
    }

    let data_size = reader.read_u32::<BigEndian>()?;
    if out.amount == YAmount::zero() && data_size != 0 {
      return Err(YErrorKind::Unknown.into());
    }
    if data_size > 0 {
      let mut data = Vec::new();
      for i in 0..data_size as usize {
        data[i] = 0;
      }
      reader.read_exact(data.as_mut_slice())?;
      out.data = Some(YData::from_bytes(data.as_slice())?);
    }

    let has_custom = reader.read_u32::<BigEndian>()?;
    if has_custom == 1 {
      let mut custom = [0u8; 32];
      reader.read_exact(&mut custom[..])?;
      out.custom = Some(custom);
    }

    Ok(out)
  }

  pub fn drop(mut self) -> YOutput {
    if self.data.is_some() {
      let data = self.data.unwrap().clone();
      self.data = Some(data.drop());
    }
    self
  }
}
