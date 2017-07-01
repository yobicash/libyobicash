use byteorder::{BigEndian, WriteBytesExt};
use MAX_SIZE;
use errors::*;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use address::Address;
use address::check_address;
use amount::YAmount;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct YOutput {
    pub to: Address,
    pub amount: YAmount,
    pub size: u32,
    pub checksum: Hash,
    pub data: Vec<u8>,
}

impl YOutput {
    pub fn new(m: &YAmount, to: &Address, data: &Vec<u8>) -> YResult<Self> {
        check_address(to)?;
        let size = data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(YErrorKind::InvalidSize.into());
        }
        if YAmount::new(size) != m.clone() {
            return Err(YErrorKind::InvalidAmount.into());
        }
        let checksum = hash(data.clone().as_slice())?;
        Ok(YOutput {
            to: to.to_owned(),
            amount: m.to_owned(),
            size: size,
            checksum: checksum, // NB: including hash(b"")
            data: data.to_owned(),
        })
    }

    pub fn check(&self) -> YResult<()> {
        check_hash_size(&self.to)?;
        let size = self.data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(YErrorKind::InvalidSize.into());
        }
        if self.size != size {
            return Err(YErrorKind::InvalidSize.into());
        }
        if YAmount::new(self.size) != self.amount.clone() {
            return Err(YErrorKind::InvalidAmount.into());
        }
        let checksum = hash(self.data.clone().as_slice())?;
        if self.checksum != checksum {
            return Err(YErrorKind::InvalidChecksum.into());
        }
        Ok(())
    }

    pub fn to_vec(&self) -> YResult<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.to.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        bin.write_all(self.amount.to_vec().as_slice())?;
        // NB: wo\ data: it will/could be dropped later
        bin.write_all(self.checksum.to_vec().as_slice())?;
        Ok(bin)
    }
}
