use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::elliptic::keys::{YSecretKey, YPublicKey};
use amount::YAmount;
use data::YData;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YOutput {
    pub sender: YPublicKey,
    pub recipient: YPublicKey,
    pub amount: YAmount,
    pub data: Option<YData>,
    pub custom: Option<Vec<u8>>,
}

impl YOutput {
    pub fn new(
        sk: &YSecretKey,
        recipient: &YPublicKey,
        amount: YAmount,
        custom: Option<Vec<u8>>,
    ) -> YResult<YOutput> {
        if sk.g != recipient.g {
            let msg = String::from("Invalid generator");
            return Err(YErrorKind::InvalidPoint(msg).into());
        }
        let sender = sk.to_public();
        amount.check()?;
        if let Some(_custom) = custom.clone() {
            if _custom.len() != 256 {
                return Err(YErrorKind::InvalidLength.into());
            }
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
        plain: &[u8],
        custom: Option<Vec<u8>>,
    ) -> YResult<YOutput> {
        if let Some(_custom) = custom.clone() {
            if _custom.len() != 256 {
                return Err(YErrorKind::InvalidLength.into());
            }
        }
        let sender = sk.to_public();
        let data = YData::new(sk, recipient, plain)?;
        Ok(YOutput {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount: data.amount()?,
            data: Some(data),
            custom: custom,
        })
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;

        let mut buf = Vec::new();

        buf.write(&self.sender.to_bytes()[..])?;

        buf.write(&self.recipient.to_bytes()[..])?;

        let amount_buf = self.amount.to_bytes()?;
        buf.write_u32::<BigEndian>(amount_buf.len() as u32)?;
        buf.write(amount_buf.as_slice())?;

        if let Some(_data) = self.data.clone() {
            let _data_buf = _data.to_bytes()?;
            buf.write_u32::<BigEndian>(_data_buf.len() as u32)?;
            buf.write(_data_buf.as_slice())?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

        if let Some(_custom) = self.custom.clone() {
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
            for _ in 0..amount_size as usize {
                amount.push(0);
            }
            reader.read_exact(amount.as_mut_slice())?;
            out.amount = YAmount::from_bytes(amount.as_slice())?;
        }

        let data_size = reader.read_u32::<BigEndian>()?;
        if data_size > 0 {
            let mut data = Vec::new();
            for _ in 0..data_size as usize {
                data.push(0);
            }
            reader.read_exact(data.as_mut_slice())?;
            out.data = Some(YData::from_bytes(data.as_slice())?);
        }

        let has_custom = reader.read_u32::<BigEndian>()?;
        if has_custom == 1 {
            let mut custom = Vec::new();
            for _ in 0..256 {
                custom.push(0);
            }
            reader.read_exact(&mut custom.as_mut_slice())?;
            out.custom = Some(custom);
        }

        out.check()?;

        Ok(out)
    }

    pub fn from_hex(s: &str) -> YResult<YOutput> {
        let buf = s.from_hex()?;
        YOutput::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn drop(mut self) -> YOutput {
        if let Some(_data) = self.data {
            self.data = Some(_data.drop());
        }
        self
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    pub fn is_dropped(&self) -> bool {
        !self.has_data()
    }

    pub fn check(&self) -> YResult<()> {
        if self.sender.g != self.recipient.g {
            let msg = String::from("Invalid generator");
            return Err(YErrorKind::InvalidPoint(msg).into());
        }
        self.amount.check()?;
        if let Some(_custom) = self.custom.clone() {
            if _custom.len() != 256 {
                return Err(YErrorKind::InvalidLength.into());
            }
        }
        if let Some(_data) = self.data.clone() {
            let data_size = _data.amount()?;
            if data_size != YAmount::zero() && self.amount != data_size {
               return Err(YErrorKind::InvalidAmount.into()); 
            }
        }
        Ok(())
    }
}
