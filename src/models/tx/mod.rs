use byteorder::{BigEndian, WriteBytesExt};
use num_traits::Zero;
use itertools::Itertools;
use semver::Version;
use chrono::{DateTime, Utc};
use VERSION;
use errors::*;
use size::check_size;
use length::MAX_LEN;
use length::check_length;
use crypto::hash::*;
use crypto::sign::Signature;
use crypto::sign::sign;
use crypto::sign::check_signature_size;
use crypto::sign::check_unique_signatures;
use mining::por::Segment;
use mining::por::read_segment;
use models::amount::Amount;
use models::wallet::Wallet;
use models::signers::Signers;
use models::input::*;
use models::content::Content;
use models::output::*;
use models::outpoint::*;
use std::io::Write;
use std::iter::repeat;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Tx {
    id: Hash,
    time: DateTime<Utc>,
    version: Version,
    signers: Signers,
    inputs_len: u32,
    inputs: Vec<Input>,
    outputs_len: u32,
    outputs: Vec<Output>,
    fee: Amount,
    signatures_len: u32,
    signatures: Vec<Signature>,
}

impl Tx {
    pub fn new() -> Result<Self> {
        let version = Version::parse(VERSION)?;
        let signers = Signers::new()?;
        let id: Vec<u8> = repeat(0u8).take(HASH_SIZE).collect();
        Ok(Tx {
            id: id,
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

    pub fn get_time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn set_time(&mut self, time: &DateTime<Utc>) -> Result<Self> {
        if *time > Utc::now() {
            return Err(ErrorKind::InvalidTime.into())
        }
        self.time = time.to_owned();
        Ok(self.to_owned())
    }

    fn check_time(&self) -> Result<()> {
        if self.time > Utc::now() {
            return Err(ErrorKind::InvalidTime.into())
        }
        Ok(())
    }

    pub fn get_version(&self) -> Version {
        self.version.to_owned()
    }

    pub fn set_version(&mut self, version: &Version) -> Result<Self> {
        let v = Version::parse(VERSION)?;
        if *version > v {
            return Err(ErrorKind::InvalidVersion.into());
        }
        self.version = version.to_owned();
        Ok(self.to_owned())
    }

    fn check_version(&self) -> Result<()> {
        let v = Version::parse(VERSION)?;
        if self.version > v {
            return Err(ErrorKind::InvalidVersion.into());
        }
        Ok(())
    }

    pub fn get_signers(&self) -> Signers {
        self.signers.to_owned()
    }

    pub fn set_signers(&mut self, signers: &Signers) -> Result<Self> {
        signers.check()?;
        self.signers = signers.to_owned();
        Ok(self.to_owned())
    }

    fn check_signers(&self) -> Result<()> {
        self.signers.check()
    }

    pub fn get_inputs_len(&self) -> u32 {
        self.inputs_len
    }

    fn check_inputs_len(&self) -> Result<()> {
        if self.inputs_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }    
        Ok(())
    }

    pub fn get_inputs(&self) -> Vec<Input> {
        self.inputs.to_owned()
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

    fn check_inputs(&self) -> Result<()> {
        check_length(&self.inputs)?;
        let len = self.inputs.len();
        if len != self.inputs_len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        check_unique_inputs(&self.inputs)?;
        for i in 0..len as usize {
            self.inputs[i].check()?;
        }
        Ok(())
    }

    fn check_outputs_len(&self) -> Result<()> {
        if self.outputs_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn get_outputs_len(&self) -> u32 {
        self.outputs_len
    }

    pub fn get_outputs(&self) -> Vec<Output> {
        self.outputs.to_owned()
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

    pub fn get_outputs_amount(&self) -> Amount {
        let mut amount = Amount::zero();
        for i in 0..self.outputs_len as usize {
            amount = amount + self.outputs[i].get_amount();
        }
        amount
    }

    fn check_outputs(&self) -> Result<()> {
        check_length(&self.outputs)?;
        let len = self.outputs.len();
        if len != self.outputs_len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        check_unique_outputs(&self.outputs)?;
        for i in 0..len as usize {
            self.outputs[i].check()?;
        }
        Ok(())
    }

    pub fn get_fee(&self) -> Amount {
        self.fee.to_owned()
    }

    pub fn set_fee(&mut self, fee: &Amount) -> Self {
        self.fee = fee.to_owned();
        self.to_owned()
    }

    pub fn get_tot_amount(&self) -> Amount {
        self.get_outputs_amount() + self.fee.to_owned() 
    }

    pub fn check_balance(&self, inputs_amount: &Amount) -> Result<()> {
        if self.get_tot_amount() != inputs_amount.to_owned() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    fn check_pre_checksum(&self) -> Result<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_signers()?;
        self.check_inputs_len()?;
        self.check_inputs()?;
        self.check_outputs_len()?;
        self.check_outputs()
    }

    pub fn calc_checksum(&self) -> Result<Hash> {
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

    pub fn get_signatures_len(&self) -> u32 {
        self.signatures_len
    }

    fn check_signatures_len(&self) -> Result<()> {
        if self.signatures_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        if self.signatures_len != self.signatures.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn get_signatures(&self) -> Vec<Signature> {
        self.signatures.to_owned()
    }

    fn check_pre_sign(&self) -> Result<()> {
        self.check_pre_checksum()?;
        self.check_signatures_len()?;
        for i in 0..self.signatures_len as usize {
            check_signature_size(&self.signatures[i])?;
        }
        Ok(())
    }

    pub fn sign(&mut self, w: &Wallet) -> Result<Self> {
        self.check_pre_sign()?;
        let checksum = self.calc_checksum()?;
        if !self.signers.lookup_signer(&w.public_key)? {
            return Err(ErrorKind::NotFound.into());
        }
        let sig = sign(&checksum, &w.secret_key)?;
        for i in 0..self.signatures_len as usize {
            if sig == self.signatures[i] {
                return Ok(self.to_owned())
            }
        }
        self.signatures.push(sig);
        self.signatures_len += 1;
        Ok(self.to_owned())
    }

    pub fn verify_signatures(&self) -> Result<bool> {
        let cksm = self.calc_checksum()?;
        self.signers.verify_signatures(&cksm, &self.signatures)
    }

    pub fn check_signatures(&self) -> Result<()> {
        check_length(&self.signatures)?;
        check_unique_signatures(&self.signatures)?;
        for i in 0..self.signatures_len as usize {
            check_signature_size(&self.signatures[i])?;
        }
        let cksm = self.calc_checksum()?;
        self.signers.check_signatures(&cksm, &self.signatures)
    }

    fn check_pre_id(&self) -> Result<()> {
        self.check_pre_checksum()?;
        self.check_signatures()
    }

    pub fn get_id(&self) -> Hash {
        self.id.to_owned()
    }

    pub fn calc_id(&self) -> Result<Hash> {
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

    pub fn finalize(&mut self) -> Result<Self> {
        self.check_pre_id()?;
        self.id = self.calc_id()?;
        Ok(self.to_owned())
    }

    fn check_id(&self) -> Result<()> {
        if self.id != self.calc_id()? {
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
        self.check_signatures_len()?;
        self.check_signatures()?;
        self.check_id()
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

    pub fn read_segment(&self, seed: &Hash) -> Result<Segment> {
        self.check()?;
        check_hash_size(seed)?;
        let v = self.to_vec()?;
        read_segment(seed, &v)
    }

    pub fn coinbase(wallet: &Wallet, to: &Signers, amount: &Amount, data: &Vec<u8>) -> Result<Self> {
        to.check()?;
        check_size(data)?;
        let size = data.len() as u32;
        if size > 0 && Amount::new(size) != amount.to_owned() {
            return Err(ErrorKind::InvalidSize.into());
        }
        let signers = Signers::new()?
            .add_signer(&wallet.public_key, 1)?
            .finalize()?;
        let content = Content::new(&signers, data)?
            .sign(&wallet)?
            .finalize()?;
        let outp = Output::new(amount, &to.get_address(), &content)?;
        signers.check()?;
        let tx = Tx::new()?
            .add_output(&outp)?
            .set_signers(&signers)?
            .sign(wallet)?
            .finalize()?;
        tx.check()?;
        Ok(tx)
    }

    pub fn is_coinbase(&self) -> Result<bool> {
        self.check()?;
        let ok = self.signers.get_len() == 1 &&
            self.signers.get_threshold() == 0 &&
            self.inputs_len == 0 &&
            self.outputs_len == 1 &&
            self.outputs[0].get_amount() != Amount::zero();
        Ok(ok)
    }

    pub fn check_coinbase(&self) -> Result<()> {
        if !self.is_coinbase()? {
            return Err(ErrorKind::InvalidCoinbase.into())
        }
        Ok(())
    }

    pub fn check_doublespending(&self, outpoints: &Vec<OutPoint>) -> Result<()> {
        let _outpoints = OutPoints::from_vec(outpoints)?;
        _outpoints.check_unique()?;
        let mut _inputs = _outpoints.to_inputs()?;
        _inputs.sort();
        let mut inputs = self.inputs.to_owned();
        check_unique_inputs(&inputs)?;
        inputs.sort();
        if _inputs != inputs {
            return Err(ErrorKind::InvalidOutPoints.into());
        }
        let _outputs = Outputs::from_vec(&self.outputs)?;
        _outputs.check_unique()?;
        for outpoint in _outpoints.to_owned() {
            outpoint.check()?;
            let output = outpoint.get_output();
            if output.get_to() != self.signers.get_address() {
                return Err(ErrorKind::InvalidAddress.into());
            }
        }
        for output in _outputs.to_owned() {
            output.check()?;
        }
        let outpoints_amount = _outpoints.tot_amount();
        let tot_amount = self.get_tot_amount() + self.get_fee();
        if outpoints_amount != tot_amount {
            return Err(ErrorKind::InvalidAmount.into());
        }
        // 1) a = unique oupoints content ids
        // 2) b = all non own outputs content ids
        // 3) if len(a) < len(b), doublespending
        let mut outpoints_ids: Vec<Hash> = Vec::new();
        for outpoint in _outpoints.to_owned() {
            if let Some(content) = outpoint.get_output().get_content() {
                let content_id = content.get_id();
                outpoints_ids.push(content_id);
            }
        }
        outpoints_ids = unique_hashes(&outpoints_ids)?;
        let mut outputs_ids: Vec<Hash> = Vec::new();
        for output in _outputs.to_owned() {
            if let Some(content) = output.get_content() {
                if content.get_creators() != self.signers {
                    let content_id = content.get_id();
                    outputs_ids.push(content_id);
                }
            }
        }
        outputs_ids = unique_hashes(&outputs_ids)?;
        if outpoints_ids.len() < outputs_ids.len() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn from_outpoints(outpoints: &Vec<OutPoint>, outputs: &Vec<Output>, fee: &Amount, signers: &Signers) -> Result<Self> {
        let _outpoints = OutPoints::from_vec(outpoints)?;
        let _outputs = Outputs::from_vec(outputs)?;
        _outpoints.check_unique()?;
        _outputs.check_unique()?;
        for outpoint in _outpoints.to_owned() {
            outpoint.check()?;
            let output = outpoint.get_output();
            if output.get_to() != signers.get_address() {
                return Err(ErrorKind::InvalidAddress.into());
            }
        }
        for output in _outputs.to_owned() {
            output.check()?;
        }
        let outpoints_amount = _outpoints.tot_amount();
        let tot_amount = _outputs.tot_amount() + fee.to_owned();
        if outpoints_amount != tot_amount {
            return Err(ErrorKind::InvalidAmount.into());
        }
        // 1) a = unique oupoints content ids
        // 2) b = all non own outputs content ids
        // 3) if len(a) < len(b), doublespending
        let mut outpoints_ids: Vec<Hash> = Vec::new();
        for outpoint in _outpoints.to_owned() {
            if let Some(content) = outpoint.get_output().get_content() {
                let content_id = content.get_id();
                outpoints_ids.push(content_id);
            }
        }
        outpoints_ids = unique_hashes(&outpoints_ids)?;
        let mut outputs_ids: Vec<Hash> = Vec::new();
        for output in _outputs.to_owned() {
            if let Some(content) = output.get_content() {
                if content.get_creators() != *signers {
                    let content_id = content.get_id();
                    outputs_ids.push(content_id);
                }
            }
        }
        outputs_ids = unique_hashes(&outputs_ids)?;
        if outpoints_ids.len() < outputs_ids.len() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        let mut tx = Tx::new()?;
        for input in _outpoints.to_inputs()? {
            tx.add_input(&input)?;
        }
        for output in _outputs.to_owned() {
            tx.add_output(&output)?;
        }
        tx.set_signers(signers)?;
        Ok(tx)
    }
}

#[derive(Clone, Debug)]
pub struct Txs {
    length: u32,
    idx: u32,
    items: Vec<Tx>,
}

impl Txs {
    pub fn new() -> Txs {
        Txs {
            length: 0,
            idx: 0,
            items: Vec::new(),
        }
    }

    pub fn from_vec(items: &Vec<Tx>) -> Result<Txs> {
        check_length(items)?;
        let len = items.len();
        Ok(Txs {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Tx) {
        self.items.push(item);
        self.length += 1;
    }

    pub fn unique(&self) -> Vec<Tx> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<Tx> = self.unique();
        if uniques.len() != self.len() {
            return Err(ErrorKind::DuplicatedElements.into());
        }
        Ok(())
    }

    pub fn check(&self) -> Result<()> {
        let len = self.length;
        if self.idx >= len {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        if len != self.items.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }
}

impl Index<usize> for Txs {
    type Output = Tx;

    fn index(&self, idx: usize) -> &Tx {
        self.items.index(idx)
    }
}

impl Iterator for Txs {
    type Item = Tx;

    fn next(&mut self) -> Option<Tx> {
        match self.check() {
            Ok(_) => {
                let item = self.items[self.idx as usize].to_owned();
                self.idx += 1;
                Some(item)
            },
            Err(_) => { None },
        }
    }
}

pub fn unique_txs(txs: &Vec<Tx>) -> Result<Vec<Tx>> {
    Ok(Txs::from_vec(txs)?.unique().collect())
}

pub fn check_unique_txs(txs: &Vec<Tx>) -> Result<()> {
    let uniques: Vec<Tx> = unique_txs(txs)?;
    if uniques.len() != txs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
