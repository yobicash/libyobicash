use byteorder::{BigEndian, WriteBytesExt};
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use models::output::Output;
use errors::*;
use std::io::Write;

pub struct OutPoint {
    tx_id: Hash,
    idx: u32,
    output: Output,
}

impl OutPoint {
    pub fn new(tx_id: &Hash, idx: u32, output: &Output) -> Result<Self> {
        check_hash_size(tx_id)?;
        output.check()?;
        Ok(OutPoint{
            tx_id: tx_id.to_owned(),
            idx: idx,
            output: output.to_owned(),
        })
    }

    pub fn get_tx_id(&self) -> Hash {
        self.tx_id.to_owned()
    }

    pub fn get_idx(&self) -> u32 {
        self.idx
    }

    pub fn get_output(&self) -> Output {
        self.output.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_hash_size(&self.tx_id)?;
        self.output.check()
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.tx_id.as_slice())?;
        bin.write_u32::<BigEndian>(self.idx)?;
        bin.write_all(self.output.to_vec()?.as_slice())?;
        Ok(bin)
    }
}
