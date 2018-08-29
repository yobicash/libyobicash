// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `output` module provides the transaction output type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use constants::{TESTWITNESS, MAINWITNESS};
use error::ErrorKind;
use result::Result;
use traits::{Validate, Identify, BinarySerialize, Serialize};
use utils::Amount;
use crypto::{Digest, ZKPWitness};
use crypto::Validate as CryptoValidate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::input::Input;

use std::io::Write;

/// An `Output` is an allocation of balance to a user. It can be spent only
/// providing a zero-knowledge proof verifing its zero-knowledge challenge.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Output {
    /// The id of the output.
    pub id: Digest,
    /// The amount of coins sent.
    pub amount: Amount,
    /// The Schnorr Protocol zero-knowledge-proof witness of the receiver.
    pub witness: ZKPWitness, // Point w = g^x
}

impl Output {
    /// Creates a new `Output`.
    pub fn new(amount: &Amount, witness: ZKPWitness) -> Result<Output> {
        witness.validate()?;

        if witness == ZKPWitness::from_hex(TESTWITNESS)? ||
            witness == ZKPWitness::from_hex(MAINWITNESS)? {
            if amount != &Amount::genesis_value() {
                return Err(ErrorKind::OutOfBound.into());
            }
        }

        let mut output = Output::default();
        output.amount = amount.clone();
        output.witness = witness;
        output.id = output.id()?;

        Ok(output)
    }

    /// Creates a new genesis `Output`.
    fn new_genesis_output(witness: ZKPWitness) -> Result<Output> {
        witness.validate()?;

        let amount = Amount::genesis_value();
        
        Output::new(&amount, witness)
    }

    /// Creates a new regtest genesis `Output`.
    pub fn new_regtest_genesis(witness: ZKPWitness) -> Result<Output> {
        Output::new_genesis_output(witness)
    }

    /// Creates a new testnet genesis `Output`.
    pub fn new_testnet_genesis() -> Result<Output> {
        let witness = ZKPWitness::from_hex(TESTWITNESS)?;

        Output::new_genesis_output(witness)
    }

    /// Creates a new mainnet genesis `Output`.
    pub fn new_mainnet_genesis() -> Result<Output> {
        let witness = ZKPWitness::from_hex(MAINWITNESS)?;

        Output::new_genesis_output(witness)
    }

    /// Verify if the `Output` is a genesis.
    pub fn is_genesis(&self) -> Result<bool> {
        let amount = self.amount.clone();
        let witness = self.witness;

        if witness == ZKPWitness::from_hex(TESTWITNESS)? ||
            witness == ZKPWitness::from_hex(MAINWITNESS)? {
            if amount != Amount::genesis_value() {
                return Err(ErrorKind::InvalidWitness.into());
            } else {
                return Ok(true);
            }
        } else {
            if amount == Amount::genesis_value() {
                return Ok(true);
            } 
        }

        Ok(false)
    }

    /// Verify the `Output` against an `Input`.
    pub fn verify(&self, input: &Input) -> Result<bool> {
        self.validate()?;
        input.validate()?;

        Ok(input.proof.verify(self.witness)?)
    }
}

impl<'a> Identify<'a> for Output {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.amount.to_bytes()?)?;
        buf.write_all(&self.witness.to_bytes()?)?;

        Ok(Digest::hash(&buf))
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
       Ok(id.to_bytes()?)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(Digest::from_hex(s)?)
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id.to_hex()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(self.id()?.to_bytes()?)
    }

    fn string_id(&self) -> Result<String> {
        let id = self.id()?;

        Self::id_to_string(id)
    }
}

impl Validate for Output {
    fn validate(&self) -> Result<()> {
        if self.id != self.id()? {
            return Err(ErrorKind::InvalidDigest.into());
        }
        
        let _ = self.is_genesis()?;

        Ok(())
    }
}

impl<'a> Serialize<'a> for Output {
    fn to_json(&self) -> Result<String> {
        let obj = json::json!({
            "id": self.string_id()?,
            "amount": self.amount.to_string(),
            "witness": self.witness.to_hex()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = Output::id_from_string(&id_str)?;

        let amount_value = obj["amount"].clone();
        let amount_str: String = json::from_value(amount_value)?;
        let amount = Amount::from_string(&amount_str)?;
        
        let witness_value = obj["witness"].clone();
        let witness_hex: String = json::from_value(witness_value)?;
        let witness = ZKPWitness::from_hex(&witness_hex)?;

        let output = Output {
            id: id,
            amount: amount,
            witness: witness,
        };

        Ok(output)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let output = messagepack::from_slice(b)?;

        Ok(output)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
