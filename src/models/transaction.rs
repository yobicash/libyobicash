// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `transaction` module provides the transaction types and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use utils::{Version, NetworkType, Timestamp, Amount};
use crypto::Digest;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::coin::Coin;
use models::input::Input;

use std::io::Write;

/// Transaction is a transfer of balance from the inputs of
/// (generally) one user to one or more users.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// The id of the output.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The unix timestamp of the time the transaction was created.
    pub timestamp: Timestamp,
    /// The size of the inputs.
    pub inputs_length: u32,
    /// The inputs referencing the outputs to spend.
    pub inputs: Vec<Input>,
    /// The outputs amount.
    pub outputs_amount: Amount,
    /// The length of the outputs.
    pub outputs_length: u32,
    /// The sent outputs ids.
    pub outputs_ids: Vec<Digest>,
    /// The fee output of the transaction.
    pub fee: Output,
}

impl Transaction {
    /// Creates a new `Transaction`.
    pub fn new(network_type: NetworkType,
               coins: &[Coin],
               outputs: &[Output],
               fee: &Output) -> Result<Transaction> {
        for coin in coins {
            coin.validate()?;
            if coin.network_type != network_type {
                return Err(ErrorKind::InvalidNetwork.into());
            }
        }

        for output in outputs {
            output.validate()?;
        }
        
        fee.validate()?;

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
            coins_amount += &coin.output.amount;
        }

        let mut outputs_ids = Vec::new();
        let mut outputs_amount = Amount::new();
        for output in outputs {
            outputs_ids.push(output.id);
            outputs_amount += &output.amount;
        }

        let mut outputs_binary_ids = Vec::new();
        for id in outputs_ids.clone() {
            outputs_binary_ids.push(id.to_bytes()?);
        }

        if outputs_binary_ids.iter().unique().count() != outputs.len() {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        if coins_amount != &outputs_amount + &fee.amount {
            return Err(ErrorKind::OutOfBound.into());
        }

        let timestamp = Timestamp::now();

        let mut inputs = Vec::new();
        for i in 0..coins_length {
            let coin = &coins[i];

            let input = Input::new(
                coin,
                &Version::default(),
                timestamp,
                &outputs_ids,
                fee.id)?;

            inputs.push(input);
        }

        let mut tx = Transaction::default();
        tx.network_type = network_type;
        tx.timestamp = timestamp;
        tx.inputs_length = inputs.len() as u32;
        tx.inputs = inputs;
        tx.outputs_amount = outputs_amount;
        tx.outputs_length = outputs.len() as u32;
        tx.outputs_ids = outputs_ids;
        tx.fee = fee.clone();
        tx.id = tx.id()?;

        Ok(tx)
    }

    /// Returns the fee amount.
    pub fn fee_amount(&self) -> Amount {
        self.fee.amount.clone()
    }

    /// Returns the total amount sent.
    pub fn total_amount(&self) -> Amount {
        &self.outputs_amount + &self.fee_amount()
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
            outputs_ids: Vec::new(),
            fee: Output::default(),
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
            let id = &self.outputs_ids[i];
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

        if self.inputs_length as usize != self.inputs.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        self.outputs_amount.validate()?;

        if self.outputs_length as usize != self.outputs_ids.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        let mut inputs_binary_ids = Vec::new();
        for input in self.inputs.clone() {
            input.validate()?;
            inputs_binary_ids.push(input.binary_id()?);
        }

        if inputs_binary_ids.iter().unique().count() != self.inputs_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }

        let mut outputs_binary_ids = Vec::new();
        for id in self.outputs_ids.clone() {
            outputs_binary_ids.push(id.to_bytes()?);
        }

        if outputs_binary_ids.iter().unique().count() != self.outputs_length as usize {
            return Err(ErrorKind::DuplicatesFound.into()); 
        }
        
        self.fee.validate()?;
        
        Ok(())
    }
}

impl<'a> Serialize<'a> for Transaction {
    fn to_json(&self) -> Result<String> {
        let mut json_inputs = Vec::new();
        for input in self.inputs.clone() {
            json_inputs.push(input.to_json()?);
        }

        let mut json_outputs_ids = Vec::new();
        for id in self.outputs_ids.clone() {
            json_outputs_ids.push(id.to_hex()?);
        }

        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "timestamp": self.timestamp.to_string(),
            "inputs_length": self.inputs_length,
            "inputs": json_inputs,
            "outputs_amount": self.outputs_amount.to_string(),
            "outputs_length": self.outputs_length,
            "outputs_ids": json_outputs_ids,
            "fee": self.fee.to_json()?,
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

        let outputs_ids_value = obj["outputs_ids"].clone();
        let outputs_ids_json: Vec<String> = json::from_value(outputs_ids_value)?;

        let mut outputs_ids = Vec::new();

        for output_id_hex in outputs_ids_json {
            let id = Digest::from_hex(&output_id_hex)?;
            outputs_ids.push(id);
        }
        
        let fee_value = obj["fee"].clone();
        let fee_json: String = json::from_value(fee_value)?;
        let fee = Output::from_json(&fee_json)?;

        let transaction = Transaction {
            id: id,
            version: version,
            network_type: network_type,
            timestamp: timestamp,
            inputs_length: inputs_length,
            inputs: inputs,
            outputs_amount: outputs_amount,
            outputs_length: outputs_length,
            outputs_ids: outputs_ids,
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
