// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transaction` module provides the transaction type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use utils::{Version, NetworkType, Timestamp, Amount};
use crypto::{Digest, ZKPWitness};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::data::Data;
use models::coin::Coin;
use models::input::Input;

use std::io::Write;

/// A `Transaction` is a transfer of balance from the inputs of
/// (generally) one user to one or more users.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// The id of the transaction.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The unix timestamp of the time the transaction was created.
    pub timestamp: Timestamp,
    /// The size of the transaction inputs.
    pub inputs_length: u32,
    /// The transaction inputs.
    pub inputs: Vec<Input>,
    /// The transaction outputs' amount.
    pub outputs_amount: Amount,
    /// The length of the transaction outputs.
    pub outputs_length: u32,
    /// The transaction outputs.
    pub outputs: Vec<Output>,
    /// The size of the transaction data.
    pub data_size: u32,
    /// The length of the transaction data.
    pub data_length: u32,
    /// The transaction data ids.
    pub data_ids: Vec<Digest>,
    /// The transaction fee.
    pub fee: Amount,
}

impl Transaction {
    /// Creates a new `Transaction`.
    pub fn new(network_type: NetworkType,
               coins: &[Coin],
               outputs: &[Output],
               data: &[Data],
               fee: &Amount) -> Result<Transaction> {
        for coin in coins {
            coin.validate()?;
        }

        for output in outputs {
            output.validate()?;
        }

        for d in data {
            d.validate()?;
        }

        let coins_length = coins.len();

        let mut coins_ids = Vec::new();
        for coin in coins {
            coins_ids.push(coin.binary_id()?);
        }

        if coins_ids.iter().unique().count() != coins_length {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        let mut coins_amount = Amount::new();
        for i in 0..coins_length {
            let coin = &coins[i];
            coins_amount += &coin.amount;
        }

        let outputs_length = outputs.len() as u32;

        let mut outputs_ids = Vec::new();
        let mut outputs_amount = Amount::new();
        for output in outputs {
            outputs_ids.push(output.id);
            outputs_amount += &output.amount;
        }

        if coins_amount != outputs_amount.clone() + fee {
            return Err(ErrorKind::OutOfBound.into());
        }

        let mut outputs_binary_ids = Vec::new();
        for id in outputs_ids.clone() {
            outputs_binary_ids.push(id.to_bytes()?);
        }

        if outputs_binary_ids.iter().unique().count() != outputs.len() {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        let data_length = data.len();

        let mut data_ids = Vec::new();
        for d in data {
            data_ids.push(d.id);
        }

        let mut data_binary_ids = Vec::new();
        for id in data_ids.clone() {
            data_binary_ids.push(id.to_bytes()?);
        }

        let mut data_size = 0;
        for i in 0..data_length {
            let d = &data[i];
            data_size += &d.cyph_size;
        }

        let timestamp = Timestamp::now();

        let mut inputs: Vec<Input> = Vec::new();

        let mut message_header = Vec::new();
        
        message_header.write_all(&Version::default().to_bytes()?)?;
        message_header.write_all(&network_type.to_bytes()?)?;
        message_header.write_all(&timestamp.to_bytes()?)?;
        
        message_header.write_u32::<BigEndian>(coins_length as u32)?;
        
        message_header.write_all(&outputs_amount.to_bytes()?)?;
        message_header.write_u32::<BigEndian>(outputs_length)?;
        
        for i in 0..outputs_length as usize {
            let id = &outputs_ids[i];
            message_header.write_all(&id.to_bytes()?)?;
        }
        
        message_header.write_all(&fee.to_bytes()?)?;

        for i in 0..coins_length as usize {
            let coin = &coins[i];

            let mut message = message_header.clone();
            
            for i in 0..inputs.len() {
                let input = inputs[i];
                let input_buf = input.to_bytes()?;
                message.write_all(&input_buf)?;
            }

            let input = Input::new(coin, &message)?;

            inputs.push(input);
        }

        let mut tx = Transaction::default();
        tx.network_type = network_type;
        tx.timestamp = timestamp;
        tx.inputs_length = coins_length as u32;
        tx.inputs = inputs;
        tx.outputs_amount = outputs_amount;
        tx.outputs_length = outputs_length;
        tx.outputs = outputs.to_vec();
        tx.data_size = data_size;
        tx.data_length = data.len() as u32;
        tx.data_ids = data_ids;
        tx.fee = fee.clone();
        tx.id = tx.id()?;

        Ok(tx)
    }

    /// Creates a new genesis `Transaction`.
    fn new_genesis(version: &Version, network_type: NetworkType, genesis_output: &Output) -> Result<Transaction> {
        version.validate()?;
        genesis_output.validate()?;

        if genesis_output.amount != Amount::genesis_value() {
            return Err(ErrorKind::InvalidGenesis.into());
        }

        let timestamp = if network_type == NetworkType::RegTest {
            Timestamp::now()
        } else {
            Timestamp::min_value()
        };

        let mut genesis_tx = Transaction::default();
        genesis_tx.version = version.clone();
        genesis_tx.network_type = network_type;
        genesis_tx.timestamp = timestamp;
        genesis_tx.outputs_length = 1;
        genesis_tx.outputs_amount = Amount::genesis_value();
        genesis_tx.outputs = vec![genesis_output.clone()];
        genesis_tx.id = genesis_tx.id()?;

        Ok(genesis_tx)
    }

    /// Creates a new regtest genesis `Transaction`.
    pub fn new_regtest_genesis(genesis_witness: ZKPWitness) -> Result<Transaction> {
        let version = Version::default();
        let network_type = NetworkType::RegTest;
        let genesis_output = Output::new_regtest_genesis(genesis_witness)?;

        Transaction::new_genesis(&version, network_type, &genesis_output)
    }

    /// Creates a new testnet genesis `Transaction`.
    pub fn new_testnet_genesis() -> Result<Transaction> {
        let version = Version::default();
        let network_type = NetworkType::TestNet;
        let genesis_output = Output::new_testnet_genesis()?;

        Transaction::new_genesis(&version, network_type, &genesis_output)
    }

    /// Creates a new mainnet genesis `Transaction`.
    pub fn new_mainnet_genesis() -> Result<Transaction> {
        let version = Version::default();
        let network_type = NetworkType::MainNet;
        let genesis_output = Output::new_mainnet_genesis()?;

        Transaction::new_genesis(&version, network_type, &genesis_output)
    }

    /// Verifies if the `Transaction` is a coinbase transaction.
    pub fn is_coinbase(&self) -> Result<bool> {
        if self.inputs_length == 0 {
            if self.outputs_length != 1 {
                return Err(ErrorKind::InvalidLength.into());
            }
            
            if self.data_length != 0 {
                return Err(ErrorKind::InvalidLength.into());
            }
           
            if self.outputs_amount < Amount::genesis_value() {
                return Err(ErrorKind::OutOfBound.into());
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Verifies if the `Transaction` is a genesis transaction.
    pub fn is_genesis(&self) -> Result<bool> {
        if !self.is_coinbase()? {
            return Ok(false);
        }

        let output = self.outputs[0].clone();
        
        if !output.is_genesis()? {
            return Ok(false);
        };

        if output.id == Output::new_testnet_genesis()?.id {
            if self.network_type != NetworkType::TestNet {
                return Err(ErrorKind::InvalidNetwork.into());
            }
            
            return Ok(true);
        }

        if output.id == Output::new_mainnet_genesis()?.id {
            if self.network_type != NetworkType::MainNet {
                return Err(ErrorKind::InvalidNetwork.into());
            }

            return Ok(true);
        }

        if self.network_type != NetworkType::RegTest {
            return Err(ErrorKind::InvalidNetwork.into());    
        }

        Ok(true)
    }

    /// Returns the total amount sent in the `Transaction`.
    pub fn total_amount(&self) -> Amount {
        &self.outputs_amount + &self.fee
    }

    /// Returns the size of the `Transaction`.
    pub fn size(&self) -> Result<u32> {
        Ok(self.to_bytes()?.len() as u32)
    }
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            id: Digest::default(),
            version: Version::default(),
            network_type: NetworkType::default(),
            timestamp: Timestamp::default(),
            inputs_length: 0,
            inputs: Vec::new(),
            outputs_amount: Amount::new(),
            outputs_length: 0,
            outputs: Vec::new(),
            data_size: 0,
            data_length: 0,
            data_ids: Vec::new(),
            fee: Amount::default(),
        }
    }
}

impl<'a> Identify<'a> for Transaction {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
        buf.write_all(&self.timestamp.to_bytes()?)?;
        buf.write_u32::<BigEndian>(self.inputs_length)?;
        
        for i in 0..self.inputs_length as usize {
            let input = self.inputs[i];
            let input_buf = input.to_bytes()?;
            buf.write_all(&input_buf)?;
        }

        buf.write_all(&self.outputs_amount.to_bytes()?)?;
        
        buf.write_u32::<BigEndian>(self.outputs_length)?;
        for i in 0..self.outputs_length as usize {
            let id = &self.outputs[i].id;
            buf.write_all(&id.to_bytes()?)?;
        }

        buf.write_all(&self.fee.to_bytes()?)?;

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

impl Validate for Transaction {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.timestamp.validate()?;

        if self.id != self.id()? {
            return Err(ErrorKind::InvalidDigest.into());
        }

        if self.inputs_length as usize != self.inputs.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        if self.outputs_length as usize != self.outputs.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        if self.data_length as usize != self.data_ids.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        let mut inputs_binary_ids = Vec::new();
        for input in self.inputs.clone() {
            input.validate()?;
            inputs_binary_ids.push(input.binary_id()?);
        }

        if inputs_binary_ids.iter().unique().count() !=
            self.inputs_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }

        let mut outputs_binary_ids = Vec::new();
        for output in self.outputs.clone() {
            outputs_binary_ids.push(output.id.to_bytes()?);
        }

        if outputs_binary_ids.iter().unique().count() !=
            self.outputs_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }

        let mut data_binary_ids = Vec::new();
        for id in self.data_ids.clone() {
            data_binary_ids.push(id.to_bytes()?);
        }

        if data_binary_ids.iter().unique().count() !=
            self.data_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }

        if !self.is_genesis()? && !self.is_coinbase()? {
            if self.inputs_length == 0 {
                return Err(ErrorKind::InvalidLength.into());
            } 
            
            for output in self.outputs.clone() {
                if output.is_genesis()? {
                    return Err(ErrorKind::InvalidTransaction.into());
                }
            }
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for Transaction {
    fn to_json(&self) -> Result<String> {
        let mut json_inputs = Vec::new();
        for input in self.inputs.clone() {
            json_inputs.push(input.to_json()?);
        }

        let mut json_outputs = Vec::new();
        for output in self.outputs.clone() {
            json_outputs.push(output.to_json()?);
        }

        let mut json_data_ids = Vec::new();
        for id in self.data_ids.clone() {
            json_data_ids.push(id.to_hex()?);
        }

        let obj = json::json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "timestamp": self.timestamp.to_string(),
            "inputs_length": self.inputs_length,
            "inputs": json_inputs,
            "outputs_amount": self.outputs_amount.to_string(),
            "outputs_length": self.outputs_length,
            "outputs": json_outputs,
            "data_size": self.data_size,
            "data_length": self.data_length,
            "data_ids": json_data_ids,
            "fee": self.fee.to_hex()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = Transaction::id_from_string(&id_str)?;
        
        let version_value = obj["version"].clone();
        let version_str: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_str)?;
        
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;
        
        let timestamp_value = obj["timestamp"].clone();
        let timestamp_str: String = json::from_value(timestamp_value)?;
        let timestamp = Timestamp::from_string(&timestamp_str)?;

        let inputs_length_value = obj["inputs_length"].clone();
        let inputs_length: u32 = json::from_value(inputs_length_value)?;

        let inputs_value = obj["inputs"].clone();
        let inputs_json: Vec<String> = json::from_value(inputs_value)?;

        let mut inputs = Vec::new();

        for input_json in inputs_json {
            let input = Input::from_json(&input_json)?;
            inputs.push(input);
        }

        let outputs_amount_value = obj["outputs_amount"].clone();
        let outputs_amount_str: String = json::from_value(outputs_amount_value)?;
        let outputs_amount = Amount::from_string(&outputs_amount_str)?;

        let outputs_length_value = obj["outputs_length"].clone();
        let outputs_length: u32 = json::from_value(outputs_length_value)?;

        let outputs_value = obj["outputs"].clone();
        let outputs_json: Vec<String> = json::from_value(outputs_value)?;

        let mut outputs = Vec::new();

        for output_json in outputs_json {
            let output = Output::from_json(&output_json)?;
            outputs.push(output);
        }

        let data_size_value = obj["data_size"].clone();
        let data_size: u32 = json::from_value(data_size_value)?;

        let data_length_value = obj["data_length"].clone();
        let data_length: u32 = json::from_value(data_length_value)?;

        let data_ids_value = obj["data_ids"].clone();
        let data_ids_json: Vec<String> = json::from_value(data_ids_value)?;

        let mut data_ids = Vec::new();

        for data_id_hex in data_ids_json {
            let id = Digest::from_hex(&data_id_hex)?;
            data_ids.push(id);
        }
        
        let fee_value = obj["fee"].clone();
        let fee_hex: String = json::from_value(fee_value)?;
        let fee = Amount::from_hex(&fee_hex)?;

        let transaction = Transaction {
            id: id,
            version: version,
            network_type: network_type,
            timestamp: timestamp,
            inputs_length: inputs_length,
            inputs: inputs,
            outputs_amount: outputs_amount,
            outputs_length: outputs_length,
            outputs: outputs,
            data_size: data_size,
            data_length: data_length,
            data_ids: data_ids,
            fee: fee,
        };

        Ok(transaction)
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
