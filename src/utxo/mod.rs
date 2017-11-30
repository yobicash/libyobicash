use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use crypto::hash::digest::YDigest64;
use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::YPoint;
use crypto::elliptic::keys::*;
use crypto::zkp::schnorr_protocol::YSchnorrProtocol;
use amount::YAmount;
use input::YInput;
use output::YOutput;
use std::io::{Write, Read, Cursor};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YUTXO {
pub id: YDigest64,
pub idx: u32,
pub recipient: YPublicKey,
pub amount: YAmount,
}

impl YUTXO {
    pub fn new(id: YDigest64, idx: u32, recipient: YPublicKey, amount: YAmount) -> YResult<YUTXO> {
        amount.check()?;
        Ok(YUTXO {
            id: id,
            idx: idx,
            recipient: recipient,
            amount: amount,
        })
    }

    pub fn from_output(out: &YOutput, id: YDigest64, idx: u32) -> YResult<YUTXO> {
        let amount = out.amount.clone();
        amount.check()?;
        Ok(YUTXO {
            id: id,
            idx: idx,
            recipient: out.recipient,
            amount: amount,
        })
    }

    pub fn to_input(&self, x: YScalar, u: YScalar, c: YScalar) -> YResult<YInput> {
        let g = YPoint::default();
        let w = &g*&x;
        if w != self.recipient.pk {
            let msg = String::from("Invalid point");
            return Err(YErrorKind::InvalidPoint(msg).into());
        }
        let secret_prot = YSchnorrProtocol {
            g: YPoint::default(),
            x: x,
            w: w,
            u: u,
            t: &g*&u,
            c: c,
            r: &u + &(&x*&c),
        };
        let prot = secret_prot.to_public();
        let input = YInput::new(self.id, self.idx, prot);
        Ok(input)
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;
        buf.write_u32::<BigEndian>(self.idx)?;
        buf.write(&self.recipient.to_bytes()[..])?;
        let amount_buf = self.amount.to_bytes()?;
        buf.write_u32::<BigEndian>(amount_buf.len() as u32)?;
        buf.write(amount_buf.as_slice())?;
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YUTXO> {
        if b.len() < 136 {
            return Err(YErrorKind::InvalidLength.into());
        }
        
        let mut utxo = YUTXO::default();

        let mut reader = Cursor::new(b);

        let mut id_buf = [0u8; 64];
        reader.read_exact(&mut id_buf[..])?;
        utxo.id = YDigest64::from_bytes(&id_buf[..])?;

        utxo.idx = reader.read_u32::<BigEndian>()?;

        let mut recipient_buf = [0u8; 64];
        reader.read_exact(&mut recipient_buf[..])?;
        utxo.recipient = YPublicKey::from_bytes(&recipient_buf[..])?;

        let amount_size = reader.read_u32::<BigEndian>()?;
        if amount_size > 0 {
            let mut amount = Vec::new();
            for _ in 0..amount_size as usize {
                amount.push(0);
            }
            reader.read_exact(amount.as_mut_slice())?;
            utxo.amount = YAmount::from_bytes(amount.as_slice())?;
        }

        utxo.check()?;

        Ok(utxo)
    }

    pub fn from_hex(s: &str) -> YResult<YUTXO> {
        let buf = s.from_hex()?;
        YUTXO::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn check(&self) -> YResult<()> {
        self.amount.check()
    }
}
