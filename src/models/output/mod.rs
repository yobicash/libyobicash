use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use size::MAX_SIZE;
use crypto::hash::Hash;
use crypto::hash::hash;
use amount::YAmount;
use models::address::Address;
use models::address::check_address;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct YOutput {
    to: Address,
    amount: YAmount,
    size: u32,
    checksum: Hash,
    data: Vec<u8>,
}

impl YOutput {
    pub fn new(amount: &YAmount, to: &Address, data: &Vec<u8>) -> YResult<Self> {
        check_address(to)?;
        let size = data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(YErrorKind::InvalidSize.into());
        }
        if size > 0 && YAmount::new(size) != amount.to_owned() {
                return Err(YErrorKind::InvalidAmount.into());
        }
        let checksum = hash(data.to_owned().as_slice())?;
        Ok(YOutput {
            to: to.to_owned(),
            amount: amount.to_owned(),
            size: size,
            checksum: checksum, // NB: including hash(b"")
            data: data.to_owned(),
        })
    }

    pub fn get_to(&self) -> Address {
        unreachable!()
    }

    pub fn set_to(&mut self, to: &Address) -> YResult<Self> {
        unreachable!()
    }

    pub fn get_amount(&self) -> YAmount {
        unreachable!()
    }

    pub fn set_amount(&mut self, amount: &YAmount) -> YResult<Self> {
        unreachable!()
    }

    pub fn get_size(&self) -> u32 {
        unreachable!()
    }

    pub fn get_checksum(&self) -> u32 {
        unreachable!()
    }

    pub fn get_data(&self) -> Vec<u8> {
        unreachable!()
    }

    pub fn set_data(&mut self, data: &Vec<u8>) -> YResult<Self> {
        unreachable!()
    }

    pub fn check(&self) -> YResult<()> {
        check_address(&self.to)?;
        let size = self.data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(YErrorKind::InvalidSize.into());
        }
        if self.size != size {
            return Err(YErrorKind::InvalidSize.into());
        }
        if size > 0 &&
            YAmount::new(self.size) != self.amount.to_owned() {
            return Err(YErrorKind::InvalidAmount.into());
        }
        let checksum = hash(self.data.to_owned().as_slice())?;
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
