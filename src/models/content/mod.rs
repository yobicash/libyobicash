use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;
use length::check_length;
use errors::*;
use length::MAX_LEN;
use size::MAX_SIZE;
use size::check_size;
use crypto::sign::*;
use crypto::hash::*;
use models::wallet::Wallet;
use models::signers::Signers;
use std::io::Write;
use std::ops::Index;
use std::iter::repeat;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Content {
    id: Hash,
    creators: Signers,
    checksum: Hash,
    size: u32,
    signatures_len: u32,
    signatures: Vec<Signature>,
    data: Vec<u8>,
}

impl Content {
    pub fn new(creators: &Signers, data: &Vec<u8>) -> Result<Self> {
        creators.check()?;
        check_size(data)?;
        let size = data.len() as u32;
        let checksum = hash(data.to_owned().as_slice())?;
        let id: Hash = repeat(0u8).take(HASH_SIZE).collect();
        Ok(Content {
            id: id,
            creators: creators.to_owned(),
            checksum: checksum,
            size: size,
            signatures_len: 0,
            signatures: Vec::new(),
            data: data.to_owned(),
        })
    }

    pub fn get_creators(&self) -> Signers {
        self.creators.to_owned()
    }

    pub fn get_checksum(&self) -> Hash {
        self.checksum.to_owned()
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn check_signatures_len(&self) -> Result<()> {
        if self.signatures_len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        if self.signatures_len != self.signatures.len() as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    fn check_pre_sign(&self) -> Result<()> {
        self.creators.check()?;
        check_hash_size(&self.checksum)?;
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
        self.check_signatures_len()?;
        for i in 0..self.signatures_len as usize {
            check_signature_size(&self.signatures[i])?;
        }
        Ok(())
    }

    pub fn calc_signature_checksum(&self) -> Result<Hash> {
        let mut bin = Vec::new();
        bin.write_all(self.creators.to_vec()?.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        hash(bin.as_slice())
    }

    pub fn sign(&mut self, wallet: &Wallet) -> Result<Self> {
        self.check_pre_sign()?;
        let checksum = self.calc_signature_checksum()?;
        if !self.creators.lookup_signer(&wallet.public_key)? {
            return Err(ErrorKind::NotFound.into());
        }
        let sig = sign(&checksum, &wallet.secret_key)?;
        for i in 0..self.signatures_len as usize {
            if sig == self.signatures[i] {
                return Ok(self.to_owned())
            }
        }
        self.signatures.push(sig);
        self.signatures_len += 1;
        Ok(self.to_owned())
    }

    pub fn get_signatures(&self) -> Vec<Signature> {
        self.signatures.to_owned()
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.to_owned()
    }

    pub fn get_id(&self) -> Hash {
        self.id.to_owned()
    }

    fn check_pre_id(&self) -> Result<()> {
        self.creators.check()?;
        check_hash_size(&self.checksum)?;
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
        self.check_signatures_len()?;
        for i in 0..self.signatures_len as usize {
            check_signature_size(&self.signatures[i])?;
        }
        let sig_cksm = self.calc_signature_checksum()?;
        self.creators.check_signatures(&sig_cksm, &self.signatures)?;
        Ok(())
    }

    pub fn calc_id(&self) -> Result<Hash> {
        self.check_pre_id()?;
        let mut bin = Vec::new();
        bin.write_all(self.creators.to_vec()?.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        for i in 0..self.signatures_len as usize {
            bin.write_all(self.signatures[i].to_vec().as_slice())?;
        }
        hash(bin.as_slice())
    }

    pub fn finalize(&mut self) -> Result<Self> {
        self.id = self.calc_id()?;
        Ok(self.to_owned())
    }

    pub fn check(&self) -> Result<()> {
        check_hash_size(&self.id)?;
        self.creators.check()?;
        check_hash_size(&self.checksum)?;
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
        self.check_signatures_len()?;
        for i in 0..self.signatures_len as usize {
            check_signature_size(&self.signatures[i])?;
        }
        let sig_checksum = self.calc_signature_checksum()?;
        self.creators.check_signatures(&sig_checksum, &self.signatures)?;
        let id = self.calc_id()?;
        if id != self.id {
            return Err(ErrorKind::InvalidId.into());
        }
        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.id.as_slice())?;
        bin.write_all(self.creators.to_vec()?.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        self.check_signatures_len()?;
        for i in 0..self.signatures_len as usize {
            bin.write_all(&self.signatures[i].as_slice())?;
        }
        // NB: wo\ data: it will/could be dropped later
        Ok(bin)
    }
}

#[derive(Clone, Debug)]
pub struct Contents {
    length: u32,
    idx: u32,
    items: Vec<Content>,
}

impl Contents {
    pub fn new(items: &Vec<Content>) -> Result<Contents> {
        check_length(items)?;
        let len = items.len();
        Ok(Contents {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Content) {
        self.items.push(item)
    }

    pub fn unique(&self) -> Vec<Content> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<Content> = self.unique();
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

impl Index<usize> for Contents {
    type Output = Content;

    fn index(&self, idx: usize) -> &Content {
        self.items.index(idx)
    }
}

impl Iterator for Contents {
    type Item = Content;

    fn next(&mut self) -> Option<Content> {
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

pub fn unique_contents(contents: &Vec<Content>) -> Result<Vec<Content>> {
    Ok(Contents::new(contents)?.unique().collect())
}

pub fn check_unique_contents(contents: &Vec<Content>) -> Result<()> {
    let uniques: Vec<Content> = unique_contents(contents)?;
    if uniques.len() != contents.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
