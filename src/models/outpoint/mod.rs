use byteorder::{BigEndian, WriteBytesExt};
use num_traits::Zero;
use itertools::Itertools;
use length::check_length;
use crypto::hash::Hash;
use crypto::hash::check_hash_size;
use models::amount::Amount;
use models::input::Input;
use models::output::Output;
use errors::*;
use std::io::Write;
use std::ops::Index;
use std::iter::Iterator;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct OutPoint {
    tx_id: Hash,
    idx: u32,
    output: Output,
}

impl OutPoint {
    pub fn new(tx_id: &Hash, idx: u32, output: &Output) -> Result<Self> {
        check_hash_size(tx_id)?;
        output.check()?;
        Ok(OutPoint{
            tx_id: tx_id.to_owned(),
            idx: idx,
            output: output.to_owned(),
        })
    }

    pub fn get_tx_id(&self) -> Hash {
        self.tx_id.to_owned()
    }

    pub fn get_idx(&self) -> u32 {
        self.idx
    }

    pub fn get_output(&self) -> Output {
        self.output.to_owned()
    }

    pub fn check(&self) -> Result<()> {
        check_hash_size(&self.tx_id)?;
        self.output.check()
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.tx_id.as_slice())?;
        bin.write_u32::<BigEndian>(self.idx)?;
        bin.write_all(self.output.to_vec()?.as_slice())?;
        Ok(bin)
    }
}

#[derive(Clone, Debug)]
pub struct OutPoints {
    length: u32,
    idx: u32,
    items: Vec<OutPoint>,
}

impl OutPoints {
    pub fn new(items: &Vec<OutPoint>) -> Result<OutPoints> {
        check_length(items)?;
        let len = items.len();
        for i in 0..items.len() {
            items[i].check()?;
        }
        Ok(OutPoints {
            length: len as u32,
            idx: 0,
            items: items.to_owned(),
        })
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn push(&mut self, item: OutPoint) {
        self.items.push(item)
    }

    pub fn to_raw(&self) -> Vec<OutPoint> {
        self.items.to_owned()
    }

    pub fn tot_amount(&self) -> Amount {
        let mut tot_amount = Amount::zero();
        for outpoint in self.to_owned() {
            tot_amount = tot_amount + outpoint.output.get_amount();
        }
        tot_amount
    }

    pub fn to_inputs(&self) -> Result<Vec<Input>> {
        let mut inputs: Vec<Input> = Vec::new(); 
        for outpoint in self.to_owned() {
            let tx_id = outpoint.get_tx_id();
            let idx = outpoint.get_idx();
            let input = Input::new(&tx_id, idx)?;
            inputs.push(input);
        }
        // println!("to_inputs inputs: {:?}", inputs);
        Ok(inputs)
    }

    pub fn to_outputs(&self) -> Vec<Output> {
        let mut outputs: Vec<Output> = Vec::new(); 
        for outpoint in self.to_owned() {
            outputs.push(outpoint.output);
        }
        outputs
    }

    pub fn unique(&self) -> Vec<OutPoint> {
        self.to_owned().unique().collect()
    }

    pub fn check_unique(&self) -> Result<()> {
        let uniques: Vec<OutPoint> = self.unique();
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

impl Index<usize> for OutPoints {
    type Output = OutPoint;

    fn index(&self, idx: usize) -> &OutPoint {
        self.items.index(idx)
    }
}

impl Iterator for OutPoints {
    type Item = OutPoint;

    fn next(&mut self) -> Option<OutPoint> {
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

pub fn unique_outpoints(outpoints: &Vec<OutPoint>) -> Result<Vec<OutPoint>> {
    Ok(OutPoints::new(outpoints)?.unique().collect())
}

pub fn check_unique_outpoints(outpoints: &Vec<OutPoint>) -> Result<()> {
    let uniques: Vec<OutPoint> = unique_outpoints(outpoints)?;
    if uniques.len() != outpoints.len() {
        return Err(ErrorKind::DuplicatedElements.into());
    }
    Ok(())
}
