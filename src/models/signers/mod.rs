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
    pub address: Address,
    pub len: u32,
    pub signers: Vec<PublicKey>,
    pub weights: Vec<u32>,
    pub threshold: u32,
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

    pub fn check_len(&self) -> Result<()> {
        if self.len > MAX_LEN as u32 {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_signers(&self) -> Result<()> {
        if self.signers.len() != self.len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        for i in 0..self.len as usize {
            check_public_key_size(&self.signers[i])?;
        }
        // NB: no check for signers uniqueness. TODO or not TODO?
        // it requires a sorting algo, to be used also after every
        // add
        Ok(())
    }

    pub fn check_weights(&self) -> Result<()> {
        self.check_len()?;
        if self.weights.len() != self.len as usize {
            return Err(ErrorKind::InvalidLength.into());
        }
        Ok(())
    }

    pub fn check_threshold(&self) -> Result<()> {
        if self.weights_sum() < self.threshold {
            return Err(ErrorKind::InvalidSum.into());
        }
        Ok(())
    }

    pub fn check_pre_address(&self) -> Result<()> {
        self.check_len()?;
        self.check_signers()?;
        self.check_weights()?;
        self.check_threshold()
    }

    pub fn check_address(&self) -> Result<()> {
        check_address(&self.address)?;
        if self.address != self.address()? {
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

    pub fn address(&self) -> Result<Address> {
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

    pub fn weights_sum(&self) -> u32 {
        let mut weights_sum = 0;
        for i in 0..self.weights.len() {
            weights_sum += self.weights[i];
        }
        weights_sum 
    }

    pub fn set_threshold(&mut self, threshold: u32) -> Result<Self> {
        self.threshold = threshold;
        self.check_threshold()?;
        Ok(self.to_owned())
    }

    pub fn set_address(&mut self) -> Result<Self> {
        self.check_pre_address()?;
        self.address = self.address()?;
        Ok(self.to_owned())
    }

    pub fn verify_signatures(&self, msg: &Hash, sigs: &Vec<Signature>) -> Result<bool> {
        check_hash_size(msg)?;
        for i in 0..sigs.len() {
            check_signature_size(&sigs[i])?;
        }
        let mut sum_weights = 0;
        for i in 0..sigs.len() {
            for j in 0..self.len as usize {
                let pk = self.signers[i].to_owned();
                let sig = sigs[j].to_owned();
                if verify_signature(&sig, &msg, &pk)? {
                    sum_weights += self.weights[i];
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
        // NB: no check for signatures uniqueness. TODO or not TODO?
        // this is strictly dependent on (non-)enforced uniqueness
        // in signers
        // it requires a sorting algo
        if !self.verify_signatures(msg, sigs)? {
            return Err(ErrorKind::InvalidSignature.into());
        }
        Ok(())
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
}
