use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use serialize::hex::{FromHex, ToHex};
use errors::*;
use utils::version::YVersion;
use utils::time::YTime;
use crypto::hash::digest::YDigest64;
use crypto::hash::sha::YSHA512;
use crypto::elliptic::keys::*;
use output::YOutput;
use proof::storage::*;
use proof::work::*;
use amount::*;
use transaction::*;
use std::io::{Write, Cursor, Read};

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct YCoinbase {
    pub id: YDigest64,
    pub version: YVersion,
    pub time: YTime,
    pub post: Option<YPoSt>,
    pub pow: Option<YPoW>,
    pub outputs: Vec<YOutput>,
}

impl YCoinbase {
    pub fn new() -> YResult<YCoinbase> {
        let now = YTime::now();

        let version = YVersion::default();
        let id = YDigest64::default();
        
        let mut cb = YCoinbase {
            id: id,
            version: version,
            time: now,
            post: None,
            pow: None,
            outputs: Vec::new(),
        };

        cb.id = cb.calc_id()?;

        Ok(cb)
    }

    pub fn calc_id(&self) -> YResult<YDigest64> {
        let mut buf = Vec::new();

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_post) = self.post.clone() {
            let post_buf = _post.to_bytes()?;
            buf.write_u32::<BigEndian>(post_buf.len() as u32)?;
            buf.write(&post_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

        if let Some(_pow) = self.pow.clone() {
            let pow_buf = _pow.to_bytes()?;
            buf.write_u32::<BigEndian>(pow_buf.len() as u32)?;
            buf.write(&pow_buf[..])?;
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

    pub fn to_pow_bytes(&self) -> YResult<Vec<u8>> {
        self.pre_pow_check()?;

        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_post) = self.post.clone() {
            let post_buf = _post.to_bytes()?;
            buf.write_u32::<BigEndian>(post_buf.len() as u32)?;
            buf.write(&post_buf[..])?;
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

    pub fn to_bytes(&self) -> YResult<Vec<u8>> {
        self.check()?;

        let mut buf = Vec::new();
        buf.write(&self.id.to_bytes()[..])?;

        let version_buf = self.version.to_bytes()?;
        buf.write(&version_buf[..])?;

        let time_buf = self.time.to_bytes();
        buf.write(&time_buf[..])?;

        if let Some(_post) = self.post.clone() {
            let post_buf = _post.to_bytes()?;
            buf.write_u32::<BigEndian>(post_buf.len() as u32)?;
            buf.write(&post_buf[..])?;
        } else {
            buf.write_u32::<BigEndian>(0)?;
        }

        if let Some(_pow) = self.pow.clone() {
            let pow_buf = _pow.to_bytes()?;
            buf.write_u32::<BigEndian>(pow_buf.len() as u32)?;
            buf.write(&pow_buf[..])?;
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
        if b.len() < 100 {
            return Err(YErrorKind::InvalidLength.into());
        }

        let mut cb = YCoinbase::default();

        let mut reader = Cursor::new(b);

        let mut id_buf = [0u8; 64];
        reader.read_exact(&mut id_buf[..])?;
        cb.id = YDigest64::from_bytes(&id_buf[..])?;

        let mut ver_buf = [0u8; 24];
        reader.read_exact(&mut ver_buf[..])?;
        cb.version = YVersion::from_bytes(&ver_buf[..])?;

        let mut time_buf = [0u8; 8];
        reader.read_exact(&mut time_buf[..])?;
        cb.time = YTime::from_bytes(&time_buf[..])?;

        let post_size = reader.read_u32::<BigEndian>()?;
        if post_size > 0 {
            let mut post = Vec::new();
            for _ in 0..post_size as usize {
                post.push(0);
            }
            reader.read_exact(post.as_mut_slice())?;
            cb.post = Some(YPoSt::from_bytes(post.as_slice())?);
        }
        
        let pow_size = reader.read_u32::<BigEndian>()?;
        if pow_size > 0 {
            let mut pow = Vec::new();
            for _ in 0..pow_size as usize {
                pow.push(0);
            }
            reader.read_exact(pow.as_mut_slice())?;
            cb.pow = Some(YPoW::from_bytes(pow.as_slice())?);
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
            cb.outputs.push(output);
        }

        cb.check()?;

        Ok(cb)
    }

    pub fn from_hex(s: &str) -> YResult<YCoinbase> {
        let buf = s.from_hex()?;
        YCoinbase::from_bytes(buf.as_slice())
    }

    pub fn to_hex(&self) -> YResult<String> {
        Ok(self.to_bytes()?.to_hex())
    }

    pub fn set_post(&mut self, id_tx: YDigest64, diff: u32, nonce: u32, chunks: &Vec<u8>) -> YResult<()> {
        let post = YPoSt::new(id_tx, diff, nonce, chunks)?;
        self.post = Some(post);
        self.id = self.calc_id()?;
        Ok(())
    }

    pub fn coinbase_amount(&self, increment: u32) -> YResult<YAmount> {
        if self.post.is_none() {
            return Err(YErrorKind::PoStNotFound.into());
        }
        let post = self.post.clone().unwrap();
        let pow = YPoW::new(post.digest, post.difficulty, increment)?;
        YAmount::from_u64(pow.memory()?)
    }
    
    pub fn set_pow(&mut self, increment: u32, miner_sk: YSecretKey, recipient_pk: YPublicKey, fee_pk: YPublicKey) -> YResult<()> {
        if miner_sk.to_public() == recipient_pk {
            return Err(YErrorKind::DuplicateItem.into());
        }
        if self.post.is_none() {
            return Err(YErrorKind::PoStNotFound.into());
        }
        
        let post = self.post.clone().unwrap();
        let mut pow = YPoW::new(post.digest, post.difficulty, increment)?;
        
        let cb_amount = self.coinbase_amount(increment)?;
        let mut fee_amount = YAmount::default();
        let mut miner_amount = cb_amount.clone();
        if cb_amount > YAmount::from_u64(100)? {
            fee_amount = cb_amount / YAmount::from_u64(100)?;
            miner_amount -= fee_amount.clone();
        }
        
        let height = 0;
        let miner_output = YOutput::new(&miner_sk, &recipient_pk, height, miner_amount, None)?;
        let fee_output = YOutput::new(&miner_sk, &fee_pk, height, fee_amount, None)?;
        self.outputs = vec![miner_output, fee_output];
        
        let msg = self.clone().to_pow_bytes()?;
        pow.mine(msg.as_slice())?;

        if pow.digest.is_none() {
            return Err(YErrorKind::PoWDigestNotFound.into());
        }
        
        self.pow = Some(pow);
        self.id = self.calc_id()?;
        Ok(())
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

    pub fn verify(&self) -> YResult<bool> {
        if self.id != self.calc_id()? {
            return Err(YErrorKind::InvalidChecksum.into());
        }

        if self.version.major() > YVersion::default().major() {
            let v = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(v).into());
        }
        
        let time = self.time.clone();
        let now = YTime::now();
        if time > now {
            return Err(YErrorKind::InvalidTime.into())
        }

        if let Some(_post) = self.post.clone() {
            if !_post.verify() {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        if let Some(_pow) = self.pow.clone() {
            if !_pow.verify()? {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        let mut tot_amount = YAmount::default();

        for output in self.outputs.clone() {
            if output.height != 0 {
                return Err(YErrorKind::InvalidHeight.into());
            }
            output.check()?;
            tot_amount += output.amount;
        }

        let memory = YAmount::from_u64(self.pow.clone().unwrap().memory()?)?;
        if tot_amount != memory {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn pre_pow_check(&self) -> YResult<()> {
        if self.version > YVersion::default() {
            let v = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(v).into());
        }
        
        let time = self.time.clone();
        let now = YTime::now();
        if time > now {
            return Err(YErrorKind::InvalidTime.into())
        }

        if let Some(_post) = self.post.clone() {
            _post.check()?
        } else {
            return Err(YErrorKind::PoStNotFound.into())
        }

        for output in self.outputs.clone() {
            if output.height != 0 {
                return Err(YErrorKind::InvalidHeight.into());
            }
            output.check()?;
        }

        Ok(())
    }

    pub fn check(&self) -> YResult<()> {
        if self.id != self.calc_id()? {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        if self.version > YVersion::default() {
            let v = self.version.to_string();
            return Err(YErrorKind::InvalidVersion(v).into());
        }
        
        let time = self.time.clone();
        let now = YTime::now();
        if time > now {
            return Err(YErrorKind::InvalidTime.into())
        }

        if let Some(_post) = self.post.clone() {
            _post.check()?
        } else {
            return Err(YErrorKind::PoStNotFound.into())
        }

        if let Some(_pow) = self.pow.clone() {
            _pow.check()?
        } else {
            return Err(YErrorKind::PoWNotFound.into())
        }

        let mut tot_amount = YAmount::default();

        for output in self.outputs.clone() {
            if output.height != 0 {
                return Err(YErrorKind::InvalidHeight.into());
            }
            output.check()?;
            tot_amount += output.amount;
        }

        let memory = YAmount::from_u64(self.pow.clone().unwrap().memory()?)?;
        if tot_amount != memory {
            return Err(YErrorKind::InvalidAmount.into());
        }

        Ok(())
    }

    pub fn mine(
        tx_id: YDigest64,
        diff: u32,
        nonce: u32,
        chunks: &Vec<u8>,
        increment: u32,
        miner_sk: YSecretKey,
        recipient_pk: YPublicKey,
        fee_pk: YPublicKey) -> YResult<(YCoinbase, u32)> {
       
        let mut tries = 0;

        loop {
            let mut cb = YCoinbase::new()?;
            cb.set_post(tx_id, diff, nonce, chunks)?;
            tries += 1;
            match cb.set_pow(increment, miner_sk, recipient_pk, fee_pk) {
                Ok(_) => {
                    return Ok((cb, tries));
                },
                Err(YError(YErrorKind::PoWNotFound, _)) => {
                    continue; 
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    } 

    pub fn mine_genesys(
        diff: u32,
        nonce: u32,
        chunks: &Vec<u8>,
        miner_sk: YSecretKey,
        recipient_pk: YPublicKey,
        fee_pk: YPublicKey) -> YResult<((YCoinbase, YTransaction), u32)> {

        let genesys_tx = YTransaction::new_genesys()?;
        let gen_tx_id = genesys_tx.id;
        
        let (genesys_cb, tries) = YCoinbase::mine(gen_tx_id, diff,
                                         nonce, chunks, 0,
                                         miner_sk, recipient_pk,
                                         fee_pk)?;

        Ok(((genesys_cb, genesys_tx), tries))
    } 
}
