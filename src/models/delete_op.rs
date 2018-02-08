// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `delete_op` module provides the delete operation operation types and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};
use itertools::Itertools;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, Serialize};
use utils::{Version, Timestamp, Amount};
use crypto::{Random, Digest, Scalar, ZKPProof};
use crypto::Validate as CryptoValidate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::coin::Coin;
use models::input::Input;
use models::write_op::WriteOp;

use std::io::Write;

/// A type representing a delete opearation done on the Yobicash dagchain.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeleteOp {
    /// The id of the output.
    pub id: Digest,
    /// The version of the library.
    pub version: Version,
    /// The unix timestamp of the time the write was created.
    pub timestamp: Timestamp,
    /// The size of the inputs.
    pub inputs_length: u32,
    /// The inputs referencing the outputs to spend.
    pub inputs: Vec<Input>,
    /// The data size.
    pub data_size: u32,
    /// The write operation.
    pub write_id: Digest,
    /// A nonce for the zero-knowledge proof.
    pub nonce: u64,
    /// The zero-knowledge proof proof of the write operation.
    pub proof: ZKPProof,
    /// The fee output of the write.
    pub fee: Output,
}

impl DeleteOp {
    /// Creates a new `DeleteOp`.
    pub fn new(coins: &[Coin],
               write_op: &WriteOp,
               proof: ZKPProof,
               fee: &Output) -> Result<DeleteOp> {
        for coin in coins {
            coin.validate()?;
        }

        write_op.validate()?;
    
        proof.validate()?;
        
        if !proof.verify(write_op.witness)? {
            return Err(ErrorKind::InvalidProof.into());
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

        if coins_amount != fee.amount  {
            return Err(ErrorKind::OutOfBound.into());
        }

        let version = Version::default();
        let timestamp = Timestamp::now();

        let outputs_ids = vec![];

        let mut inputs = Vec::new();
        for i in 0..coins_length {
            let coin = &coins[i];

            let input = Input::new(
                coin,
                &version,
                timestamp,
                &outputs_ids,
                fee.id)?;

            inputs.push(input);
        }

        let mut d_op = DeleteOp::default();

        d_op.timestamp = timestamp;
        d_op.inputs_length = inputs.len() as u32;
        d_op.inputs = inputs;
        d_op.data_size = write_op.data_size;
        d_op.write_id = write_op.id;
        d_op.proof = proof;
        d_op.fee = fee.clone();
        d_op.id = d_op.id()?;

        Ok(d_op)
    }
    
    /// Creates the proof required to build the `DeleteOp`.
    pub fn proof(write_op: &WriteOp,
                 instance: Scalar,
                 fee: &Output) -> Result<ZKPProof> {
        write_op.validate()?;
        instance.validate()?;
        fee.validate()?;

        let version = Version::default();
        let timestamp = Timestamp::now();

        let nonce = Random::u64();

        let mut nonce_buf = Vec::new();
        nonce_buf.write_u64::<BigEndian>(nonce)?;

        let mut message = Vec::new();
        
        message.extend_from_slice(&nonce_buf);
        message.extend_from_slice(&version.to_bytes()?);
        message.extend_from_slice(&timestamp.to_bytes()?);
        message.extend_from_slice(&write_op.id.to_bytes()?);
        message.extend_from_slice(&fee.id.to_bytes()?);
        
        Ok(ZKPProof::new(instance, &message)?)
    }


    /// Returns the fee amount.
    pub fn fee_amount(&self) -> Amount {
        self.fee.amount.clone()
    }

    /// Verifies the `DeleteOp` against a `WriteOp`.
    pub fn verify(&self, write_op: &WriteOp) -> Result<bool> {
        self.validate()?;
        write_op.validate()?;

        Ok(self.proof.verify(write_op.witness)?)
    }
}

impl Default for DeleteOp {
    fn default() -> DeleteOp {
        DeleteOp {
            id: Digest::default(),
            version: Version::default(),
            timestamp: Timestamp::default(),
            inputs_length: 0,
            inputs: Vec::new(),
            data_size: 0,
            write_id: Digest::default(),
            nonce: 0,
            proof: ZKPProof::default(),
            fee: Output::default(),
        }
    }
}

impl<'a> Identify<'a> for DeleteOp {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.timestamp.to_bytes()?)?;
        buf.write_u32::<BigEndian>(self.inputs_length)?;
        
        for i in 0..self.inputs_length as usize {
            let input = self.inputs[i];
            let input_buf = input.to_bytes()?;
            buf.write_all(&input_buf)?;
        }
        
        buf.write_u32::<BigEndian>(self.data_size)?;
        buf.write_all(&self.write_id.to_bytes()?)?;
        buf.write_u64::<BigEndian>(self.nonce)?;
        buf.write_all(&self.proof.to_bytes()?)?;
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

impl Validate for DeleteOp {
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

        self.proof.validate()?;
        
        self.fee.validate()?;
        
        Ok(())
    }
}

impl<'a> Serialize<'a> for DeleteOp {
    fn to_json(&self) -> Result<String> {
        let mut json_inputs = Vec::new();
        for input in self.inputs.clone() {
            json_inputs.push(input.to_json()?);
        }

        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "timestamp": self.timestamp.to_string(),
            "inputs_length": self.inputs_length,
            "inputs": json_inputs,
            "data_size": self.data_size,
            "write_id": self.write_id.to_hex()?,
            "nonce": self.nonce,
            "proof": self.proof.to_hex()?,
            "fee": self.fee.to_json()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = DeleteOp::id_from_string(&id_str)?;
        
        let version_value = obj["version"].clone();
        let version_string: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_string)?;
        
        let timestamp_value = obj["timestamp"].clone();
        let timestamp_string: String = json::from_value(timestamp_value)?;
        let timestamp = Timestamp::from_string(&timestamp_string)?;

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
        
        let write_id_value = obj["write_id"].clone();
        let write_id_hex: String = json::from_value(write_id_value)?;
        let write_id = Digest::from_hex(&write_id_hex)?;

        let nonce_value = obj["nonce"].clone();
        let nonce: u64 = json::from_value(nonce_value)?;
        
        let proof_value = obj["proof"].clone();
        let proof_hex: String = json::from_value(proof_value)?;
        let proof = ZKPProof::from_hex(&proof_hex)?;
        
        let fee_value = obj["fee"].clone();
        let fee_json: String = json::from_value(fee_value)?;
        let fee = Output::from_json(&fee_json)?;

        let delete_op = DeleteOp {
            id: id,
            version: version,
            timestamp: timestamp,
            inputs_length: inputs_length,
            inputs: inputs,
            data_size: data_size,
            write_id: write_id,
            nonce: nonce,
            proof: proof,
            fee: fee,
        };

        Ok(delete_op)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let delete_op = messagepack::from_slice(b)?;

        Ok(delete_op)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
