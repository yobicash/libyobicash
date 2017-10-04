use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crypto::elliptic::credentials::{YSecretKey, YPublicKey};
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
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    let sender = sk.public_key();
    let max_amount = YAmount::max_value();
    if amount > max_amount {
      return None;
    }
    Some(YOutput {
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
    custom: Option<[u8; 32]>) -> Option<YOutput> {
    let sender = sk.public_key();
    if let Some(data) = YData::new(sk, recipient, iv, plain) {
      Some(YOutput {
        sender: sender.clone(),
        recipient: recipient.clone(),
        amount: data.amount(),
        data: Some(data),
        custom: custom,
      })
    } else {
      None
    }
  }

  pub fn to_bytes(&self) -> Option<Vec<u8>> {
    let mut buf = Vec::new();

    match buf.write(&self.sender.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }

    match buf.write(&self.recipient.to_bytes()[..]) {
      Ok(_) => {},
      Err(_) => { return None; },
    }

    let amount_buf = self.amount.to_bytes();
    match buf.write_u32::<BigEndian>(amount_buf.len() as u32) {
      Ok(_) => {},
      Err(_) => { return None; },
    }
    match buf.write(amount_buf.as_slice()) {
      Ok(_) => {},
      Err(_) => { return None; },
    }

    if let Some(_data) = self.data.clone() {
      if let Some(_data_buf) = _data.to_bytes() {
        match buf.write_u32::<BigEndian>(_data_buf.len() as u32) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
        match buf.write(_data_buf.as_slice()) {
          Ok(_) => {},
          Err(_) => { return None; },
        }
      } else {
        return None;
      }
    } else {
      match buf.write_u32::<BigEndian>(0) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
    }

    if let Some(_custom) = self.custom {
      match buf.write_u32::<BigEndian>(1) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
      match buf.write(&_custom[..]) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
    } else {
      match buf.write_u32::<BigEndian>(0) {
        Ok(_) => {},
        Err(_) => { return None; },
      }
    }

    Some(buf)
  }

  pub fn from_bytes(b: &[u8]) -> Option<YOutput> {
    if b.len() < 140 {
      return None;
    }

    let mut out = YOutput::default();

    let mut reader = Cursor::new(b);

    let mut sender_buf = [0u8; 64];
    match reader.read_exact(&mut sender_buf[..]) {
      Ok(_) => { 
        if let Some(out_sender) = YPublicKey::from_bytes(&sender_buf[..]) {
          out.sender = out_sender;
        } else {
          return None;
        }
      },
      Err(_) => { return None; },
    }

    let mut recipient_buf = [0u8; 64];
    match reader.read_exact(&mut recipient_buf[..]) {
      Ok(_) => { 
        if let Some(out_recipient) = YPublicKey::from_bytes(&recipient_buf[..]) {
          out.recipient = out_recipient;
        } else {
          return None;
        }
      },
      Err(_) => { return None; },
    }

    let mut amount_size = 0u32;
    match reader.read_u32::<BigEndian>() {
      Ok(_amount_size) => { amount_size = _amount_size },
      Err(_) => { return None },
    }

    if amount_size > 0 {
      let mut amount = Vec::new();
      for i in 0..amount_size as usize {
        amount[i] = 0;
      }
      match reader.read_exact(amount.as_mut_slice()) {
        Ok(_) => {
          out.amount = YAmount::from_bytes(amount.as_slice());
        },
        Err(_) => { return None; },
      }
    }

    let mut data_size = 0u32;
    match reader.read_u32::<BigEndian>() {
      Ok(_data_size) => {
        if amount_size == 0 && _data_size != 0 {
          return None;
        }
        data_size = _data_size
      },
      Err(_) => { return None },
    }

    // NB: Result to manage
    if YAmount::from_u64(data_size as u64).unwrap() <= out.amount {
      return None;
    }

    if data_size > 0 {
      let mut data = Vec::new();
      for i in 0..data_size as usize {
        data[i] = 0;
      }
      match reader.read_exact(data.as_mut_slice()) {
        Ok(_) => {
          if let Some(out_data) = YData::from_bytes(data.as_slice()) {
            out.data = Some(out_data);
          } else {
            return None;
          }
        },
        Err(_) => { return None; },
      }
    } else {
      out.data = None;
    }

    let mut has_custom = 0u32;
    match reader.read_u32::<BigEndian>() {
      Ok(_has_custom) => { has_custom = _has_custom },
      Err(_) => { return None },
    }

    if has_custom == 1 {
      let mut custom = [0u8; 32];
      match reader.read_exact(&mut custom[..]) {
        Ok(_) => {
          out.custom = Some(custom)
        },
        Err(_) => { return None; },
      }
    }

    Some(out)
  }

  pub fn drop(mut self) -> YOutput {
    if self.data.is_some() {
      let data = self.data.unwrap().clone();
      self.data = Some(data.drop());
    }
    self
  }
}
