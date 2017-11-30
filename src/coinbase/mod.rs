use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use output::YOutput;
use std::io::{Write, Cursor, Read};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct YCoinbase {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub height: u64,
    pub activation: Option<YTime>,
    pub outputs: Vec<YOutput>,
}

impl YCoinbase {
    pub fn new(outputs: &Vec<YOutput>, activation: Option<YTime>) -> YResult<YCoinbase> {
        let outputs_len = outputs.len();
        let mut outputs_refs = Vec::new();
        for i in 0..outputs_len {
            let out = outputs[i].clone();
            let refs = YSHA512::hash(&out.sender.to_bytes()[..]);
            outputs_refs.push(refs);
        }
        outputs_refs.sort();
        outputs_refs.dedup();
        if outputs_refs.len() != outputs_len {
            return Err(YErrorKind::DuplicateItem.into());
        }
        
        let now = YTime::now();
        if let Some(_activation) = activation.clone() {
            if _activation <= now {
                return Err(YErrorKind::InvalidTime.into());
            }
        }

        let version = YVersion::default();
        let id = YDigest64::default();
        
        let mut cb = YCoinbase {
            id: id,
            version: version,
            height: 0,
            time: now,
            activation: activation,
            outputs: outputs.clone(),
        };

        cb.id = cb.calc_id()?;

        Ok(cb)
    }

    pub fn calc_id(&self) -> YResult<YDigest64> {
        let mut buf = Vec::new();

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        buf.write_u64::<BigEndian>(self.height)?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_activation) = self.activation.clone() {
            buf.write_u32::<BigEndian>(1)?;
            let activation_buf = _activation.to_bytes();
            buf.write(&activation_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

        let outputs = self.outputs.clone();
        let outputs_len = outputs.len();

        buf.write_u32::<BigEndian>(outputs_len as u32)?;

        for i in 0..outputs_len {
            let output_buf = outputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
            buf.write(output_buf.as_slice())?;
        }
        Ok(YSHA512::hash(buf.as_slice()))
    }

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;

        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        buf.write_u64::<BigEndian>(self.height)?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_activation) = self.activation.clone() {
            buf.write_u32::<BigEndian>(1)?;
            let activation_buf = _activation.to_bytes();
            buf.write(&activation_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

        let outputs = self.outputs.clone();
        let outputs_len = outputs.len();
        buf.write_u32::<BigEndian>(outputs_len as u32)?;
        for i in 0..outputs_len {
            let output_buf = outputs[i].to_bytes()?;
            buf.write_u32::<BigEndian>(output_buf.len() as u32)?;
            buf.write(output_buf.as_slice())?;
        }
        
        Ok(buf)
    }

    pub fn from_bytes(b: &[u8]) -> YResult<YCoinbase> {
        if b.len() < 104 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut tx = YCoinbase::default();

        let mut reader = Cursor::new(b);

        let mut id_buf = [0u8; 64];
        reader.read_exact(&mut id_buf[..])?;
        tx.id = YDigest64::from_bytes(&id_buf[..])?;

        let mut ver_buf = [0u8; 24];
        reader.read_exact(&mut ver_buf[..])?;
        tx.version = YVersion::from_bytes(&ver_buf[..])?;

        tx.height = reader.read_u64::<BigEndian>()?;

        let mut time_buf = [0u8; 8];
        reader.read_exact(&mut time_buf[..])?;
        tx.time = YTime::from_bytes(&time_buf[..])?;

        let has_activation = reader.read_u32::<BigEndian>()?;
        if has_activation == 1 {
            let mut activation_buf = [0u8; 8];
            reader.read_exact(&mut activation_buf[..])?;
            tx.activation = Some(YTime::from_bytes(&activation_buf[..])?);
        }

        let outputs_len = reader.read_u32::<BigEndian>()? as usize;

        for _ in 0..outputs_len {
            let output_len = reader.read_u32::<BigEndian>()? as usize;
            let mut output_buf = Vec::new();
            for _ in 0..output_len {
                output_buf.push(0);
            }
            reader.read_exact(&mut output_buf.as_mut_slice())?;
            let output = YOutput::from_bytes(output_buf.as_slice())?;
            tx.outputs.push(output);
        }

        tx.check()?;

        Ok(tx)
    }

    pub fn from_hex(s: &str) -> YResult<YCoinbase> {
        let buf = s.from_hex()?;
        YCoinbase::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn verify(&self) -> YResult<bool> {
        Ok(true)
    }

    pub fn drop_output(mut self, idx: u32) -> YResult<YCoinbase> {
        let i = idx as usize;
        if self.outputs.len() - 1 < i {
            return Err(
                YErrorKind::IndexOutOfBound(i as usize, self.outputs.len()).into(),
            );
        }
        self.outputs[i] = self.outputs[i].clone().drop();
        Ok(self)
    }

    pub fn drop_all(mut self) -> YCoinbase {
        for i in 0..self.outputs.len() {
            self.outputs[i] = self.outputs[i].clone().drop();
        }
        self
    }

    pub fn is_dropped(&self) -> bool {
        let mut dropped = true;
        for i in 0..self.outputs.len() {
            dropped &= self.outputs[i].is_dropped();
        }
        dropped
    }

    pub fn is_active(&self) -> bool {
        if let Some(_activation) = self.activation.clone() {
            _activation <= YTime::now()
        } else {
            false
        }
    }

    pub fn check(&self) -> YResult<()> {
        if self.id != self.calc_id()? {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        if self.version > YVersion::default() {
            let v = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(v).into());
        }
        
        if self.height != 0 {
            return Err(YErrorKind::InvalidHeight.into());
        }

        let time = self.time.clone();
        let now = YTime::now();
        if time > now {
            return Err(YErrorKind::InvalidTime.into())
        }

        if let Some(_activation) = self.activation.clone() {
            if _activation <= time {
                return Err(YErrorKind::InvalidTime.into())
            }
        }

        for output in self.outputs.clone() {
            output.check()?;
        }

        Ok(())
    }
}
