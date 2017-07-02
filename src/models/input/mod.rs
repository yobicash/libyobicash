use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct YInput {
    pub tx_id: Hash,
    pub idx: u32,
}

impl YInput {
    pub fn new(tx_id: &Hash, idx: u32) -> YResult<Self> {
        check_hash_size(tx_id)?;
        Ok(YInput {
            tx_id: tx_id.to_owned(),
            idx: idx,
        })
    }

    pub fn check(&self) -> YResult<()> {
        check_hash_size(&self.tx_id)
    }

    pub fn to_vec(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.tx_id.as_slice())?;
        bin.write_u32::<BigEndian>(self.idx)?;
        Ok(bin)
    }
}
