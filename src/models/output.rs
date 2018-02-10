// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `output` module provides the transaction output types and
//! methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use constants::{TESTWITNESS, MAINWITNESS};
use error::ErrorKind;
use result::Result;
use traits::{Validate, Identify, BinarySerialize, HexSerialize, Serialize};
use utils::{Version, NetworkType, Amount};
use crypto::{Digest, ZKPWitness};
use crypto::Validate as CryptoValidate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::input::Input;

use std::io::Write;

/// Output is an allocation of balance to a user. It can be spent only
/// providing a zero-knowledge proof verifing its zero-knowledge challenge.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Output {
    /// The id of the output.
    pub id: Digest,
    /// The protocol version.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The amount of coins sent.
    pub amount: Amount,
    /// The Schnorr Protocol zero-knowledge-proof witness of the receiver.
    pub witness: ZKPWitness, // Point w = g^x
}

impl Output {
    /// Creates a new `Output`.
    pub fn new(network_type: NetworkType, amount: &Amount, witness: ZKPWitness) -> Result<Output> {
        amount.validate()?;
        witness.validate()?;

        let mut output = Output {
            id: Digest::default(),
            version: Version::default(),
            network_type: network_type,
            amount: amount.clone(),
            witness: witness,
        };
        
        output.id = output.id()?;

        Ok(output)
    }

    /// Creates a new genesis fee.
    fn new_genesis_fee(version: Version, network_type: NetworkType, witness: ZKPWitness) -> Result<Output> {
        version.validate()?;
        witness.validate()?;
        
        let mut genesis_output = Output {
            id: Digest::default(),
            version: version,
            network_type: network_type,
            amount: Amount::max_value(),
            witness: witness,
        };
        
        genesis_output.id = genesis_output.id()?;

        Ok(genesis_output)
    }

    /// Creates a new regtest genesis fee.
    pub fn new_regtest_genesis(witness: ZKPWitness) -> Result<Output> {
        let version = Version::current()?;
        let network_type = NetworkType::RegTest;

        Output::new_genesis_fee(version, network_type, witness)
    }

    /// Creates a new testnet genesis fee.
    pub fn new_testnet_genesis() -> Result<Output> {
        let version = Version::min_value()?;
        let network_type = NetworkType::TestNet;
        let witness = ZKPWitness::from_hex(TESTWITNESS)?;

        Output::new_genesis_fee(version, network_type, witness)
    }

    /// Creates a new mainnet genesis fee.
    pub fn new_mainnet_genesis() -> Result<Output> {
        let version = Version::min_value()?;
        let network_type = NetworkType::MainNet;
        let witness = ZKPWitness::from_hex(MAINWITNESS)?;

        Output::new_genesis_fee(version, network_type, witness)
    }

    /// Verify if it is a genesis.
    pub fn verify_genesis(&self) -> Result<bool> {
        let network_type = self.network_type;
        let version = self.version.clone();
        let amount = self.amount.clone();
        let witness = self.witness;

        if network_type != NetworkType::TestNet ||
            amount != Amount::max_value() {
                if witness == ZKPWitness::from_hex(TESTWITNESS)? {
                    return Err(ErrorKind::InvalidWitness.into());
                }
        }
            
        if network_type != NetworkType::MainNet ||
            amount != Amount::max_value() {
                if witness == ZKPWitness::from_hex(MAINWITNESS)? {
                    return Err(ErrorKind::InvalidWitness.into());
                }
        }

        if amount == Amount::max_value() {
            if network_type != NetworkType::RegTest && 
                version != Version::min_value()? {
                    return Err(ErrorKind::InvalidVersion.into());
            }
        
            Ok(true)
        } else {
            Ok(false)
        }
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

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
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
        self.version.validate()?;
        self.amount.validate()?;

        let _ = self.verify_genesis()?;

        Ok(())
    }
}

impl<'a> Serialize<'a> for Output {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
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
        
        let version_value = obj["version"].clone();
        let version_string: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_string)?;
        
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;

        let amount_value = obj["amount"].clone();
        let amount_str: String = json::from_value(amount_value)?;
        let amount = Amount::from_string(&amount_str)?;
        
        let witness_value = obj["witness"].clone();
        let witness_hex: String = json::from_value(witness_value)?;
        let witness = ZKPWitness::from_hex(&witness_hex)?;

        let output = Output {
            id: id,
            version: version,
            network_type: network_type,
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
