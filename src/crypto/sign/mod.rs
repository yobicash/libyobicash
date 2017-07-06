use sodiumoxide::crypto::sign as _sign;
use itertools::Itertools;
use errors::*;
use length::check_length;
use crypto::utils::init;
use crypto::utils::check_binary_size;
use std::ops::Index;
use std::iter::Iterator;

pub const SEED_SIZE: usize = 32;

pub type Seed = Vec<u8>;

pub fn check_seed_size(sig: &Seed) -> Result<()> {
   check_binary_size(sig.as_slice(), SEED_SIZE as u32) 
}

pub const SECRETKEY_SIZE: usize = 64;

pub type SecretKey = Vec<u8>;

pub fn check_secret_key_size(sk: &SecretKey) -> Result<()> {
   check_binary_size(sk.as_slice(), SECRETKEY_SIZE as u32) 
}

#[derive(Clone, Debug)]
pub struct SecretKeys {
    length: u32,
    idx: u32,
    items: Vec<SecretKey>,
}

impl SecretKeys {
    pub fn new(items: &Vec<SecretKey>) -> Result<SecretKeys> {
        check_length(items)?;
        let len = items.len();
        Ok(SecretKeys {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: SecretKey) {
        self.items.push(item)
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

impl Index<usize> for SecretKeys {
    type Output = SecretKey;

    fn index(&self, idx: usize) -> &SecretKey {
        self.items.index(idx)
    }
}

impl Iterator for SecretKeys {
    type Item = SecretKey;

    fn next(&mut self) -> Option<SecretKey> {
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

pub fn check_unique_secret_keys(sks: &Vec<SecretKey>) -> Result<()> {
    let uniques: Vec<SecretKey> = SecretKeys::new(sks)?.unique().collect();
    if uniques.len() != sks.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}

pub const PUBLICKEY_SIZE: usize = 32;

pub type PublicKey = Vec<u8>;

pub fn check_public_key_size(pk: &PublicKey) -> Result<()> {
   check_binary_size(pk.as_slice(), PUBLICKEY_SIZE as u32) 
}

#[derive(Clone, Debug)]
pub struct PublicKeys {
    length: u32,
    idx: u32,
    items: Vec<PublicKey>,
}

impl PublicKeys {
    pub fn new(items: &Vec<PublicKey>) -> Result<PublicKeys> {
        check_length(items)?;
        let len = items.len();
        Ok(PublicKeys {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: PublicKey) {
        self.items.push(item)
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

impl Index<usize> for PublicKeys {
    type Output = PublicKey;

    fn index(&self, idx: usize) -> &PublicKey {
        self.items.index(idx)
    }
}

impl Iterator for PublicKeys {
    type Item = PublicKey;

    fn next(&mut self) -> Option<PublicKey> {
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

pub fn check_unique_public_keys(pks: &Vec<PublicKey>) -> Result<()> {
    let uniques: Vec<PublicKey> = PublicKeys::new(pks)?.unique().collect();
    if uniques.len() != pks.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}

pub const MESSAGE_SIZE: usize = 32;

pub type Message = Vec<u8>;

pub fn check_message_size(sig: &Message) -> Result<()> {
   check_binary_size(sig.as_slice(), MESSAGE_SIZE as u32) 
}

pub const SIGNATURE_SIZE: usize = 64;

pub type Signature = Vec<u8>;

pub fn check_signature_size(sig: &Signature) -> Result<()> {
   check_binary_size(sig.as_slice(), SIGNATURE_SIZE as u32) 
}

#[derive(Clone, Debug)]
pub struct Signatures {
    length: u32,
    idx: u32,
    items: Vec<Signature>,
}

impl Signatures {
    pub fn new(items: &Vec<Signature>) -> Result<Signatures> {
        check_length(items)?;
        let len = items.len();
        Ok(Signatures {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Signature) {
        self.items.push(item)
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

impl Index<usize> for Signatures {
    type Output = Signature;

    fn index(&self, idx: usize) -> &Signature {
        self.items.index(idx)
    }
}

impl Iterator for Signatures {
    type Item = Signature;

    fn next(&mut self) -> Option<Signature> {
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

pub fn check_unique_signatures(sigs: &Vec<Signature>) -> Result<()> {
    let uniques: Vec<Signature> = Signatures::new(sigs)?.unique().collect();
    if uniques.len() != sigs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}

pub fn generate_keypair() -> Result<(PublicKey, SecretKey)> {
    init()?;
    let (_pk, _sk) = _sign::gen_keypair();
    Ok((_pk.as_ref().to_vec(), _sk.0[..].to_vec()))
}

pub fn generate_keypair_from_seed(seed: &Message) -> Result<(PublicKey, SecretKey)> {
    check_seed_size(seed)?;
    let _s = _sign::Seed::from_slice(seed.as_slice()).unwrap();
    let (_pk, _sk) = _sign::keypair_from_seed(&_s);
    Ok((_pk.as_ref().to_vec(), _sk.0[..].to_vec()))
}

pub fn sign(msg: &Message, sk: &SecretKey) -> Result<Signature> {
    init()?;
    check_message_size(msg)?;
    check_secret_key_size(sk)?;
    let _sk = _sign::SecretKey::from_slice(sk.as_slice()).unwrap();
    Ok(_sign::sign_detached(msg.as_slice(), &_sk).as_ref().to_vec())
}

pub fn verify_signature(sig: &Signature, msg: &Message, pk: &PublicKey) -> Result<bool> {
    init()?;
    check_signature_size(sig)?;
    check_message_size(msg)?;
    check_public_key_size(pk)?;
    let _pk = _sign::PublicKey::from_slice(pk.as_slice()).unwrap();
    let _sig = _sign::Signature::from_slice(sig.as_slice()).unwrap();
    Ok(_sign::verify_detached(&_sig, msg.as_slice(), &_pk))
}
