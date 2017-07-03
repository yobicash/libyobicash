use byteorder::{BigEndian, WriteBytesExt};
use num_traits::Zero;
use semver::Version;
use chrono::{DateTime, Utc};
use VERSION;
use errors::*;
use length::MAX_LEN;
use size::MAX_SIZE;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use crypto::sign::Signature;
use crypto::sign::sign;
use mining::por::Segment;
use mining::por::read_u32_from_seed;
use mining::por::read_segment;
use amount::YAmount;
use models::wallet::YWallet;
use models::signers::YSigners;
use models::input::YInput;
use models::output::YOutput;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct YTx {
    pub id: Hash,
    pub time: DateTime<Utc>,
    pub version: Version,
    pub signers: YSigners,
    pub inputs_len: u32,
    pub inputs: Vec<YInput>,
    pub outputs_len: u32,
    pub outputs: Vec<YOutput>,
    pub fee: YAmount,
    pub signatures_len: u32,
    pub signatures: Vec<Signature>,
}

impl YTx {
    pub fn new() -> YResult<Self> {
        let version = Version::parse(VERSION)?;
        let signers = YSigners::new()?;
        Ok(YTx {
            id: Hash::default(),
            time: Utc::now(),
            version: version,
            signers: signers,
            inputs_len: 0,
            inputs: Vec::new(),
            outputs_len: 0,
            outputs: Vec::new(),
            fee: YAmount::zero(),
            signatures_len: 0,
            signatures: Vec::new(),
        })
    }

    pub fn check_time(&self) -> YResult<()> {
        if self.time > Utc::now() {
            return Err(YErrorKind::InvalidTime.into())
        }
        Ok(())
    }

    pub fn check_version(&self) -> YResult<()> {
        let v = Version::parse(VERSION)?;
        if self.version > v {
            return Err(YErrorKind::InvalidVersion.into());
        }
        Ok(())
    }

    pub fn check_signers(&self) -> YResult<()> {
        self.signers.check()
    }

    pub fn check_inputs_len(&self) -> YResult<()> {
        if self.inputs_len > MAX_LEN as u32 {
            return Err(YErrorKind::InvalidLength.into());
        }    
        Ok(())
    }

    pub fn check_inputs(&self) -> YResult<()> {
        if self.inputs.len() != self.inputs_len as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        for i in 0..self.inputs_len as usize {
            self.inputs[i].check()?;
        }
        Ok(())
    }

    pub fn check_outputs_len(&self) -> YResult<()> {
        if self.outputs_len > MAX_LEN as u32 {
            return Err(YErrorKind::InvalidLength.into());
        }    
        Ok(())
    }

    pub fn check_outputs(&self) -> YResult<()> {
        if self.outputs.len() != self.outputs_len as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        for i in 0..self.outputs_len as usize {
            self.outputs[i].check()?;
        }
        Ok(())
    }

    pub fn check_tot_amount(&self, inputs_amount: &YAmount) -> YResult<()> {
        if self.tot_amount() != inputs_amount.to_owned() {
            return Err(YErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn check_pre_checksum(&self) -> YResult<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_signers()?;
        self.check_inputs_len()?;
        self.check_inputs()?;
        self.check_outputs_len()?;
        self.check_outputs()
    }

    pub fn check_signatures_len(&self) -> YResult<()> {
        if self.signatures_len > MAX_LEN as u32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        if self.signatures_len != self.signatures.len() as u32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_signatures(&self) -> YResult<()> {
        let cksm = self.checksum()?;
        self.signers.check_signatures(&cksm, &self.signatures)
    }

    pub fn check_pre_id(&self) -> YResult<()> {
        self.check_pre_checksum()?;
        self.check_signatures()
    }

    pub fn check_id(&self) -> YResult<()> {
        if self.id != self.id()? {
            return Err(YErrorKind::InvalidId.into());
        }
        Ok(())
    }

    pub fn check(&self) -> YResult<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_signers()?;
        self.check_inputs_len()?;
        self.check_inputs()?;
        self.check_outputs_len()?;
        self.check_outputs()?;
        self.check_signatures()?;
        self.check_id()
    }

    pub fn checksum(&self) -> YResult<Hash> {
        self.check_pre_checksum()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.inputs_len)?;
        for i in 0..self.inputs_len as usize {
            bin.write_all(self.inputs[i].to_vec()?.as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        bin.write_all(self.fee.to_vec().as_slice())?;
        hash(bin.as_slice())
    }

    pub fn id(&self) -> YResult<Hash> {
        self.check_pre_id()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.inputs_len)?;
        for i in 0..self.inputs_len as usize {
            bin.write_all(self.inputs[i].to_vec()?.as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        bin.write_all(self.fee.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.signatures_len)?;
        for i in 0..self.signatures_len as usize {
            bin.write_all(self.signatures[i].to_vec().as_slice())?;
        }
        hash(bin.as_slice())
    }

    pub fn set_id(&mut self) -> YResult<Self> {
        self.check_pre_id()?;
        self.id = self.id()?;
        Ok(self.to_owned())
    }

    pub fn add_input(&mut self, inp: &YInput) -> YResult<Self> {
        self.check_inputs()?;
        for i in 0..self.inputs_len as usize {
            if self.inputs[i] == *inp {
                return Err(YErrorKind::AlreadyFound.into());
            }
        }
        self.inputs_len += 1;
        self.inputs.push(inp.to_owned());
        Ok(self.to_owned())
    }

    pub fn add_output(&mut self, outp: &YOutput) -> YResult<Self> {
        self.check_outputs()?;
        for i in 0..self.outputs_len as usize {
            if self.outputs[i] == *outp {
                return Err(YErrorKind::AlreadyFound.into());
            }
        }
        self.outputs_len += 1;
        self.outputs.push(outp.to_owned());
        Ok(self.to_owned())
    }

    pub fn outputs_amount(&self) -> YAmount {
        let mut amount = YAmount::zero();
        for i in 0..self.outputs_len as usize {
            amount = amount.to_owned() + self.outputs[i].amount.to_owned();
        }
        amount
    }

    pub fn tot_amount(&self) -> YAmount {
        self.outputs_amount() + self.fee.to_owned() 
    }

    pub fn sign(&mut self, w: &YWallet) -> YResult<Self> {
        let checksum = self.checksum()?;
        if !self.signers.lookup_signer(&w.public_key)? {
            return Err(YErrorKind::NotFound.into());
        }
        let sig = sign(&checksum, &w.secret_key)?;
        for i in 0..self.signatures_len as usize {
            if sig == self.signatures[i] {
                // NB: making signing idempotent
                return Ok(self.to_owned())
            }
        }
        self.signatures.push(sig);
        Ok(self.to_owned())
    }

    pub fn segment_start_idx(&self, seed: &Hash) -> YResult<u32> {
        self.check()?;
        check_hash_size(seed)?;
        let v = self.to_vec()?;
        let size = v.len() as u32;
        read_u32_from_seed(seed, size)
    }

    pub fn read_segment(&self, seed: &Hash) -> YResult<Segment> {
        self.check()?;
        check_hash_size(seed)?;
        let v = self.to_vec()?;
        read_segment(seed, &v)
    }

    pub fn coinbase(w: &YWallet, to: &YSigners, m: &YAmount, data: &Vec<u8>) -> YResult<Self> {
        to.check()?;
        let size = data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(YErrorKind::InvalidSize.into());
        }
        if YAmount::new(size) != m.to_owned() {
            return Err(YErrorKind::InvalidSize.into());
        }
        let mut tx = YTx::new()?;
        let outp = YOutput::new(m, &to.address, data)?;
        tx.add_output(&outp)?.sign(w)?;
        tx.check()?;
        if !tx.is_coinbase()? {
            return Err(YErrorKind::InvalidCoinbase.into());
        }
        Ok(tx)
    }

    pub fn is_coinbase(&self) -> YResult<bool> {
        self.check()?;
        Ok(self.inputs_len == 0 &&
            self.outputs_len == 1 &&
            self.outputs[0].amount.to_owned() != YAmount::zero())
    }

    pub fn to_vec(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.to_vec().as_slice())?;
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.inputs_len)?;
        for i in 0..self.inputs_len as usize {
            bin.write_all(self.inputs[i].to_vec()?.as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        bin.write_all(self.fee.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.signatures_len)?;
        for i in 0..self.signatures_len as usize {
            bin.write_all(self.signatures[i].to_vec().as_slice())?;
        }
        Ok(bin)
    }
}
