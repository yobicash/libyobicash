// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `block` module provides the block type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;

use constants::{TESTWITNESS, MAINWITNESS};
use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use utils::{Version, NetworkType, Timestamp};
use crypto::{Digest, ZKPWitness};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::transaction::Transaction;

use std::io::Write;

/// A `Block` is a commitment of validated transactions.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Block {
    /// The block id.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The unix timestamp of the time the block.
    pub timestamp: Timestamp,
    /// The size of the block transactions.
    pub transactions_size: u32,
    /// The length of the block transactions.
    pub transactions_length: u32,
    /// The block transactions' ids.
    pub transactions_ids: Vec<Digest>,
}

impl Block {
    /// Creates a new `Block`.
    pub fn new(network_type: NetworkType,
               transactions: &[Transaction]) -> Result<Block> {
        for transaction in transactions {
            transaction.validate()?;

            if transaction.network_type != network_type {
                return Err(ErrorKind::InvalidNetwork.into());
            }
        }

        let transactions_length = transactions.len() as u32;

        let mut transactions_ids = Vec::new();
        let mut transactions_size = 0;
        for transaction in transactions {
            transactions_ids.push(transaction.id);
            transactions_size += &transaction.size()?;
        }

        let mut transactions_binary_ids = Vec::new();
        for id in transactions_ids.clone() {
            transactions_binary_ids.push(id.to_bytes()?);
        }

        if transactions_binary_ids.iter().unique().count() !=
            transactions.len() {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        let mut block = Block::default();
        block.network_type = network_type;
        block.timestamp = Timestamp::now();
        block.transactions_size = transactions_size;
        block.transactions_length = transactions_length;
        block.transactions_ids = transactions_ids;
        block.id = block.id()?;

        Ok(block)
    }

    /// Creates a new genesis `Block`.
    pub fn new_genesis(version: &Version, network_type: NetworkType, genesis_witness: Option<ZKPWitness>) -> Result<Block> {
        version.validate()?;

        if let Some(gw) = genesis_witness {
            if network_type == NetworkType::TestNet {
                if gw != ZKPWitness::from_hex(TESTWITNESS)? {
                    return Err(ErrorKind::InvalidWitness.into());
                }
            }

            if network_type == NetworkType::MainNet {
                if gw != ZKPWitness::from_hex(MAINWITNESS)? {
                    return Err(ErrorKind::InvalidWitness.into());
                }
            }
        } else {
            if network_type == NetworkType::RegTest {
                return Err(ErrorKind::InvalidNetwork.into());
            }
        }

        let genesis_transaction = if network_type == NetworkType::TestNet {
            Transaction::new_testnet_genesis()?
        } else if network_type == NetworkType::MainNet {
            Transaction::new_mainnet_genesis()?
        } else {
            Transaction::new_regtest_genesis(genesis_witness.unwrap())?
        };

        let mut block = Block::default();

        if network_type != NetworkType::RegTest {
            block.timestamp = Timestamp::min_value();
        }
       
        block.network_type = network_type;
        block.transactions_size = genesis_transaction.size()?;
        block.transactions_length = 1;
        block.transactions_ids = vec![genesis_transaction.id];
        block.id = block.id()?;

        Ok(block)
    }

    /// Creates a new regtest genesis `Block`.
    pub fn new_regtest_genesis(genesis_witness: ZKPWitness) -> Result<Block> {
        let version = Version::default();
        let network_type = NetworkType::RegTest;
       
        Block::new_genesis(&version, network_type, Some(genesis_witness))
    }

    /// Creates a new testnet genesis `Block`.
    pub fn new_testnet_genesis() -> Result<Block> {
        let version = Version::default();
        let network_type = NetworkType::TestNet;
       
        Block::new_genesis(&version, network_type, None)
    }

    /// Creates a new mainnet genesis `Block`.
    pub fn new_mainnet_genesis() -> Result<Block> {
        let version = Version::default();
        let network_type = NetworkType::MainNet;
       
        Block::new_genesis(&version, network_type, None)
    }

    /// Verifies if the `Block` is a genesis.
    pub fn is_genesis(&self) -> Result<bool> {
        let testnet_genesis = Transaction::new_testnet_genesis()?;
        let mainnet_genesis = Transaction::new_mainnet_genesis()?;

        if self.transactions_length == 1 {
            let transaction_id = self.transactions_ids[0];

            if transaction_id == testnet_genesis.id {
                if self.network_type != NetworkType::TestNet {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                if self.timestamp != Timestamp::min_value() {
                    return Err(ErrorKind::InvalidTimestamp.into());
                }

                if self.transactions_size != testnet_genesis.size()? {
                    return Err(ErrorKind::InvalidLength.into());
                }
                
                return Ok(true);
            }

            if transaction_id == mainnet_genesis.id {
                if self.network_type != NetworkType::MainNet {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                if self.timestamp != Timestamp::min_value() {
                    return Err(ErrorKind::InvalidTimestamp.into());
                }

                if self.transactions_size != mainnet_genesis.size()? {
                    return Err(ErrorKind::InvalidLength.into());
                }

                return Ok(true);
            }

            if self.network_type != NetworkType::RegTest {
                return Err(ErrorKind::InvalidNetwork.into());
            }

            Ok(true)
        } else {
            for transaction_id in self.transactions_ids.clone() {
                if transaction_id == testnet_genesis.id ||
                    transaction_id == mainnet_genesis.id {
                    return Err(ErrorKind::InvalidBlock.into());
                }
            }

            Ok(false)
        }
    }

    /// Returns the size of the `Block`.
    pub fn size(&self) -> Result<u32> {
        Ok(self.to_bytes()?.len() as u32)
    }
}

impl Default for Block {
    fn default() -> Block {
        Block {
            id: Digest::default(),
            version: Version::default(),
            network_type: NetworkType::default(),
            timestamp: Timestamp::default(),
            transactions_size: 0,
            transactions_length: 0,
            transactions_ids: Vec::new(),
        }
    }
}

impl<'a> Identify<'a> for Block {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
        buf.write_all(&self.timestamp.to_bytes()?)?;

        buf.write_u32::<BigEndian>(self.transactions_size)?;
        buf.write_u32::<BigEndian>(self.transactions_length)?;
        
        for i in 0..self.transactions_length as usize {
            let id = &self.transactions_ids[i];
            buf.write_all(&id.to_bytes()?)?;
        }

        Ok(Digest::hash(&buf))
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
        Ok(id.to_bytes()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        let id = self.id()?;

        Self::id_to_bytes(id)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(Digest::from_hex(s)?)
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id.to_hex()?)
    }

    fn string_id(&self) -> Result<String> {
        let id = self.id()?;

        Self::id_to_string(id)
    }
}

impl Validate for Block {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.timestamp.validate()?;
        
        if self.id != self.id()? {
            return Err(ErrorKind::InvalidDigest.into());
        }

        if self.transactions_size == 0 ||
            self.transactions_length == 0 {
            return Err(ErrorKind::InvalidLength.into());    
        }

        if self.transactions_length as usize != self.transactions_ids.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        let mut transactions_binary_ids = Vec::new();
        for id in self.transactions_ids.clone() {
            transactions_binary_ids.push(id.to_bytes()?);
        }

        if transactions_binary_ids.iter().unique().count() != self.transactions_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }

        let _ = self.is_genesis()?;

        Ok(())
    }
}

impl<'a> Serialize<'a> for Block {
    fn to_json(&self) -> Result<String> {
        let mut json_transactions_ids = Vec::new();
        for id in self.transactions_ids.clone() {
            json_transactions_ids.push(id.to_hex()?);
        }

        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "timestamp": self.timestamp.to_string(),
            "transactions_size": self.transactions_size,
            "transactions_length": self.transactions_length,
            "transactions_ids": json_transactions_ids,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = Block::id_from_string(&id_str)?;
        
        let version_value = obj["version"].clone();
        let version_str: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_str)?;
        
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;
        
        let timestamp_value = obj["timestamp"].clone();
        let timestamp_str: String = json::from_value(timestamp_value)?;
        let timestamp = Timestamp::from_string(&timestamp_str)?;

        let transactions_size_value = obj["transactions_size"].clone();
        let transactions_size: u32 = json::from_value(transactions_size_value)?;

        let transactions_length_value = obj["transactions_length"].clone();
        let transactions_length: u32 = json::from_value(transactions_length_value)?;

        let transactions_ids_value = obj["transactions_ids"].clone();
        let transactions_ids_json: Vec<String> = json::from_value(transactions_ids_value)?;

        let mut transactions_ids = Vec::new();

        for id_hex in transactions_ids_json {
            let id = Digest::from_hex(&id_hex)?;
            transactions_ids.push(id);
        }

        let block = Block {
            id: id,
            version: version,
            network_type: network_type,
            timestamp: timestamp,
            transactions_size: transactions_size,
            transactions_length: transactions_length,
            transactions_ids: transactions_ids,
        };

        Ok(block)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let tx = messagepack::from_slice(b)?;

        Ok(tx)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
