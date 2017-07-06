use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use size::MAX_SIZE;
use size::check_size;
use crypto::sign::*;
use crypto::hash::*;
use models::wallet::Wallet;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Content {
    author: PublicKey,
    checksum: Hash,
    signature: Signature,
    size: u32,
    data: Vec<u8>,
}

impl Content {
    pub fn new(wallet: &Wallet, data: &Vec<u8>) -> Result<Self> {
        check_size(data)?;
        let size = data.len() as u32;
        let checksum = hash(data.to_owned().as_slice())?;
        let signature = sign(&checksum, &wallet.secret_key)?;
        Ok(Content {
            author: wallet.public_key.to_owned(),
            checksum: checksum,
            signature: signature,
            size: size,
            data: data.to_owned(),
        })
    }

    pub fn get_author(&self) -> PublicKey {
        self.author.to_owned()
    }

    pub fn get_checksum(&self) -> Hash {
        self.checksum.to_owned()
    }

    pub fn get_signature(&self) -> Signature {
        self.signature.to_owned()
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_public_key_size(&self.author)?;
        check_hash_size(&self.checksum)?;
        check_signature_size(&self.signature)?;
        check_size(&self.data)?;
        let size = self.data.len() as u32;
        if size > MAX_SIZE as u32 {
            return Err(ErrorKind::InvalidSize.into());
        }
        if self.size != size {
            return Err(ErrorKind::InvalidSize.into());
        }
        let checksum = hash(self.data.to_owned().as_slice())?;
        if self.checksum != checksum {
            return Err(ErrorKind::InvalidChecksum.into());
        }
        if !verify_signature(&self.signature, &self.checksum, &self.author)? {
            return Err(ErrorKind::InvalidSignature.into());
        }
        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.author.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_all(self.signature.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        // NB: wo\ data: it will/could be dropped later
        Ok(bin)
    }
}
