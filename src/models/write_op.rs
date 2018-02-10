// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `write_op` module provides the write operation types and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use utils::{Version, NetworkType,Timestamp, Amount};
use crypto::{Digest, Scalar, ZKPWitness};
use crypto::Validate as CryptoValidate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::coin::Coin;
use models::input::Input;
use models::data::Data;
use models::delete_op::DeleteOp;

use std::io::Write;

/// A type representing a write opearation done on the Yobicash dagchain.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct WriteOp {
    /// The id of the output.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The unix timestamp of the time the write was created.
    pub timestamp: Timestamp,
    /// The size of the inputs.
    pub inputs_length: u32,
    /// The inputs referencing the outputs to spend.
    pub inputs: Vec<Input>,
    /// The data size.
    pub data_size: u32,
    /// The encrypted data id.
    pub data_id: Digest,
    /// The zero-knowledge proof witness of the write operation.
    pub witness: ZKPWitness,
    /// The fee output of the write.
    pub fee: Output,
}

impl WriteOp {
    /// Creates a new `WriteOp`.
    pub fn new(network_type: NetworkType,
               coins: &[Coin],
               data: &Data,
               instance: Scalar,
               fee: &Output) -> Result<WriteOp> {
        for coin in coins {
            coin.validate()?;
            if coin.output.network_type != network_type {
                return Err(ErrorKind::InvalidNetwork.into());
            }
        }

        data.validate()?;
    
        fee.validate()?;
        if fee.network_type != network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        let coins_length = coins.len();

        let mut coins_ids = Vec::new();
        for coin in coins {
            if coin.instance == instance {
                return Err(ErrorKind::DuplicatesFound.into());
            }

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

        if coins_amount != fee.amount {
            return Err(ErrorKind::OutOfBound.into());
        }

        let timestamp = Timestamp::now();

        let mut inputs = Vec::new();
        let outputs_ids = vec![data.id];
        for i in 0..coins_length {
            let coin = &coins[i];

            let input = Input::new(
                coin,
                &Version::default(),
                timestamp,
                &outputs_ids,
                fee)?;

            inputs.push(input);
        }

        let mut w_op = WriteOp::default();
        w_op.network_type = network_type;
        w_op.timestamp = timestamp;
        w_op.inputs_length = inputs.len() as u32;
        w_op.inputs = inputs;
        w_op.data_size = data.to_bytes()?.len() as u32;
        w_op.data_id = data.id;
        w_op.witness = ZKPWitness::new(instance)?;
        w_op.fee = fee.clone();
        w_op.id = w_op.id()?;

        Ok(w_op)
    }

    /// Returns the fee amount.
    pub fn fee_amount(&self) -> Amount {
        self.fee.amount.clone()
    }

    /// Verifies the `WriteOp` against a `DeleteOp`.
    pub fn verify(&self, delete_op: &DeleteOp) -> Result<bool> {
        self.validate()?;
        
        delete_op.validate()?;
        if delete_op.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }

        Ok(delete_op.proof.verify(self.witness)?)
    }
}

impl Default for WriteOp {
    fn default() -> WriteOp {
        let data = Data::default();
        let data_size = data.to_bytes().unwrap().len() as u32;
        WriteOp {
            id: Digest::default(),
            version: Version::default(),
            network_type: NetworkType::default(),
            timestamp: Timestamp::default(),
            inputs_length: 0,
            inputs: Vec::new(),
            data_size: data_size,
            data_id: data.id,
            witness: ZKPWitness::default(),
            fee: Output::default(),
        }
    }
}

impl<'a> Identify<'a> for WriteOp {
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
        
        buf.write_u32::<BigEndian>(self.data_size)?;
        buf.write_all(&self.data_id.to_bytes()?)?;
        buf.write_all(&self.witness.to_bytes()?)?;
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

impl Validate for WriteOp {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.timestamp.validate()?;

        if self.inputs_length as usize != self.inputs.len() {
            return Err(ErrorKind::InvalidLength.into()); 
        }

        let mut inputs_ids = Vec::new();
        for input in self.inputs.clone() {
            input.validate()?;
            inputs_ids.push(input.binary_id()?);
        }

        if inputs_ids.iter().unique().count() != self.inputs_length as usize {
            return Err(ErrorKind::DuplicatesFound.into());
        }

        self.witness.validate()?;
        
        self.fee.validate()?;
        if self.fee.network_type != self.network_type {
            return Err(ErrorKind::InvalidNetwork.into());
        }
        
        Ok(())
    }
}

impl<'a> Serialize<'a> for WriteOp {
    fn to_json(&self) -> Result<String> {
        let mut json_inputs = Vec::new();
        for input in self.inputs.clone() {
            json_inputs.push(input.to_json()?);
        }

        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "timestamp": self.timestamp.to_string(),
            "inputs_length": self.inputs_length,
            "inputs": json_inputs,
            "data_size": self.data_size,
            "data_id": self.data_id.to_hex()?,
            "witness": self.witness.to_hex()?,
            "fee": self.fee.to_json()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = WriteOp::id_from_string(&id_str)?;
        
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

        let data_size_value = obj["data_size"].clone();
        let data_size: u32 = json::from_value(data_size_value)?;
        
        let data_id_value = obj["data_id"].clone();
        let data_id_hex: String = json::from_value(data_id_value)?;
        let data_id = Digest::from_hex(&data_id_hex)?;
        
        let witness_value = obj["witness"].clone();
        let witness_hex: String = json::from_value(witness_value)?;
        let witness = ZKPWitness::from_hex(&witness_hex)?;
        
        let fee_value = obj["fee"].clone();
        let fee_json: String = json::from_value(fee_value)?;
        let fee = Output::from_json(&fee_json)?;

        let write_op = WriteOp {
            id: id,
            version: version,
            network_type: network_type,
            timestamp: timestamp,
            inputs_length: inputs_length,
            inputs: inputs,
            data_size: data_size,
            data_id: data_id,
            witness: witness,
            fee: fee,
        };

        Ok(write_op)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let write_op = messagepack::from_slice(b)?;

        Ok(write_op)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
