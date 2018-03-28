// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `coin` module provides the coin type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, Identify, Serialize};
use crypto::{Digest, Scalar, ZKPWitness, ZKPProof};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoHexSerialize;
use crypto::Validate as CryptoValidate;
use utils::Amount;
use models::output::Output;

/// A `Coin` is an `Output` enriched with the instance needed to redeem it.
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Coin {
    /// The output id.
    pub id: Digest,
    /// The instance used to build the output.
    pub instance: Scalar,
    /// The witness used in the output.
    pub witness: ZKPWitness,
    /// The output amount.
    pub amount: Amount,
}

impl Coin {
    /// Creates a new `Coin`.
    pub fn new(output: &Output, instance: Scalar) -> Result<Coin> {
        output.validate()?;
        instance.validate()?;

        let witness = ZKPWitness::new(instance)?;
        if witness != output.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }

        let coin = Coin {
            id: output.id,
            instance: instance,
            witness: witness,
            amount: output.amount.clone(),
        };

        Ok(coin)
    }

    /// Verify the `Coin` against a `ZKPProof`.
    pub fn verify(&self, proof: ZKPProof) -> Result<bool> {
        self.validate()?;
        proof.validate()?;

        let witness = ZKPWitness::new(self.instance)?;

        Ok(proof.verify(witness)?)
    }

    /// Creates a proof from a message. It is used to build an `Input` from the `Coin`.
    pub fn proof(&self, message: &[u8]) -> Result<ZKPProof> {
        self.validate()?;

        Ok(ZKPProof::new(self.instance, message)?)
    }
}

impl<'a> Identify<'a> for Coin {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        Ok(self.id)
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
       Ok(id.to_bytes()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(self.id.to_bytes()?)
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

impl Validate for Coin {
    fn validate(&self) -> Result<()> {
        self.instance.validate()?;
        self.witness.validate()?;

        let witness = ZKPWitness::new(self.instance)?;

        if witness != self.witness {
            return Err(ErrorKind::InvalidWitness.into());
        }
    
        Ok(())
    }
}

impl<'a> Serialize<'a> for Coin {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.string_id()?,
            "instance": self.instance.to_hex()?,
            "witness": self.witness.to_hex()?,
            "amount": self.amount.to_string(),
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_hex: String = json::from_value(id_value)?;
        let id = Digest::from_hex(&id_hex)?;
        
        let instance_value = obj["instance"].clone();
        let instance_hex: String = json::from_value(instance_value)?;
        let instance = Scalar::from_hex(&instance_hex)?;
        
        let witness_value = obj["witness"].clone();
        let witness_hex: String = json::from_value(witness_value)?;
        let witness = ZKPWitness::from_hex(&witness_hex)?;

        let amount_value = obj["amount"].clone();
        let amount_str: String = json::from_value(amount_value)?;
        let amount = Amount::from_string(&amount_str)?;

        let coin = Coin {
            id: id,
            instance: instance,
            witness: witness,
            amount: amount,
        };

        Ok(coin)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let coin = messagepack::from_slice(b)?;

        Ok(coin)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
