use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;
use errors::*;
use length::check_length;
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use std::io::Write;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Input {
    tx_id: Hash,
    idx: u32,
}

impl Input {
    pub fn new(tx_id: &Hash, idx: u32) -> Result<Self> {
        check_hash_size(tx_id)?;
        Ok(Input {
            tx_id: tx_id.to_owned(),
            idx: idx,
        })
    }

    pub fn get_tx_id(&self) -> Hash {
        self.tx_id.to_owned()
    }

    pub fn set_tx_id(&mut self, tx_id: &Hash) -> Result<Self> {
        check_hash_size(tx_id)?;
        self.tx_id = tx_id.to_owned();
        Ok(self.to_owned())
    }

    pub fn get_idx(&self) -> u32 {
        self.idx
    }

    pub fn set_idx(&mut self, idx: u32) -> Self {
        self.idx = idx;
        self.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_hash_size(&self.tx_id)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.tx_id.as_slice())?;
        bin.write_u32::<BigEndian>(self.idx)?;
        Ok(bin)
    }
}

#[derive(Clone, Debug)]
pub struct Inputs {
    length: u32,
    idx: u32,
    items: Vec<Input>,
}

impl Inputs {
    pub fn new(items: &Vec<Input>) -> Result<Inputs> {
        check_length(items)?;
        let len = items.len();
        Ok(Inputs {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Input) {
        self.items.push(item)
    }

    pub fn unique(&self) -> Vec<Input> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<Input> = self.unique();
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

impl Index<usize> for Inputs {
    type Output = Input;

    fn index(&self, idx: usize) -> &Input {
        self.items.index(idx)
    }
}

impl Iterator for Inputs {
    type Item = Input;

    fn next(&mut self) -> Option<Input> {
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

pub fn unique_inputs(inputs: &Vec<Input>) -> Result<Vec<Input>> {
    Ok(Inputs::new(inputs)?.unique().collect())
}

pub fn check_unique_inputs(inputs: &Vec<Input>) -> Result<()> {
    let uniques: Vec<Input> = unique_inputs(inputs)?;
    if uniques.len() != inputs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
