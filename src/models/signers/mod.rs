use byteorder::{BigEndian, WriteBytesExt};
use errors::*;
use length::MAX_LEN;
use crypto::hash::Hash;
use crypto::hash::hash;
use crypto::hash::check_hash_size;
use crypto::sign::{PublicKey, Signature};
use crypto::sign::check_public_key_size;
use crypto::sign::check_signature_size;
use crypto::sign::verify_signature;
use models::address::*;
use std::io::Write;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Signers {
    address: Address,
    len: u32,
    signers: Vec<PublicKey>,
    weights: Vec<u32>,
    threshold: u32,
}

impl Signers {
    pub fn new() -> Result<Self> {
        let len = 0;
        let signers: Vec<PublicKey> = Vec::new();
        let weights: Vec<u32> = Vec::new();
        let threshold = 0;
        let mut bin = Vec::new();
        bin.write_u32::<BigEndian>(len)?;
        for i in 0..len as usize {
            bin.write_all(signers[i].as_slice())?;
        }
        for i in 0..len as usize {
            bin.write_u32::<BigEndian>(weights[i])?;
        }
        bin.write_u32::<BigEndian>(threshold)?;
        let addr_hash = hash(bin.as_slice())?;
        let address = hash_to_address(&addr_hash)?;
        Ok(Signers {
            address: address,
            len: len,
            signers: signers,
            weights: weights,
            threshold: threshold,
        })
    }

    pub fn get_len(&self) -> u32 {
        self.len
    }

    fn check_len(&self) -> Result<()> {
        if self.len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn get_signers(&self) -> Vec<PublicKey> {
        self.signers.to_owned()
    }

    pub fn lookup_signer(&self, pk: &PublicKey) -> Result<bool> {
        check_public_key_size(pk)?;
        self.check_signers()?;
        for i in 0..self.len as usize {
            if self.signers[i] == *pk {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn find_signer_idx(&self, pk: &PublicKey) -> Result<i32> {
        check_public_key_size(pk)?;
        self.check_signers()?;
        for i in 0..self.len as usize {
            if self.signers[i] == *pk {
                return Ok(i as i32);
            }
        }
        Ok(-1)
    }

    pub fn find_signer_weight(&self, pk: &PublicKey) -> Result<Option<u32>> {
        check_public_key_size(pk)?;
        self.check_signers()?;
        let idx = self.find_signer_idx(pk)?;
        let sig = if idx != -1 {
            Some(self.weights[idx as usize])
        } else {
            None
        };
        Ok(sig)
    }

    pub fn add_signer(&mut self, pk: &PublicKey, weight: u32) -> Result<Self> {
        check_public_key_size(pk)?;
        self.check_signers()?;
        self.check_weights()?;
        for i in 0..self.len as usize {
            if self.signers[i] == *pk {
                return Err(ErrorKind::AlreadyFound.into());
            }
        }
        self.len += 1;
        self.signers.push(pk.to_owned());
        self.weights.push(weight);
        Ok(self.to_owned())
    }

    fn check_signers(&self) -> Result<()> {
        if self.signers.len() != self.len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        for i in 0..self.len as usize {
            check_public_key_size(&self.signers[i])?;
        }
        Ok(())
    }

    pub fn get_weights(&self) -> Vec<u32> {
        self.weights.to_owned()
    }

    pub fn weights_sum(&self) -> u32 {
        let mut weights_sum = 0;
        for i in 0..self.weights.len() {
            weights_sum += self.weights[i];
        }
        weights_sum 
    }

    fn check_weights(&self) -> Result<()> {
        self.check_len()?;
        if self.weights.len() != self.len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn get_threshold(&self) -> u32 {
        self.threshold
    }

    pub fn set_threshold(&mut self, threshold: u32) -> Result<Self> {
        self.threshold = threshold;
        self.check_threshold()?;
        Ok(self.to_owned())
    }

    fn check_threshold(&self) -> Result<()> {
        if self.weights_sum() < self.threshold {
            return Err(ErrorKind::InvalidSum.into());
        }
        Ok(())
    }

    fn check_pre_address(&self) -> Result<()> {
        self.check_len()?;
        self.check_signers()?;
        self.check_weights()?;
        self.check_threshold()
    }

    pub fn calc_address(&self) -> Result<Address> {
        self.check_pre_address()?;
        let mut bin = Vec::new();
        bin.write_u32::<BigEndian>(self.len)?;
        for i in 0..self.len as usize {
            bin.write_all(self.signers[i].as_slice())?;
        }
        for i in 0..self.len as usize {
            bin.write_u32::<BigEndian>(self.weights[i])?;
        }
        bin.write_u32::<BigEndian>(self.threshold)?;
        let addr_hash = hash(bin.as_slice())?;
        hash_to_address(&addr_hash)
    }

    pub fn get_address(&self) -> Address {
        self.address.to_owned()
    }

    pub fn set_address(&mut self) -> Result<Self> {
        self.check_pre_address()?;
        self.address = self.calc_address()?;
        Ok(self.to_owned())
    }

    fn check_address(&self) -> Result<()> {
        check_address(&self.address)?;
        if self.address != self.calc_address()? {
            return Err(ErrorKind::InvalidAddress.into());
        }
        Ok(())
    }

    pub fn check(&self) -> Result<()> {
        self.check_len()?;
        self.check_signers()?;
        self.check_weights()?;
        self.check_threshold()?;
        self.check_address()
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        self.check()?;
        let mut bin = Vec::new();
        bin.write_all(self.address.as_slice())?;
        bin.write_u32::<BigEndian>(self.len)?;
        for i in 0..self.len as usize {
            bin.write_all(self.signers[i].as_slice())?;
        }
        for i in 0..self.len as usize {
            bin.write_u32::<BigEndian>(self.weights[i])?;
        }
        bin.write_u32::<BigEndian>(self.threshold)?;
        Ok(bin)
    }

    pub fn verify_signatures(&self, msg: &Hash, sigs: &Vec<Signature>) -> Result<bool> {
        check_hash_size(msg)?;
        for i in 0..sigs.len() {
            check_signature_size(&sigs[i])?;
        }
        let mut sum_weights = 0;
        for i in 0..sigs.len() {
            for j in 0..self.len as usize {
                let sig = sigs[i].to_owned();
                let pk = self.signers[j].to_owned();
                if verify_signature(&sig, &msg, &pk)? {
                    sum_weights += self.weights[j];
                }
            }
        }
        Ok(sum_weights >= self.threshold)
    }

    pub fn check_signatures(&self, msg: &Hash, sigs: &Vec<Signature>) -> Result<()> {
        check_hash_size(msg)?;
        for i in 0..sigs.len() {
            check_signature_size(&sigs[i])?;
        }
        if !self.verify_signatures(msg, sigs)? {
            return Err(ErrorKind::InvalidSignature.into());
        }
        Ok(())
    }
}
