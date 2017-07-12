use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use itertools::Itertools;
use length::check_length;
use models::address::Address;
use models::address::check_address;
use models::content::Content;
use std::io::Write;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Output {
    to: Address,
    amount: u32,
    content: Option<Content>,
}

impl Output {
    pub fn new(amount: u32, to: &Address, content: &Content) -> Result<Self> {
        check_address(to)?;
        content.check()?;
        let size = content.get_size();
        if size > 0 && size != amount {
                return Err(ErrorKind::InvalidAmount.into());
        }
        Ok(Output {
            to: to.to_owned(),
            amount: amount,
            content: Some(content.to_owned()),
        })
    }

    pub fn no_content(amount: u32, to: &Address) -> Result<Self> {
        check_address(to)?;
        Ok(Output {
            to: to.to_owned(),
            amount: amount,
            content: None,
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

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: u32) -> Result<Self> {
        self.amount = amount;
        Ok(self.to_owned())
    }

    pub fn get_content(&self) -> Option<Content> {
        self.content.to_owned()
    }

    pub fn has_content(&self) -> bool {
        self.content.is_some()
    }

    pub fn set_content(&mut self, content: &Content) -> Result<Self> {
        content.check()?;
        self.content = Some(content.to_owned());
        Ok(self.to_owned())
    }

    pub fn check(&self) -> Result<()> {
        check_address(&self.to)?;
        if let Some(content) = self.content.to_owned() {
            content.check()?;
            let size = content.get_size();
            if size > 0 && size != self.amount {
                return Err(ErrorKind::InvalidAmount.into());
            }
        }
        Ok(())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.to.as_slice())?;
        bin.write_u32::<BigEndian>(self.amount)?;
        if let Some(content) = self.content.to_owned() {
            bin.write_all(content.to_vec()?.as_slice())?;
        }
        Ok(bin)
    }
}

#[derive(Clone, Debug)]
pub struct Outputs {
    length: u32,
    idx: u32,
    items: Vec<Output>,
}

impl Outputs {
    pub fn new() -> Outputs {
        Outputs {
            length: 0,
            idx: 0,
            items: Vec::new(),
        }
    }
    
    pub fn from_vec(items: &Vec<Output>) -> Result<Outputs> {
        check_length(items)?;
        let len = items.len();
        for i in 0..items.len() {
            items[i].check()?;
        }
        Ok(Outputs {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: Output) {
        self.items.push(item);
        self.length += 1;
    }

    pub fn tot_amount(&self) -> u32 {
        let mut tot_amount = 0;
        for output in self.to_owned() {
            tot_amount = tot_amount + output.get_amount();
        }
        tot_amount
    }

    pub fn unique(&self) -> Vec<Output> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<Output> = self.unique();
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

impl Index<usize> for Outputs {
    type Output = Output;

    fn index(&self, idx: usize) -> &Output {
        self.items.index(idx)
    }
}

impl Iterator for Outputs {
    type Item = Output;

    fn next(&mut self) -> Option<Output> {
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

pub fn unique_outputs(outputs: &Vec<Output>) -> Result<Vec<Output>> {
    Ok(Outputs::from_vec(outputs)?.unique().collect())
}

pub fn check_unique_outputs(outputs: &Vec<Output>) -> Result<()> {
    let uniques: Vec<Output> = unique_outputs(outputs)?;
    if uniques.len() != outputs.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
