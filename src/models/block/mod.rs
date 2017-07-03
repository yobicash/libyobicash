use byteorder::{BigEndian, WriteBytesExt};
use num_traits::Zero;
use semver::Version;
use chrono::{DateTime, Utc};
use VERSION;
use errors::*;
use length::MAX_LEN;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use mining::targetting::*;
use mining::por::*;
use mining::pow::*;
use amount::Amount;
use models::wallet::Wallet;
use models::signers::Signers;
use models::tx::Tx;
use std::io::Write;
use std::cmp;

// NB: the coinbase is spendable right after its emission
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: Hash,
    pub time: DateTime<Utc>,
    pub version: Version,
    pub height: u32,
    pub prev_id: Hash,
    pub prev_chain_amount: Amount,
    pub chain_amount: Amount,
    pub coinbase: Tx,
    pub tx_ids_len: u32,
    pub tx_ids: Vec<Hash>,
    pub s_cost: u32,
    pub t_cost: u32,
    pub delta: u32,
    pub bits: u32,
    pub segments_root: Hash,
    pub nonce: u32,
}

impl cmp::PartialOrd for Block {
    fn partial_cmp(&self, other: &Block) -> Option<cmp::Ordering> {
        self.chain_amount.partial_cmp(&other.chain_amount)
    }
}

impl cmp::Ord for Block {
    fn cmp(&self, other: &Block) -> cmp::Ordering {
        self.chain_amount.cmp(&other.chain_amount)
    }
}

impl Block {
    pub fn new() -> Result<Self> {
        let version = Version::parse(VERSION)?;
        let coinbase = Tx::new()?;
        Ok(Block {
            id: Hash::default(),
            time: Utc::now(),
            version: version,
            height: 0,
            prev_id: Hash::default(),
            prev_chain_amount: Amount::zero(),
            chain_amount: Amount::zero(),
            coinbase: coinbase,
            tx_ids_len: 0,
            tx_ids: Vec::new(),
            s_cost: MIN_S_COST,
            t_cost: MIN_T_COST,
            delta: MIN_DELTA,
            bits: MIN_BITS,
            segments_root: Hash::new(),
            nonce: 0,
        })
    }

    pub fn from_prev(prev: &Block, confirm_t: u32) -> Result<Self> {
        prev.check()?;
        let time = Utc::now();
        let version = Version::parse(VERSION)?;
        let coinbase = Tx::new()?;
        let height = prev.height + 1;
        let prev_id = prev.id.to_owned();
        let prev_chain_amount = prev.chain_amount.to_owned();
        let old_t = prev.time.timestamp() as u64;
        let new_t = time.timestamp() as u64;
        let bits = retarget_bits(prev.bits, old_t, new_t, confirm_t)?;
        Ok(Block {
            id: Hash::default(),
            time: time,
            version: version,
            height: height,
            prev_id: prev_id,
            prev_chain_amount: prev_chain_amount,
            chain_amount: Amount::zero(),
            coinbase: coinbase,
            tx_ids_len: 0,
            tx_ids: Vec::new(),
            s_cost: MIN_S_COST,
            t_cost: MIN_T_COST,
            delta: MIN_DELTA,
            bits: bits,
            segments_root: Hash::new(),
            nonce: 0,
        })
    }

    pub fn check_time(&self) -> Result<()> {
        if self.time > Utc::now() {
            return Err(ErrorKind::InvalidTime.into());
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

    pub fn check_prev_id(&self) -> Result<()> {
        check_hash_size(&self.prev_id)
    }

    pub fn check_prev_chain_amount(&self) -> Result<()> {
        if self.prev_chain_amount.to_owned() == Amount::zero() &&
            self.height != 0 {
            return Err(ErrorKind::InvalidAmount.into())
        }
        Ok(())
    }

    pub fn check_prev(&self, prev: &Block, confirm_t: u32) -> Result<()> {
        if self.height != prev.height + 1 {
            return Err(ErrorKind::InvalidPrevBlock.into());
        }
        if self.prev_id != prev.id {
            return Err(ErrorKind::InvalidPrevBlock.into());
        }
        if self.prev_chain_amount != prev.chain_amount {
            return Err(ErrorKind::InvalidPrevBlock.into());
        }
        let old_t = prev.time.timestamp() as u64;
        let new_t = self.time.timestamp() as u64;
        if self.bits != retarget_bits(prev.bits, old_t, new_t, confirm_t)? {
            return Err(ErrorKind::InvalidPrevBlock.into());
        }
        Ok(())
    }

    pub fn check_chain_amount(&self) -> Result<()> {
        if self.chain_amount.to_owned() == Amount::zero() {
            return Err(ErrorKind::InvalidAmount.into())
        }
        if self.chain_amount != self.prev_chain_amount.to_owned() + self.coinbase_amount()? {
            return Err(ErrorKind::InvalidAmount.into())
        }
        Ok(())
    }

    pub fn check_coinbase(&self) -> Result<()> {
        self.coinbase.check()?;
        if !self.coinbase.is_coinbase()? {
            return Err(ErrorKind::InvalidCoinbase.into())
        }
        if self.coinbase.get_tot_amount().to_owned()
            != self.coinbase_amount()?.to_owned() {
            return Err(ErrorKind::InvalidCoinbase.into())
        }
        Ok(())
    }

    pub fn check_tx_ids_len(&self) -> Result<()> {
        if self.tx_ids_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_tx_ids(&self) -> Result<()> {
        if self.tx_ids.len() != self.tx_ids_len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        for i in 0..self.tx_ids_len as usize {
            check_hash_size(&self.tx_ids[i])?;
        }
        Ok(())
    }

    pub fn check_s_cost(&self) -> Result<()> {
        check_s_cost(self.s_cost)
    }

    pub fn check_t_cost(&self) -> Result<()> {
        check_t_cost(self.t_cost)
    }

    pub fn check_delta(&self) -> Result<()> {
        check_delta(self.delta)
    }

    pub fn check_pre_segments_seed(&self) -> Result<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_prev_id()?;
        self.check_prev_chain_amount()?;
        self.check_chain_amount()?;
        self.check_coinbase()?;
        self.check_tx_ids_len()?;
        self.check_tx_ids()?;
        self.check_s_cost()?;
        self.check_t_cost()?;
        self.check_delta()
    }

    pub fn check_segments_root_size(&self) -> Result<()> {
        check_hash_size(&self.segments_root)
    }

    pub fn check_segments_root(&self, segs: &Vec<Segment>) -> Result<()> {
        check_hash_size(&self.segments_root)?;
        if !verify_segments_root(segs, &self.segments_root)? {
            return Err(ErrorKind::InvalidSegmentsRoot.into());
        }
        Ok(())
    }

    pub fn check_pre_seed(&self) -> Result<()> {
        self.check_pre_segments_seed()?;
        self.check_segments_root_size()
    }

    pub fn check_pow(&self) -> Result<()> {
        if !self.verify_pow()? {
            return Err(ErrorKind::InvalidPOW.into())
        }
        Ok(())
    }

    pub fn check_pre_id(&self) -> Result<()> {
        self.check_pre_seed()?;
        self.check_pow()
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
        self.check_prev_id()?;
        self.check_prev_chain_amount()?;
        self.check_chain_amount()?;
        self.check_coinbase()?;
        self.check_tx_ids_len()?;
        self.check_tx_ids()?;
        self.check_s_cost()?;
        self.check_t_cost()?;
        self.check_delta()?;
        self.check_segments_root_size()?;
        self.check_pow()?;
        self.check_id()
    }

    pub fn segments_seed(&self) -> Result<Hash> {
        self.check_pre_segments_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.to_vec().as_slice())?;
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
        bin.write_all(self.prev_chain_amount.to_vec().as_slice())?;
        bin.write_all(self.chain_amount.to_vec().as_slice())?;
        bin.write_all(self.coinbase.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.tx_ids_len)?;
        for i in 0..self.tx_ids_len as usize {
            bin.write_all(self.tx_ids[i].to_vec().as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_u32::<BigEndian>(self.bits)?;
        let h = hash(bin.as_slice())?;
        Ok(h)
    }

    pub fn seed(&self) -> Result<Hash> {
        self.check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
        bin.write_all(self.prev_chain_amount.to_vec().as_slice())?;
        bin.write_all(self.chain_amount.to_vec().as_slice())?;
        bin.write_all(self.coinbase.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.tx_ids_len)?;
        for i in 0..self.tx_ids_len as usize {
            bin.write_all(self.tx_ids[i].to_vec().as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_u32::<BigEndian>(self.bits)?;
        bin.write_all(self.segments_root.to_vec().as_slice())?;
        hash(bin.as_slice())
    }

    pub fn id(&self) -> Result<Hash> {
        self.check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
        bin.write_all(self.prev_chain_amount.to_vec().as_slice())?;
        bin.write_all(self.chain_amount.to_vec().as_slice())?;
        bin.write_all(self.coinbase.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.tx_ids_len)?;
        for i in 0..self.tx_ids_len as usize {
            bin.write_all(self.tx_ids[i].to_vec().as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_u32::<BigEndian>(self.bits)?;
        bin.write_all(self.segments_root.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.nonce)?;
        hash(bin.as_slice())
    }

    pub fn set_id(&mut self) -> Result<Self> {
        self.check_pre_id()?;
        self.id = self.id()?;
        Ok(self.to_owned())
    }
    
    pub fn coinbase_amount(&self) -> Result<Amount> {
        let mem = balloon_memory(self.s_cost, self.t_cost, self.delta)?;
        Ok(Amount::new(mem))
    }
    
    pub fn set_coinbase(&mut self, w: &Wallet, to: &Signers, data: &Vec<u8>) -> Result<Self> {
        self.coinbase = Tx::coinbase(w, to, &self.coinbase_amount()?, data)?;
        Ok(self.to_owned())
    }

    pub fn add_tx_id(&mut self, tx_id: &Hash) -> Result<Self> {
        self.check_tx_ids()?;
        for i in 0..self.tx_ids_len as usize {
            if self.tx_ids[i] == *tx_id {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }
        self.tx_ids_len += 1;
        self.tx_ids.push(tx_id.to_owned());
        Ok(self.to_owned())
    }

    pub fn set_s_cost(&mut self, s_cost: u32) -> Result<Self> {
        check_s_cost(s_cost)?;
        self.s_cost = s_cost;
        Ok(self.to_owned())
    } 

    pub fn set_t_cost(&mut self, t_cost: u32) -> Result<Self> {
        check_t_cost(t_cost)?;
        self.t_cost = t_cost;
        Ok(self.to_owned())
    } 

    pub fn set_delta(&mut self, delta: u32) -> Result<Self> {
        check_delta(delta)?;
        self.delta = delta;
        Ok(self.to_owned())
    } 

    pub fn set_bits(&mut self, old_bits: u32, old_t: u64, confirm_t: u32) -> Result<Self> {
        check_target_bits(old_bits)?;
        let new_t = self.time.timestamp() as u64;
        self.bits = retarget_bits(old_bits, old_t, new_t, confirm_t)?; 
        Ok(self.to_owned())
    } 

    pub fn get_segments_blocks(&self) -> Result<Vec<u32>> {
        let seed = self.segments_seed()?;
        segments_idxs(&seed, self.bits, self.height)
    }

    pub fn get_segments_tx_ids(&self, seed: &Hash, bits: u32) -> Result<Vec<u32>> {
        segments_idxs(seed, bits, self.tx_ids_len)
    }

    pub fn set_segments_root(&mut self, segs: &Vec<Segment>) -> Result<Self> {
        self.segments_root = segments_root(segs)?;
        Ok(self.to_owned())
    }

    pub fn verify_segments_root(&self, segs: &Vec<Segment>) -> Result<bool> {
        verify_segments_root(segs, &self.segments_root)
    }

    pub fn mine(&mut self) -> Result<Self> {
        let s = self.seed()?;
        if let Some(nonce) = balloon_mine(self.bits, &s, self.s_cost, self.t_cost, self.delta)? {
            self.nonce = nonce;
        } else {
            return Err(ErrorKind::NotFound.into());
        }
        Ok(self.to_owned())
    }

    pub fn target(&self) -> Result<Vec<u8>> {
       target_from_bits(self.bits) 
    }

    pub fn verify_pow(&self) -> Result<bool> {
        let s = self.seed()?;
        balloon_verify(self.bits, &s, self.nonce, self.s_cost, self.t_cost, self.delta)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.to_vec().as_slice())?;
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
        bin.write_all(self.prev_chain_amount.to_vec().as_slice())?;
        bin.write_all(self.chain_amount.to_vec().as_slice())?;
        bin.write_all(self.coinbase.to_vec()?.as_slice())?;
        bin.write_u32::<BigEndian>(self.tx_ids_len)?;
        for i in 0..self.tx_ids_len as usize {
            bin.write_all(self.tx_ids[i].to_vec().as_slice())?;
        }
        bin.write_u32::<BigEndian>(self.s_cost)?;
        bin.write_u32::<BigEndian>(self.t_cost)?;
        bin.write_u32::<BigEndian>(self.delta)?;
        bin.write_u32::<BigEndian>(self.bits)?;
        bin.write_all(self.segments_root.to_vec().as_slice())?;
        bin.write_u32::<BigEndian>(self.nonce)?;
        Ok(bin)
    }

    pub fn select(new: &Block, old: &Block) -> Block {
        if new > old {
            new.to_owned()
        } else {
            old.to_owned()
        }
    }
}
