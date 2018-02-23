// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `ack` module provides the Yobicash network ack message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use crypto::{PublicKey, ZKPWitness};
use crypto::Validate as CryptoValidate;
use crypto::HexSerialize as CryptoHexSerialize;
use utils::{Version, NetworkType, Amount};
use network::session::Session;

/// The type used for ack messages in the Yobicash handshake.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Ack {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// Public key of the server.
    pub public_key: PublicKey,
    /// Difficulty of the `SynAck` PoW.
    pub pow_difficulty: u32,
    /// Fee witness.
    pub fee_witness: Option<ZKPWitness>,
    /// Fee per byte if a payable put operation.
    pub fee_per_byte: Option<Amount>,
}

impl Ack {
    /// Creates a new `Ack`.
    pub fn new(session: &Session) -> Result<Ack> {
        session.validate()?;

        let fee_witness = if session.fee_instance.is_none() {
            None
        } else {
            Some(ZKPWitness::new(session.fee_instance.unwrap())?)
        };

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;
        let public_key = session.secret_key.to_public();
        let pow_difficulty = session.pow_difficulty.unwrap();
        let fee_witness = fee_witness;
        let fee_per_byte = session.fee_per_byte.clone();

        let ack = Ack {
            id: id,
            version: version,
            network_type: network_type,
            public_key: public_key,
            pow_difficulty: pow_difficulty,
            fee_witness: fee_witness,
            fee_per_byte: fee_per_byte,
        };

        Ok(ack)
    }
}

impl Validate for Ack {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.public_key.validate()?;
        
        if self.fee_witness.is_none() && self.fee_per_byte.is_none() {
            return Err(ErrorKind::InvalidMessage.into());
        }

        if let Some(fee_witness) = self.fee_witness {
            fee_witness.validate()?;
        }

        if let Some(fee_per_byte) = self.fee_per_byte.clone() {
            fee_per_byte.validate()?;
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for Ack {
    fn to_json(&self) -> Result<String> {
        let fee_witness = if self.fee_witness.is_none() {
            String::from("")
        } else {
            self.fee_witness.unwrap().to_hex()?
        };

        let fee_per_byte = if self.fee_per_byte.is_none() {
            String::from("")
        } else {
            self.fee_per_byte.clone().unwrap().to_string()
        };

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "public_key": self.public_key.to_hex()?,
            "pow_difficulty": self.pow_difficulty,
            "fee_witness": fee_witness,
            "fee_per_byte": fee_per_byte,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
      
        let id_value = obj["id"].clone();
        let id: u32 = json::from_value(id_value)?;
      
        let version_value = obj["version"].clone();
        let version_str: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_str)?;
      
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;
      
        let public_key_value = obj["public_key"].clone();
        let public_key_hex: String = json::from_value(public_key_value)?;
        let public_key = PublicKey::from_hex(&public_key_hex)?;

        let pow_difficulty_value = obj["pow_difficulty"].clone();
        let pow_difficulty: u32 = json::from_value(pow_difficulty_value)?;
      
        let fee_witness_value = obj["fee_witness"].clone();
        let fee_witness_hex: String = json::from_value(fee_witness_value)?;
        let fee_witness = if fee_witness_hex.is_empty() {
            None
        } else {
            Some(ZKPWitness::from_hex(&fee_witness_hex)?)
        };
      
        let fee_per_byte_value = obj["fee_per_byte"].clone();
        let fee_per_byte_str: String = json::from_value(fee_per_byte_value)?;
        let fee_per_byte = if fee_per_byte_str.is_empty() {
            None
        } else {
            Some(Amount::from_string(&fee_per_byte_str)?)
        };
      
        let ack = Ack {
            id: id,
            version: version,
            network_type: network_type,
            public_key: public_key,
            pow_difficulty: pow_difficulty,
            fee_witness: fee_witness,
            fee_per_byte: fee_per_byte,
        };

        Ok(ack)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let ack = messagepack::from_slice(b)?;

        Ok(ack)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
