// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `input` module provides the transaction input type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use result::Result;
use traits::{Identify, Validate, Serialize};
use crypto::{Digest, ZKPProof};
use crypto::Validate as CryptoValidate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use models::output::Output;
use models::coin::Coin;

/// An `Input` is a reference to a past output used in transactions
/// to spend the output.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Input {
    /// The id of the referenced output.
    pub id: Digest,
    /// The zero-knowledge-proof proof used to spend the referenced output.
    pub proof: ZKPProof,
}

impl Input {
    /// Creates an `Input`.
    pub fn new(coin: &Coin, message: &[u8]) -> Result<Input> {
        coin.validate()?;

        let proof = coin.proof(message)?;

        let input = Input {
            id: coin.id,
            proof: proof,
        };

        Ok(input)
    }

    /// Verifies the `Input` against an `Output`.
    pub fn verify(&self, output: &Output) -> Result<bool> {
        self.validate()?;
        output.validate()?;

        Ok(self.proof.verify(output.witness)?)
    }
}

impl<'a> Identify<'a> for Input {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        Ok(self.id)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
        Ok(id.to_bytes()?)
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(self.id()?.to_bytes()?)
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

impl Validate for Input {
    fn validate(&self) -> Result<()> {
        self.proof.validate()?;

        Ok(())
    }
}

impl<'a> Serialize<'a> for Input {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.id.to_hex()?,
            "proof": self.proof.to_hex()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_hex: String = json::from_value(id_value)?;
        let id = Digest::from_hex(&id_hex)?;

        let proof_value = obj["proof"].clone();
        let proof_hex: String = json::from_value(proof_value)?;
        let proof = ZKPProof::from_hex(&proof_hex)?;

        let input = Input {
            id: id,
            proof: proof,
        };

        Ok(input)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let input = messagepack::from_slice(b)?;

        Ok(input)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
