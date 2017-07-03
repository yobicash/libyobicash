use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use size::MAX_SIZE;
use crypto::hash::Hash;
use crypto::hash::hash;
use amount::Amount;
use models::address::Address;
use models::address::check_address;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Output {
    to: Address,
    amount: Amount,
    size: u32,
    checksum: Hash,
    data: Vec<u8>,
}

impl Output {
    pub fn new(amount: &Amount, to: &Address, data: &Vec<u8>) -> Result<Self> {
        check_address(to)?;
        let size = data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(ErrorKind::InvalidSize.into());
        }
        if size > 0 && Amount::new(size) != amount.to_owned() {
                return Err(ErrorKind::InvalidAmount.into());
        }
        let checksum = hash(data.to_owned().as_slice())?;
        Ok(Output {
            to: to.to_owned(),
            amount: amount.to_owned(),
            size: size,
            checksum: checksum, // NB: including hash(b"")
            data: data.to_owned(),
        })
    }

    pub fn get_to(&self) -> Address {
        self.to.to_owned()
    }

    pub fn set_to(&mut self, to: &Address) -> Result<Self> {
        check_address(to)?;
        self.to = to.to_owned();
        Ok(self.to_owned())
    }

    pub fn get_amount(&self) -> Amount {
        self.amount.to_owned()
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_checksum(&self) -> Result<Hash> {
        hash(self.data.to_owned().as_slice())
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_address(&self.to)?;
        let size = self.data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(ErrorKind::InvalidSize.into());
        }
        if self.size != size {
            return Err(ErrorKind::InvalidSize.into());
        }
        if size > 0 &&
            Amount::new(self.size) != self.amount.to_owned() {
            return Err(ErrorKind::InvalidAmount.into());
        }
        let checksum = self.get_checksum()?;
        if self.checksum != checksum {
            return Err(ErrorKind::InvalidChecksum.into());
        }
        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
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
