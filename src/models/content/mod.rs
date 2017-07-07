use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;
use length::check_length;
use errors::*;
use size::MAX_SIZE;
use size::check_size;
use crypto::sign::*;
use crypto::hash::*;
use models::wallet::Wallet;
use std::io::Write;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Content {
    id: Hash,
    author: PublicKey,
    checksum: Hash,
    size: u32,
    signature: Signature,
    data: Vec<u8>,
}

impl Content {
    pub fn new(wallet: &Wallet, data: &Vec<u8>) -> Result<Self> {
        check_size(data)?;
        let author = wallet.public_key.to_owned();
        let size = data.len() as u32;
        let checksum = hash(data.to_owned().as_slice())?;
        let mut bin = Vec::new();
        bin.write_all(author.as_slice())?;
        bin.write_all(checksum.as_slice())?;
        bin.write_u32::<BigEndian>(size)?;
        let sig_checksum = hash(bin.as_slice())?;
        let signature = sign(&sig_checksum, &wallet.secret_key)?;
        bin.write_all(signature.as_slice())?;
        let id = hash(bin.as_slice())?;
        Ok(Content {
            id: id,
            author: author,
            checksum: checksum,
            size: size,
            signature: signature,
            data: data.to_owned(),
        })
    }

    pub fn get_id(&self) -> Hash {
        self.id.to_owned()
    }

    fn calc_id(&self) -> Result<Hash> {
        let mut bin = Vec::new();
        bin.write_all(self.author.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        bin.write_all(self.signature.as_slice())?;
        hash(bin.as_slice())
    }

    pub fn get_author(&self) -> PublicKey {
        self.author.to_owned()
    }

    pub fn get_checksum(&self) -> Hash {
        self.checksum.to_owned()
    }

    fn calc_signature_checksum(&self) -> Result<Hash> {
        let mut bin = Vec::new();
        bin.write_all(self.author.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        hash(bin.as_slice())
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
        check_hash_size(&self.id)?;
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
        let sig_checksum = self.calc_signature_checksum()?;
        if !verify_signature(&self.signature, &sig_checksum, &self.author)? {
            return Err(ErrorKind::InvalidSignature.into());
        }
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
        bin.write_all(self.author.as_slice())?;
        bin.write_all(self.checksum.as_slice())?;
        bin.write_u32::<BigEndian>(self.size)?;
        bin.write_all(self.signature.as_slice())?;
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
