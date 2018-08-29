// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `block_header` module provides the `BlockHeader` type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};

use constants::{CONFIRMATION_TIME, INTEREST_RATE, MIN_DIFFICULTY, MAX_DIFFICULTY};
use constants::{GENESIS_MEMORY, GENESIS_DIFFICULTY};
use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use utils::{Amount, Version, NetworkType, Timestamp};
use crypto::{Memory, Digest, ZKPWitness, PoW};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::block::Block;

use std::convert::From;
use std::io::Write;

/// Calculates the difficulty given the timestamp and the difficulty
/// of the previous block. [it's not right, but it's ok]
pub fn get_difficulty(timestamp: Timestamp, prev_timestamp: Timestamp, prev_diff: u32) -> Result<u32> {
    timestamp.validate()?;
    prev_timestamp.validate()?;

    if timestamp < prev_timestamp {
        return Err(ErrorKind::InvalidDuration.into());
    }

    let k = (timestamp.diff(prev_timestamp))/(CONFIRMATION_TIME as i64);

    let difficulty = (k as u32) * prev_diff;

    if difficulty < MIN_DIFFICULTY {
        return Ok(MIN_DIFFICULTY);
    }

    if difficulty > MAX_DIFFICULTY {
        return Ok(MAX_DIFFICULTY);
    }

    Ok(difficulty)
}

/// Calculates the memory to spend given the timestamp and the memory
/// spent by the previous block. [it's not right, but it's ok]
pub fn get_memory(timestamp: Timestamp, prev_timestamp: Timestamp, prev_memory: &Memory) -> Result<Memory> {
    timestamp.validate()?;
    prev_timestamp.validate()?;

    if timestamp < prev_timestamp {
        return Err(ErrorKind::InvalidDuration.into());
    }

    let _k = (timestamp.diff(prev_timestamp))/(CONFIRMATION_TIME as i64);
    let k = Memory::from(_k);

    let default_memory = PoW::default().memory()?;

    let memory = &k * prev_memory;

    if memory < default_memory {
        Ok(default_memory)
    } else {
        Ok(memory)
    }
}

/// Returns the coinbase amount gained at a given heigth.
pub fn get_coinbase_amount(height: u32) -> Amount {
    Amount::from(INTEREST_RATE).pow(height as i32)
}

/// Returns the coinbase output and coin for a given height greater then 0.
pub fn get_coinbase_output(height: u32, witness: ZKPWitness) -> Result<Output> {
    let amount = get_coinbase_amount(height);

    Output::new(&amount, witness)
}

/// A `BlockHeader` summarizes a `Block` and adds to it a `PoW` (proof-of-work) and link it to the blockchain.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    /// The `BlockHeader` id.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The unix timestamp of the time the block.
    pub timestamp: Timestamp,
    /// The block id.
    pub block_id: Digest,
    /// The block height.
    pub height: u32,
    /// The previous block id.
    pub prev_id: Digest,
    /// The block size.
    pub block_size: u32,
    /// The size of the block transactions.
    pub transactions_size: u32,
    /// The length of the block transactions.
    pub transactions_length: u32,
    /// The block transactions root.
    pub transactions_root: Digest,
    /// The coinabse amount.
    pub coinbase_amount: Amount,
    /// The coinbase output.
    pub coinbase_output: Output,
    /// The proof-of-work memory.
    pub pow_memory: Memory,
    /// The proof-of-work difficulty.
    pub pow_difficulty: u32,
    /// The proof-of-work nonde.
    pub pow_nonce: u64,
    /// The proof-of-work digest.
    pub pow_digest: Digest,
}

impl BlockHeader {
    /// Creates a new `BlockHeader`.
    pub fn new(block: &Block, prev_block_header: &BlockHeader, witness: ZKPWitness) -> Result<BlockHeader> {
        block.validate()?;
        prev_block_header.validate()?;

        if block.timestamp < prev_block_header.timestamp {
            return Err(ErrorKind::InvalidDuration.into());
        }

        if block.network_type != prev_block_header.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        let timestamp = Timestamp::now();
        let height = prev_block_header.height + 1;

        let mut block_header = BlockHeader::default();
        block_header.network_type = block.network_type;
        block_header.timestamp = timestamp;
        block_header.block_id = block.id;
        block_header.height = height;
        block_header.prev_id = prev_block_header.id;
        block_header.block_size = block.size()?;
        block_header.transactions_size = block.transactions_size;
        block_header.transactions_length = block.transactions_length;

        let mut buf = Vec::new();
        for i in 0..block.transactions_length as usize {
            // pros: easier to build
            // cons: cannot have set membership verif or search in any form
            buf.write_all(&block.transactions_ids[i].to_bytes()?)?;
        }

        block_header.transactions_root = Digest::hash(&buf);

        let coinbase_output = get_coinbase_output(height, witness)?;
        block_header.coinbase_amount = coinbase_output.amount.clone();
        block_header.coinbase_output = coinbase_output;

        let pow_memory = get_memory(timestamp, block.timestamp, &prev_block_header.pow_memory)?;
        block_header.pow_memory = pow_memory.clone();
        
        let pow_difficulty = get_difficulty(timestamp, block.timestamp, prev_block_header.pow_difficulty)?;
        block_header.pow_difficulty = pow_difficulty;

        let pow_salt = block_header.pow_salt()?;

        let mut pow = PoW::from_memory(pow_salt, &pow_memory, pow_difficulty)?;

        pow.mine()?;

        if !pow.verify()? {
            return Err(ErrorKind::NotFound.into());
        }

        block_header.pow_nonce = pow.nonce.unwrap();
        block_header.pow_digest = pow.digest.unwrap();
        block_header.id = block_header.id()?;

        Ok(block_header)
    }

    /// Returns the `PoW` salt.
    pub fn pow_salt(&self) -> Result<Digest> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
        buf.write_all(&self.timestamp.to_bytes()?)?;

        buf.write_all(&self.block_id.to_bytes()?)?;
        buf.write_u32::<BigEndian>(self.height)?;
        buf.write_all(&self.prev_id.to_bytes()?)?;

        buf.write_u32::<BigEndian>(self.transactions_size)?;
        buf.write_u32::<BigEndian>(self.transactions_length)?;
        buf.write_all(&self.transactions_root.to_bytes()?)?;

        buf.write_all(&self.coinbase_amount.to_bytes()?)?;
        buf.write_all(&self.coinbase_output.to_bytes()?)?;

        buf.write_all(self.pow_memory.to_string().as_bytes())?;
        buf.write_u32::<BigEndian>(self.pow_difficulty)?;

        Ok(Digest::hash(&buf))
    }

    /// Creates a new genesis `BlockHeader`.
    pub fn new_genesis(version: &Version, network_type: NetworkType, genesis_witness: Option<ZKPWitness>) -> Result<BlockHeader> {
        version.validate()?;

        if genesis_witness.is_some() &&
            network_type != NetworkType::RegTest {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        let block = Block::new_genesis(version, network_type, genesis_witness)?;

        let timestamp = Timestamp::min_value();

        let mut block_header = BlockHeader::default();
        block_header.network_type = block.network_type;
        block_header.timestamp = timestamp;
        block_header.block_id = block.id;
        block_header.block_size = block.size()?;
        block_header.transactions_size = block.transactions_size;
        block_header.transactions_length = block.transactions_length;

        let mut buf = Vec::new();
        for i in 0..block.transactions_length as usize {
            // pros: easier to build
            // cons: cannot have set membership verif or search in any form
            buf.write_all(&block.transactions_ids[i].to_bytes()?)?;
        }

        block_header.transactions_root = Digest::hash(&buf);

        block_header.coinbase_amount = Amount::genesis_value();
        block_header.coinbase_output = if network_type == NetworkType::RegTest {
            Output::new_regtest_genesis(genesis_witness.unwrap())?
        } else if network_type == NetworkType::TestNet {
            Output::new_testnet_genesis()?
        } else {
            Output::new_mainnet_genesis()?
        };

        let pow_memory = Memory::from(GENESIS_MEMORY);
        block_header.pow_memory = pow_memory.clone();

        let pow_difficulty = GENESIS_DIFFICULTY;
        block_header.pow_difficulty = pow_difficulty;

        let pow_salt = block_header.pow_salt()?;

        let mut pow = PoW::from_memory(pow_salt, &pow_memory, pow_difficulty)?; 

        pow.mine()?;

        if !pow.verify()? {
            return Err(ErrorKind::NotFound.into());
        }

        block_header.pow_nonce = pow.nonce.unwrap();
        block_header.pow_digest = pow.digest.unwrap();
        block_header.id = block_header.id()?;

        Ok(block_header)
    }

    /// Creates a new regtest genesis `BlockHeader`.
    pub fn new_regtest_genesis(genesis_witness: ZKPWitness) -> Result<BlockHeader> {
        let version = Version::default();
        let network_type = NetworkType::RegTest;
       
        BlockHeader::new_genesis(&version, network_type, Some(genesis_witness))
    }

    /// Creates a new testnet genesis `BlockHeader`.
    pub fn new_testnet_genesis() -> Result<BlockHeader> {
        let version = Version::default();
        let network_type = NetworkType::TestNet;
       
        BlockHeader::new_genesis(&version, network_type, None)
    }

    /// Creates a new mainnet genesis `BlockHeader`.
    pub fn new_mainnet_genesis() -> Result<BlockHeader> {
        let version = Version::default();
        let network_type = NetworkType::MainNet;
       
        BlockHeader::new_genesis(&version, network_type, None)
    }

    /// Verifies if the `BlockHeader` is a genesis.
    pub fn is_genesis(&self) -> Result<bool> {
        let block_id = self.block_id;
        let testnet_genesis = Block::new_testnet_genesis()?;
        let mainnet_genesis = Block::new_mainnet_genesis()?;

        if self.height == 0 {
            if self.prev_id != Digest::default() {
                return Err(ErrorKind::InvalidDigest.into());
            }

            if self.transactions_length != 1 {
                return Err(ErrorKind::InvalidLength.into());
            }

            if block_id == Block::new_testnet_genesis()?.id {
                if self.network_type != NetworkType::TestNet {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                if self.timestamp != Timestamp::min_value() {
                    return Err(ErrorKind::InvalidTimestamp.into());
                }

                if self.block_size != testnet_genesis.size()? {
                    return Err(ErrorKind::InvalidLength.into());
                }
                
                return Ok(true);
            }

            if block_id == Block::new_mainnet_genesis()?.id {
                if self.network_type != NetworkType::MainNet {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                if self.timestamp != Timestamp::min_value() {
                    return Err(ErrorKind::InvalidTimestamp.into());
                }

                if self.block_size != mainnet_genesis.size()? {
                    return Err(ErrorKind::InvalidLength.into());
                }

                return Ok(true);
            }

            if self.network_type != NetworkType::RegTest {
                return Err(ErrorKind::InvalidNetwork.into());    
            }

            if self.coinbase_amount != Amount::genesis_value() {
                return Err(ErrorKind::InvalidGenesis.into());
            }

            Ok(true)
        } else {
            if block_id == testnet_genesis.id ||
                block_id == mainnet_genesis.id {
                return Err(ErrorKind::InvalidBlock.into());
            }

            Ok(false)
        }
    }

    /// Returns the size of the `BlockHeader`.
    pub fn size(&self) -> Result<u32> {
        Ok(self.to_bytes()?.len() as u32)
    }

    /// Verifies the `BlockHeader` against a block and its previous `BlockHeader`.
    pub fn verify(&self, block: &Block, prev_block_header: Option<&BlockHeader>) -> Result<bool> {
        self.validate()?;
        block.validate()?;

        if prev_block_header.is_none() {
            if !block.is_genesis()? {
                return Err(ErrorKind::InvalidBlock.into());
            }
            
            return Ok(true);
        }

        let prev_bh = prev_block_header.unwrap();
        prev_bh.validate()?;

        if block.timestamp < prev_bh.timestamp {
            return Err(ErrorKind::InvalidDuration.into());
        }

        if block.network_type != prev_bh.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        if self.network_type != block.network_type {
            return Ok(false);
        }

        if self.timestamp < block.timestamp {
            return Ok(false);
        }

        if self.block_id != block.id {
            return Ok(false);
        }

        if self.height != prev_bh.height + 1 {
            return Ok(false);
        }

        if self.prev_id != prev_bh.id {
            return Ok(false);
        }

        if self.transactions_size != block.transactions_size {
            return Ok(false);
        }

        if self.transactions_length != block.transactions_length {
            return Ok(false);
        }

        let pow_memory = get_memory(self.timestamp, block.timestamp, &prev_bh.pow_memory)?;

        if self.pow_memory != pow_memory {
            return Ok(false);
        }
        
        let pow_difficulty = get_difficulty(self.timestamp, block.timestamp, prev_bh.pow_difficulty)?;

        if self.pow_difficulty != pow_difficulty {
            return Ok(false);
        }

        let pow_salt = self.pow_salt()?;

        let mut pow = PoW::from_memory(pow_salt, &pow_memory, pow_difficulty)?;
        pow.nonce = Some(self.pow_nonce);
        pow.digest = Some(self.pow_digest);

        Ok(pow.verify()?)
    }
}

impl Default for BlockHeader {
    fn default() -> BlockHeader {
        BlockHeader {
            id: Digest::default(),
            version: Version::default(),
            network_type: NetworkType::default(),
            timestamp: Timestamp::default(),
            block_id: Digest::default(),
            height: 0,
            prev_id: Digest::default(),
            block_size: 0,
            transactions_size: 0,
            transactions_length: 0,
            transactions_root: Digest::default(),
            coinbase_amount: Amount::zero(),
            coinbase_output: Output::default(),
            pow_memory: Memory::zero(),
            pow_difficulty: 0,
            pow_nonce: 0,
            pow_digest: Digest::default(),
        }
    }
}

impl<'a> Identify<'a> for BlockHeader {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
        buf.write_all(&self.timestamp.to_bytes()?)?;

        buf.write_all(&self.block_id.to_bytes()?)?;
        buf.write_u32::<BigEndian>(self.height)?;
        buf.write_all(&self.prev_id.to_bytes()?)?;

        buf.write_u32::<BigEndian>(self.transactions_size)?;
        buf.write_u32::<BigEndian>(self.transactions_length)?;
        buf.write_all(&self.transactions_root.to_bytes()?)?;
        buf.write_all(&self.coinbase_amount.to_bytes()?)?;
        buf.write_all(&self.coinbase_output.to_bytes()?)?;
        buf.write_all(&self.pow_memory.to_string().as_bytes())?;
        buf.write_u32::<BigEndian>(self.pow_difficulty)?;
        buf.write_u64::<BigEndian>(self.pow_nonce)?;
        buf.write_all(&self.pow_digest.to_bytes()?)?;

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

impl Validate for BlockHeader {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.timestamp.validate()?;
        
        if self.id != self.id()? {
            return Err(ErrorKind::InvalidDigest.into());
        }

        if self.prev_id == self.id {
            return Err(ErrorKind::InvalidDigest.into());
        }

        if self.transactions_size == 0 ||
            self.transactions_length == 0 {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        self.coinbase_output.validate()?;

        if self.height == 0 {
            if self.coinbase_amount != Amount::genesis_value() {
                return Err(ErrorKind::OutOfBound.into());
            }
        } else {
            if self.coinbase_amount != get_coinbase_amount(self.height) {
                return Err(ErrorKind::OutOfBound.into());
            }
        }

        if self.coinbase_amount != self.coinbase_output.amount {
            return Err(ErrorKind::OutOfBound.into());
        }

        let pow_salt = self.pow_salt()?;

        let mut pow = PoW::from_memory(pow_salt, &self.pow_memory, self.pow_difficulty)?;
        pow.nonce = Some(self.pow_nonce);
        pow.digest = Some(self.pow_digest);

        if !pow.verify()? {
            return Err(ErrorKind::InvalidPoW.into());
        }

        let _ = self.is_genesis()?;

        if self.height == 1 {
            let prev_id = self.prev_id; 

            if self.network_type == NetworkType::TestNet {
                if prev_id != BlockHeader::new_testnet_genesis()?.id {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
            }

            if self.network_type == NetworkType::MainNet {
                if prev_id != BlockHeader::new_mainnet_genesis()?.id {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
            }
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for BlockHeader {
    fn to_json(&self) -> Result<String> {
        let obj = json::json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "timestamp": self.timestamp.to_string(),
            "block_id": self.block_id.to_hex()?,
            "height": self.height,
            "prev_id": self.prev_id.to_hex()?,
            "block_size": self.block_size,
            "transactions_size": self.transactions_size,
            "transactions_length": self.transactions_length,
            "transactions_root": self.transactions_root.to_hex()?,
            "coinbase_amount": self.coinbase_amount.to_string(),
            "coinbase_output": self.coinbase_output.to_json()?,
            "pow_memory": self.pow_memory.to_string(),
            "pow_difficulty": self.pow_difficulty,
            "pow_nonce": self.pow_nonce,
            "pow_digest": self.pow_digest.to_hex()?,
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
        
        let block_id_value = obj["block_id"].clone();
        let block_id_str: String = json::from_value(block_id_value)?;
        let block_id = Block::id_from_string(&block_id_str)?;

        let height_value = obj["height"].clone();
        let height: u32 = json::from_value(height_value)?;
        
        let prev_id_value = obj["prev_id"].clone();
        let prev_id_str: String = json::from_value(prev_id_value)?;
        let prev_id = Block::id_from_string(&prev_id_str)?;

        let block_size_value = obj["block_size"].clone();
        let block_size: u32 = json::from_value(block_size_value)?;

        let transactions_size_value = obj["transactions_size"].clone();
        let transactions_size: u32 = json::from_value(transactions_size_value)?;

        let transactions_length_value = obj["transactions_length"].clone();
        let transactions_length: u32 = json::from_value(transactions_length_value)?;

        let txs_root_value = obj["transactions_root"].clone();
        let txs_root_str: String = json::from_value(txs_root_value)?;
        let txs_root = Block::id_from_string(&txs_root_str)?;

        let coinbase_amount_value = obj["coinbase_amount"].clone();
        let coinbase_amount_str: String = json::from_value(coinbase_amount_value)?;
        let coinbase_amount = Amount::from_string(&coinbase_amount_str)?;

        let coinbase_output_value = obj["coinbase_output"].clone();
        let coinbase_output_json: String = json::from_value(coinbase_output_value)?;
        let coinbase_output = Output::from_json(&coinbase_output_json)?;

        let pow_memory_value = obj["pow_memory"].clone();
        let pow_memory_str: String = json::from_value(pow_memory_value)?;
        let pow_memory = Memory::from_string(&pow_memory_str)?;

        let pow_difficulty_value = obj["pow_difficulty"].clone();
        let pow_difficulty: u32 = json::from_value(pow_difficulty_value)?;

        let pow_nonce_value = obj["pow_nonce"].clone();
        let pow_nonce: u64 = json::from_value(pow_nonce_value)?;

        let pow_digest_value = obj["pow_digest"].clone();
        let pow_digest_str: String = json::from_value(pow_digest_value)?;
        let pow_digest = Digest::from_hex(&pow_digest_str)?;

        let block_header = BlockHeader {
            id: id,
            version: version,
            network_type: network_type,
            timestamp: timestamp,
            block_id: block_id,
            height: height,
            prev_id: prev_id,
            block_size: block_size,
            transactions_size: transactions_size,
            transactions_length: transactions_length,
            transactions_root: txs_root,
            coinbase_amount: coinbase_amount,
            coinbase_output: coinbase_output,
            pow_memory: pow_memory,
            pow_difficulty: pow_difficulty,
            pow_nonce: pow_nonce,
            pow_digest: pow_digest,
        };

        Ok(block_header)
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
