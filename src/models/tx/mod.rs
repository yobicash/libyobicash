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
use amount::Amount;
use models::wallet::Wallet;
use models::signers::Signers;
use models::input::Input;
use models::output::Output;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Tx {
    pub id: Hash,
    pub time: DateTime<Utc>,
    pub version: Version,
    pub signers: Signers,
    pub inputs_len: u32,
    pub inputs: Vec<Input>,
    pub outputs_len: u32,
    pub outputs: Vec<Output>,
    pub fee: Amount,
    pub signatures_len: u32,
    pub signatures: Vec<Signature>,
}

impl Tx {
    pub fn new() -> Result<Self> {
        let version = Version::parse(VERSION)?;
        let signers = Signers::new()?;
        Ok(Tx {
            id: Hash::default(),
            time: Utc::now(),
            version: version,
            signers: signers,
            inputs_len: 0,
            inputs: Vec::new(),
            outputs_len: 0,
            outputs: Vec::new(),
            fee: Amount::zero(),
            signatures_len: 0,
            signatures: Vec::new(),
        })
    }

    pub fn check_time(&self) -> Result<()> {
        if self.time > Utc::now() {
            return Err(ErrorKind::InvalidTime.into())
        }
        Ok(())
    }

    pub fn check_version(&self) -> Result<()> {
        let v = Version::parse(VERSION)?;
        if self.version > v {
            return Err(ErrorKind::InvalidVersion.into());
        }
        Ok(())
    }

    pub fn check_signers(&self) -> Result<()> {
        self.signers.check()
    }

    pub fn check_inputs_len(&self) -> Result<()> {
        if self.inputs_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }    
        Ok(())
    }

    pub fn check_inputs(&self) -> Result<()> {
        if self.inputs.len() != self.inputs_len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        for i in 0..self.inputs_len as usize {
            self.inputs[i].check()?;
        }
        // NB: no check for inputs uniqueness. TODO or not TODO?
        // it requires a sorting algo, to be used also after every
        // add
        Ok(())
    }

    pub fn check_outputs_len(&self) -> Result<()> {
        if self.outputs_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }    
        Ok(())
    }

    pub fn check_outputs(&self) -> Result<()> {
        if self.outputs.len() != self.outputs_len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        for i in 0..self.outputs_len as usize {
            self.outputs[i].check()?;
        }
        // NB: no check for inputs uniqueness. TODO or not TODO?
        // it requires a sorting algo, to be used also after every
        // add
        Ok(())
    }

    pub fn check_tot_amount(&self, inputs_amount: &Amount) -> Result<()> {
        if self.tot_amount() != inputs_amount.to_owned() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn check_pre_checksum(&self) -> Result<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_signers()?;
        self.check_inputs_len()?;
        self.check_inputs()?;
        self.check_outputs_len()?;
        self.check_outputs()
    }

    pub fn check_signatures_len(&self) -> Result<()> {
        if self.signatures_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        if self.signatures_len != self.signatures.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_signatures(&self) -> Result<()> {
        let cksm = self.checksum()?;
        self.signers.check_signatures(&cksm, &self.signatures)
    }

    pub fn check_pre_id(&self) -> Result<()> {
        self.check_pre_checksum()?;
        self.check_signatures()
    }

    pub fn check_id(&self) -> Result<()> {
        if self.id != self.id()? {
            return Err(ErrorKind::InvalidId.into());
        }
        Ok(())
    }

    pub fn check(&self) -> Result<()> {
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

    pub fn add_input(&mut self, inp: &Input) -> Result<Self> {
        inp.check()?;
        self.check_inputs()?;
        for i in 0..self.inputs_len as usize {
            if self.inputs[i] == *inp {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }
        self.inputs_len += 1;
        self.inputs.push(inp.to_owned());
        Ok(self.to_owned())
    }

    pub fn add_output(&mut self, outp: &Output) -> Result<Self> {
        outp.check()?;
        self.check_outputs()?;
        for i in 0..self.outputs_len as usize {
            if self.outputs[i] == *outp {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }
        self.outputs_len += 1;
        self.outputs.push(outp.to_owned());
        Ok(self.to_owned())
    }

    pub fn outputs_amount(&self) -> Amount {
        let mut amount = Amount::zero();
        for i in 0..self.outputs_len as usize {
            amount = amount.to_owned() + self.outputs[i].get_amount();
        }
        amount
    }

    pub fn tot_amount(&self) -> Amount {
        self.outputs_amount() + self.fee.to_owned() 
    }

    pub fn sign(&mut self, w: &Wallet) -> Result<Self> {
        let checksum = self.checksum()?;
        if !self.signers.lookup_signer(&w.public_key)? {
            return Err(ErrorKind::NotFound.into());
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

    pub fn checksum(&self) -> Result<Hash> {
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

    pub fn id(&self) -> Result<Hash> {
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

    pub fn set_id(&mut self) -> Result<Self> {
        self.check_pre_id()?;
        self.id = self.id()?;
        Ok(self.to_owned())
    }

    pub fn segment_start_idx(&self, seed: &Hash) -> Result<u32> {
        self.check()?;
        check_hash_size(seed)?;
        let v = self.to_vec()?;
        let size = v.len() as u32;
        read_u32_from_seed(seed, size)
    }

    pub fn read_segment(&self, seed: &Hash) -> Result<Segment> {
        self.check()?;
        check_hash_size(seed)?;
        let v = self.to_vec()?;
        read_segment(seed, &v)
    }

    pub fn coinbase(w: &Wallet, to: &Signers, m: &Amount, data: &Vec<u8>) -> Result<Self> {
        to.check()?;
        let size = data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(ErrorKind::InvalidSize.into());
        }
        if Amount::new(size) != m.to_owned() {
            return Err(ErrorKind::InvalidSize.into());
        }
        let mut tx = Tx::new()?;
        let outp = Output::new(m, &to.get_address(), data)?;
        tx.add_output(&outp)?.sign(w)?;
        tx.check()?;
        if !tx.is_coinbase()? {
            return Err(ErrorKind::InvalidCoinbase.into());
        }
        Ok(tx)
    }

    pub fn is_coinbase(&self) -> Result<bool> {
        self.check()?;
        Ok(self.inputs_len == 0 &&
            self.outputs_len == 1 &&
            self.outputs[0].get_amount() != Amount::zero())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
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
