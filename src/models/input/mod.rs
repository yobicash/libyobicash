use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Input {
    tx_id: Hash,
    idx: u32,
}

impl Input {
    pub fn new(tx_id: &Hash, idx: u32) -> Result<Self> {
        check_hash_size(tx_id)?;
        Ok(Input {
            tx_id: tx_id.to_owned(),
            idx: idx,
        })
    }

    pub fn get_tx_id(&self) -> Hash {
        self.tx_id.to_owned()
    }

    pub fn set_tx_id(&mut self, tx_id: &Hash) -> Result<Self> {
        check_hash_size(tx_id)?;
        self.tx_id = tx_id.to_owned();
        Ok(self.to_owned())
    }

    pub fn get_idx(&self) -> u32 {
        self.idx
    }

    pub fn set_idx(&mut self, idx: u32) -> Self {
        self.idx = idx;
        self.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_hash_size(&self.tx_id)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.tx_id.as_slice())?;
        bin.write_u32::<BigEndian>(self.idx)?;
        Ok(bin)
    }
}
