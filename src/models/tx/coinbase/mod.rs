use byteorder::{BigEndian, WriteBytesExt};
use num_traits::Zero;
use itertools::Itertools;
use semver::Version;
use chrono::{DateTime, Utc};
use VERSION;
use errors::*;
use length::MAX_LEN;
use length::check_length;
use crypto::hash::*;
use crypto::sign::Signature;
use crypto::sign::sign;
use crypto::sign::check_signature_size;
use crypto::sign::check_unique_signatures;
use mining::pow::*;
use models::amount::Amount;
use models::wallet::Wallet;
use models::signers::Signers;
use models::height::COINBASE_HEIGHT;
use models::output::*;
use models::outpoint::*;
use std::io::Write;
use std::iter::repeat;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct CoinbaseTx {
    id: Hash,
    time: DateTime<Utc>,
    version: Version,
    height: u32,
    signers: Signers,
    s_cost: u32,
    t_cost: u32,
    delta: u32,
    amount: Amount,
    outputs_len: u32,
    outputs: Vec<Output>,
    coins: Hash,
    signatures_len: u32,
    signatures: Vec<Signature>,
}

impl CoinbaseTx {
    pub fn new() -> Result<Self> {
        let version = Version::parse(VERSION)?;
        let signers = Signers::new()?;
        let s_cost = MIN_S_COST;
        let t_cost = MIN_T_COST;
        let delta = MIN_DELTA;
        let amount = Amount::new(balloon_memory(s_cost, t_cost, delta)?);
        let base_hash: Vec<u8> = repeat(0u8).take(HASH_SIZE).collect();
        let coins = base_hash.to_owned();
        let id = base_hash.to_owned();
        Ok(CoinbaseTx {
            id: id,
            time: Utc::now(),
            version: version,
            height: COINBASE_HEIGHT,
            signers: signers,
            s_cost: s_cost,
            t_cost: t_cost,
            delta: delta,
            amount: amount,
            outputs_len: 0,
            outputs: Vec::new(),
            coins: coins,
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
    
    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) -> Result<Self> {
        if height < 1 {
            return Err(ErrorKind::InvalidHeight.into())
        }
        self.height = height;
        Ok(self.to_owned())
    }

    pub fn check_height(&self) -> Result<()> {
        if self.height != COINBASE_HEIGHT {
            return Err(ErrorKind::InvalidHeight.into())
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

    pub fn get_s_cost(&self) -> u32 {
        self.s_cost
    }

    pub fn set_s_cost(&mut self, s_cost: u32) -> Result<Self> {
        check_s_cost(s_cost)?;
        self.s_cost = s_cost;
        self.amount = self.calc_amount()?;
        self.coins = self.calc_coins()?;
        Ok(self.to_owned())
    }

    fn check_s_cost(&self) -> Result<()> {
        check_s_cost(self.s_cost)
    }

    pub fn get_t_cost(&self) -> u32 {
        self.t_cost
    }

    pub fn set_t_cost(&mut self, t_cost: u32) -> Result<Self> {
        check_t_cost(t_cost)?;
        self.t_cost = t_cost;
        self.amount = self.calc_amount()?;
        self.coins = self.calc_coins()?;
        Ok(self.to_owned())
    }

    fn check_t_cost(&self) -> Result<()> {
        check_t_cost(self.t_cost)
    }

    pub fn get_delta(&self) -> u32 {
        self.delta
    }

    pub fn set_delta(&mut self, delta: u32) -> Result<Self> {
        check_delta(delta)?;
        self.delta = delta;
        self.amount = self.calc_amount()?;
        self.coins = self.calc_coins()?;
        Ok(self.to_owned())
    }

    fn check_delta(&self) -> Result<()> {
        check_delta(self.delta)
    }

    pub fn get_amount(&self) -> Amount {
        self.amount.to_owned()
    }

    pub fn calc_amount(&self) -> Result<Amount> {
        let mem = balloon_memory(self.s_cost, self.t_cost, self.delta)?;
        Ok(Amount::new(mem))
    }

    pub fn set_amount(&mut self) -> Result<Self> {
        self.amount = self.calc_amount()?;
        Ok(self.to_owned())
    }

    fn check_amount(&self) -> Result<()> {
        if self.amount.to_owned() == Amount::zero() {
            return Err(ErrorKind::InvalidAmount.into())
        }
        if self.amount != self.calc_amount()? {
            return Err(ErrorKind::InvalidAmount.into())
        }
        Ok(())
    }

    fn check_outputs_len(&self) -> Result<()> {
        if self.outputs_len == 0 {
            return Err(ErrorKind::InvalidLength.into());
        }
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

    pub fn get_output(&self, idx: u32) -> Result<Output> {
        if idx > self.outputs_len - 1 {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        let output = self.outputs[idx as usize].to_owned();
        Ok(output)
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
        if self.get_outputs_amount() > self.get_amount() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn check_outputs_content(&self) -> Result<()> {
        self.check_outputs()?;
        let signers = self.get_signers();
        for i in 0..self.outputs_len as usize {
            let output = self.outputs[i].to_owned();
            if let Some(content) = output.get_content() {
                if content.get_creators() != signers {
                    return Err(ErrorKind::InvalidContent.into());
                }
            }
        }
        Ok(())
    }

    pub fn check_balance(&self) -> Result<()> {
        if self.get_outputs_amount() != self.get_amount() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(())
    }

    fn check_pre_checksum(&self) -> Result<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_height()?;
        self.check_signers()?;
        self.check_s_cost()?;
        self.check_t_cost()?;
        self.check_delta()?;
        self.check_amount()?;
        self.check_outputs_len()?;
        self.check_outputs()?;
        self.check_outputs_content()
    }

    pub fn calc_checksum(&self) -> Result<Hash> {
        self.check_pre_checksum()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_all(self.amount.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        hash(bin.as_slice())
    }

    pub fn get_coins(&self) -> Hash {
        self.coins.to_owned()
    }

    pub fn calc_coins(&self) -> Result<Hash> {
        let seed = self.calc_checksum()?;
        let nonce = hash(&seed)?;
        balloon_hash(&seed, &nonce, self.s_cost, self.t_cost, self.delta)
    }

    pub fn set_coins(&mut self) -> Result<Self> {
        self.coins = self.calc_coins()?;
        Ok(self.to_owned())
    }

    pub fn check_coins(&self) -> Result<()> {
        if self.coins != self.calc_coins()? {
            return Err(ErrorKind::InvalidCoins.into())
        }
        Ok(())
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
        self.check_coins()?;
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
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_all(self.amount.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        bin.write_all(self.coins.as_slice())?;
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
        self.check_height()?;
        self.check_signers()?;
        self.check_s_cost()?;
        self.check_t_cost()?;
        self.check_delta()?;
        self.check_amount()?;
        self.check_outputs_len()?;
        self.check_outputs()?;
        self.check_outputs_content()?;
        self.check_coins()?;
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
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.signers.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_all(self.amount.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.outputs_len)?;
        for i in 0..self.outputs_len as usize {
            bin.write_all(self.outputs[i].to_vec()?.as_slice())?;
        }
        bin.write_all(self.coins.as_slice())?;
        bin.write_u32::<BigEndian>(self.signatures_len)?;
        for i in 0..self.signatures_len as usize {
            bin.write_all(self.signatures[i].to_vec().as_slice())?;
        }
        Ok(bin)
    }

    pub fn get_outpoint(&self, idx: u32) -> Result<OutPoint> {
        self.check()?;
        if idx > self.outputs_len - 1 {
            return Err(ErrorKind::IndexOutOfRange.into());
        }
        let id = self.get_id();
        let height = self.get_height();
        let output = self.get_output(idx)?;
        OutPoint::new(&id, height, idx, &output)
    }

    pub fn get_outpoints(&self) -> Result<Vec<OutPoint>> {
        let mut outpoints: Vec<OutPoint> = Vec::new();
        for i in 0..self.outputs_len {
            let outpoint = self.get_outpoint(i)?;
            outpoints.push(outpoint);
        }
        Ok(outpoints)
    }

}

#[derive(Clone, Debug)]
pub struct CoinbaseTxs {
    length: u32,
    idx: u32,
    items: Vec<CoinbaseTx>,
}

impl CoinbaseTxs {
    pub fn new() -> CoinbaseTxs {
        CoinbaseTxs {
            length: 0,
            idx: 0,
            items: Vec::new(),
        }
    }

    pub fn from_vec(items: &Vec<CoinbaseTx>) -> Result<CoinbaseTxs> {
        check_length(items)?;
        let len = items.len();
        Ok(CoinbaseTxs {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: CoinbaseTx) {
        self.items.push(item);
        self.length += 1;
    }

    pub fn unique(&self) -> Vec<CoinbaseTx> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<CoinbaseTx> = self.unique();
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

impl Index<usize> for CoinbaseTxs {
    type Output = CoinbaseTx;

    fn index(&self, idx: usize) -> &CoinbaseTx {
        self.items.index(idx)
    }
}

impl Iterator for CoinbaseTxs {
    type Item = CoinbaseTx;

    fn next(&mut self) -> Option<CoinbaseTx> {
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

pub fn unique_cointxs(txs: &Vec<CoinbaseTx>) -> Result<Vec<CoinbaseTx>> {
    Ok(CoinbaseTxs::from_vec(txs)?.unique().collect())
}

pub fn check_unique_cointxs(txs: &Vec<CoinbaseTx>) -> Result<()> {
    let uniques: Vec<CoinbaseTx> = unique_cointxs(txs)?;
    if uniques.len() != txs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
