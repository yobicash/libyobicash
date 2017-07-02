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
use mining::target::*;
use mining::por::*;
use mining::pow::*;
use amount::YAmount;
use models::wallet::YWallet;
use models::signers::YSigners;
use models::tx::YTx;
use std::io::Write;
use std::cmp;

// NB: the coinbase is spendable right after its emission
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct YBlock {
    pub id: Hash,
    pub time: DateTime<Utc>,
    pub version: Version,
    pub height: u32,
    pub prev_id: Hash,
    pub chain_amount: YAmount,
    pub coinbase: YTx,
    pub tx_ids_len: u32,
    pub tx_ids: Vec<Hash>,
    pub s_cost: u32,
    pub t_cost: u32,
    pub delta: u32,
    pub bits: u32,
    pub segments_root: Hash,
    pub nonce: u32,
}

impl cmp::PartialOrd for YBlock {
    fn partial_cmp(&self, other: &YBlock) -> Option<cmp::Ordering> {
        self.chain_amount.partial_cmp(&other.chain_amount)
    }
}

impl cmp::Ord for YBlock {
    fn cmp(&self, other: &YBlock) -> cmp::Ordering {
        self.chain_amount.cmp(&other.chain_amount)
    }
}

impl YBlock {
    pub fn new() -> YResult<Self> {
        let version = Version::parse(VERSION)?;
        let coinbase = YTx::new()?;
        Ok(YBlock {
            id: Hash::default(),
            time: Utc::now(),
            version: version,
            height: 0,
            prev_id: Hash::default(),
            chain_amount: YAmount::zero(),
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

    pub fn check_time(&self) -> YResult<()> {
        if self.time > Utc::now() {
            return Err(YErrorKind::InvalidTime.into());
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

    pub fn check_prev_id(&self) -> YResult<()> {
        check_hash_size(&self.prev_id)
    }

    pub fn check_chain_amount(&self) -> YResult<()> {
        if self.chain_amount.clone() == YAmount::zero() {
            return Err(YErrorKind::InvalidAmount.into())
        }
        Ok(())
    }

    pub fn check_coinbase(&self) -> YResult<()> {
        self.coinbase.check()?;
        if !self.coinbase.is_coinbase()? {
            return Err(YErrorKind::InvalidCoinbase.into())
        }
        if self.coinbase.tot_amount().clone()
            != self.coinbase_amount()?.clone() {
            return Err(YErrorKind::InvalidCoinbase.into())
        }
        Ok(())
    }

    pub fn check_tx_ids_len(&self) -> YResult<()> {
        if self.tx_ids_len > MAX_LEN as u32 {
            return Err(YErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_tx_ids(&self) -> YResult<()> {
        if self.tx_ids.len() != self.tx_ids_len as usize {
            return Err(YErrorKind::InvalidLength.into());
        }
        for i in 0..self.tx_ids_len as usize {
            check_hash_size(&self.tx_ids[i])?;
        }
        Ok(())
    }

    pub fn check_s_cost(&self) -> YResult<()> {
        check_s_cost(self.s_cost)
    }

    pub fn check_t_cost(&self) -> YResult<()> {
        check_t_cost(self.t_cost)
    }

    pub fn check_delta(&self) -> YResult<()> {
        check_delta(self.delta)
    }

    fn _check_pre_segments_seed(&self) -> YResult<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_prev_id()?;
        self.check_chain_amount()?;
        self.check_coinbase()?;
        self.check_tx_ids_len()?;
        self.check_tx_ids()?;
        self.check_s_cost()?;
        self.check_t_cost()?;
        self.check_delta()
    }

    fn _segments_seed(&self) -> YResult<Hash> {
        self._check_pre_segments_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.to_vec().as_slice())?;
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
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

    pub fn check_segments_root_size(&self) -> YResult<()> {
        check_hash_size(&self.segments_root)
    }

    pub fn check_segments_root(&self, segs: &Vec<Segment>) -> YResult<()> {
        check_hash_size(&self.segments_root)?;
        check_segments_root(segs, &self.segments_root)
    }

    fn _check_pre_seed(&self) -> YResult<()> {
        self._check_pre_segments_seed()?;
        self.check_segments_root_size()
    }

    fn _seed(&self) -> YResult<Hash> {
        self._check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
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

    pub fn check_pow(&self) -> YResult<()> {
        if !self.verify_mining()? {
            return Err(YErrorKind::InvalidPOW.into())
        }
        Ok(())
    }

    fn _check_pre_id(&self) -> YResult<()> {
        self._check_pre_seed()?;
        self.check_pow()
    }

    fn _id(&self) -> YResult<Hash> {
        self._check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
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

    pub fn set_id(&mut self) -> YResult<Self> {
        self._check_pre_id()?;
        self.id = self._id()?;
        Ok(self.to_owned())
    }

    pub fn check_id(&self) -> YResult<()> {
        if self.id != self._id()? {
            return Err(YErrorKind::InvalidId.into());
        }
        Ok(())
    }

    pub fn check(&self) -> YResult<()> {
        self.check_time()?;
        self.check_version()?;
        self.check_prev_id()?;
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
    
    pub fn coinbase_amount(&self) -> YResult<YAmount> {
        let mem = balloon_memory(self.s_cost, self.t_cost, self.delta);
        Ok(YAmount::new(mem))
    }
    
    pub fn set_coinbase(&mut self, w: &YWallet, to: &YSigners, data: &Vec<u8>) -> YResult<Self> {
        self.coinbase = YTx::coinbase(w, to, &self.coinbase_amount()?, data)?;
        Ok(self.to_owned())
    }

    pub fn add_tx_id(&mut self, tx_id: &Hash) -> YResult<Self> {
        self.check_tx_ids()?;
        for i in 0..self.tx_ids_len as usize {
            if self.tx_ids[i] == *tx_id {
                return Err(YErrorKind::AlreadyFound.into());
            }
        }
        self.tx_ids_len += 1;
        self.tx_ids.push(tx_id.to_owned());
        Ok(self.to_owned())
    }

    pub fn set_s_cost(&mut self, s_cost: u32) -> YResult<Self> {
        check_s_cost(s_cost)?;
        self.s_cost = s_cost;
        Ok(self.to_owned())
    } 

    pub fn set_t_cost(&mut self, t_cost: u32) -> YResult<Self> {
        check_t_cost(t_cost)?;
        self.t_cost = t_cost;
        Ok(self.to_owned())
    } 

    pub fn set_delta(&mut self, delta: u32) -> YResult<Self> {
        check_delta(delta)?;
        self.delta = delta;
        Ok(self.to_owned())
    } 

    pub fn set_bits(&mut self, old_bits: u32, old_t: DateTime<Utc>, confirm_t: u32) -> YResult<Self> {
        check_target_bits(old_bits)?;
        self.bits = retarget_bits(old_bits, old_t, self.time, confirm_t)?; 
        Ok(self.to_owned())
    } 

    pub fn get_segments_blocks(&self) -> YResult<Vec<u32>> {
        let seed = self._segments_seed()?;
        segments_idxs(&seed, self.bits, self.height)
    }

    pub fn get_segments_tx_ids(&self, seed: &Hash, bits: u32) -> YResult<Vec<u32>> {
        segments_idxs(seed, bits, self.tx_ids_len)
    }

    pub fn set_segments_root(&mut self, segs: &Vec<Segment>) -> YResult<Self> {
        self.segments_root = segments_root(segs)?;
        Ok(self.to_owned())
    }

    pub fn verify_segments_root(&self, segs: &Vec<Segment>) -> YResult<bool> {
        verify_segments_root(segs, &self.segments_root)
    }

    pub fn mine(&mut self) -> YResult<Self> {
        let t = self._target()?;
        let s = self._seed()?;
        if let Some(nonce) = balloon_mine(&t, &s, self.s_cost, self.t_cost, self.delta)? {
            self.nonce = nonce;
        } else {
            return Err(YErrorKind::NotFound.into());
        }
        Ok(self.to_owned())
    }

    fn _target(&self) -> YResult<Vec<u8>> {
       target_from_bits(self.bits) 
    }

    fn _nonce(&self) -> YResult<Vec<u8>> {
        ballon_nonce(self.nonce)
    }

    pub fn verify_mining(&self) -> YResult<bool> {
        let t = self._target()?;
        let s = self._seed()?;
        let n = self._nonce()?;
        balloon_verify(&t, &s, &n, self.s_cost, self.t_cost, self.delta)
    }

    pub fn to_vec(&self) -> YResult<Vec<u8>> {
        self._check_pre_seed()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.to_vec().as_slice())?;
        bin.write_all(self.time.to_rfc3339().into_bytes().as_slice())?;
        bin.write_all(self.version.to_string().into_bytes().as_slice())?;
        bin.write_u32::<BigEndian>(self.height)?;
        bin.write_all(self.prev_id.as_slice())?;
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

    pub fn select(new: &YBlock, old: &YBlock) -> YBlock {
        if new > old {
            new.to_owned()
        } else {
            old.to_owned()
        }
    }
}
