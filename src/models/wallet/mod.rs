use itertools::Itertools;
use errors::*;
use length::check_length;
use crypto::sign::{Seed, PublicKey, SecretKey};
use crypto::sign::check_seed_size;
use crypto::sign::generate_keypair;
use crypto::sign::generate_keypair_from_seed;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Wallet {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let (pk, sk) = generate_keypair()?;
        Ok(Wallet {
            public_key: pk,
            secret_key: sk,
        })
    }

    pub fn from_seed(seed: &Seed) -> Result<Self> {
        check_seed_size(seed)?;
        let (pk, sk) = generate_keypair_from_seed(seed)?;
        Ok(Wallet {
            public_key: pk,
            secret_key: sk,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Wallets {
    length: u32,
    idx: u32,
    items: Vec<Wallet>,
}

impl Wallets {
    pub fn new(items: &Vec<Wallet>) -> Result<Wallets> {
        check_length(items)?;
        let len = items.len();
        Ok(Wallets {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Wallet) {
        self.items.push(item);
        self.length += 1;
    }

    pub fn unique(&self) -> Vec<Wallet> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<Wallet> = self.unique();
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

impl Index<usize> for Wallets {
    type Output = Wallet;

    fn index(&self, idx: usize) -> &Wallet {
        self.items.index(idx)
    }
}

impl Iterator for Wallets {
    type Item = Wallet;

    fn next(&mut self) -> Option<Wallet> {
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

pub fn unique_wallets(wallets: &Vec<Wallet>) -> Result<Vec<Wallet>> {
    Ok(Wallets::new(wallets)?.unique().collect())
}

pub fn check_unique_wallets(wallets: &Vec<Wallet>) -> Result<()> {
    let uniques: Vec<Wallet> = unique_wallets(wallets)?;
    if uniques.len() != wallets.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
