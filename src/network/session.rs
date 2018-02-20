// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `session` module provides the Yobicash network session type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use crypto::{Random, Scalar, SecretKey, PublicKey};
use crypto::Validate as CryptoValidate;
use crypto::HexSerialize as CryptoHexSerialize;
use utils::{Version, NetworkType, Amount};

/// The type used for network sessions.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Session {
    /// The session id.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The secret key of this end of the connection.
    pub secret_key: SecretKey,
    /// The public key of the other end of the connection.
    pub public_key: Option<PublicKey>,
    /// The maximum size of a `Message`.
    pub max_size: Option<u32>,
    /// Difficulty of the `SynAck` PoW.
    pub pow_difficulty: Option<u32>,
    /// The fee_instance if this end requires a fee.
    pub fee_instance: Option<Scalar>,
    /// The fee_per_byte if this end requires a fee.
    pub fee_per_byte: Option<Amount>,
}

impl Session {
    /// Creates a new `Session`.
    pub fn new(network_type: NetworkType) -> Session {

        let id = Random::u32();
        let version = Version::default();
        let secret_key = SecretKey::random();

        Session {
            id: id,
            version: version,
            network_type: network_type,
            secret_key: secret_key,
            public_key: None,
            max_size: None,
            pow_difficulty: None,
            fee_instance: None,
            fee_per_byte: None,
        }
    }

    /// Adds the public key of the other side of the connection.
    pub fn add_public_key(&mut self, pk: PublicKey) -> Result<()> {
        pk.validate()?;

        if self.secret_key.to_public() == pk {
            return Err(ErrorKind::InvalidPublicKey.into());
        }

        self.public_key = Some(pk);

        Ok(())
    }

    /// Adds the `Message` maximum size.
    pub fn add_size(&mut self, max_size: u32) {
        self.max_size = Some(max_size);
    }

    /// Adds the pow params and difficulty.
    pub fn add_pow_difficulty(&mut self, pow_difficulty: u32) {
        self.pow_difficulty = Some(pow_difficulty);
    }

    /// Adds the fee instance and price per byte.
    pub fn add_fee_data(&mut self, fee_instance: Scalar, fee_per_byte: &Amount) -> Result<()> {
        fee_instance.validate()?;
        fee_per_byte.validate()?;

        self.fee_instance = Some(fee_instance);
        self.fee_per_byte = Some(fee_per_byte.clone());

        Ok(())
    }
}

impl Validate for Session {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.secret_key.validate()?;

        if let Some(public_key) = self.public_key {
            public_key.validate()?;
            if self.secret_key.to_public() == public_key {
                return Err(ErrorKind::InvalidPublicKey.into());
            }
        }
        
        if self.fee_instance.is_none() ^ self.fee_per_byte.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }

        if let Some(fee_instance) = self.fee_instance {
            fee_instance.validate()?;
        }

        if let Some(fee_per_byte) = self.fee_per_byte.clone() {
            fee_per_byte.validate()?;
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for Session {
    fn to_json(&self) -> Result<String> {
        let public_key_hex = if self.public_key.is_none() {
            String::from("")
        } else {
            self.public_key.unwrap().to_hex()?
        };

        let max_size_u32 = self.max_size.unwrap_or(0);

        let pow_difficulty_u32 = self.pow_difficulty.unwrap_or(0);

        let fee_instance_hex = if self.fee_instance.is_none() {
            String::from("")
        } else {
            self.fee_instance.unwrap().to_hex()?
        };

        let fee_per_byte_str = if self.fee_per_byte.is_none() {
            String::from("")
        } else {
            self.fee_per_byte.clone().unwrap().to_string()
        };

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "secret_key": self.secret_key.to_hex()?,
            "public_key": public_key_hex,
            "max_size": max_size_u32,
            "pow_difficulty": pow_difficulty_u32,
            "fee_instance": fee_instance_hex,
            "fee_per_byte": fee_per_byte_str,
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
      
        let secret_key_value = obj["secret_key"].clone();
        let secret_key_hex: String = json::from_value(secret_key_value)?;
        let secret_key = SecretKey::from_hex(&secret_key_hex)?;
      
        let public_key_value = obj["public_key"].clone();
        let public_key_hex: String = json::from_value(public_key_value)?;
        let public_key = if public_key_hex.is_empty() {
            None
        } else {
            Some(PublicKey::from_hex(&public_key_hex)?)
        };

        let max_size_value = obj["max_size"].clone();
        let max_size_u32: u32 = json::from_value(max_size_value)?;
        let max_size = if max_size_u32 == 0 {
            None
        } else {
            Some(max_size_u32)
        };

        let pow_difficulty_value = obj["pow_difficulty"].clone();
        let pow_difficulty_u32: u32 = json::from_value(pow_difficulty_value)?;
        let pow_difficulty = if pow_difficulty_u32 == 0 {
            None
        } else {
            Some(pow_difficulty_u32)
        };
      
        let fee_instance_value = obj["fee_instance"].clone();
        let fee_instance_hex: String = json::from_value(fee_instance_value)?;
        let fee_instance = if fee_instance_hex.is_empty() {
            None
        } else {
            Some(Scalar::from_hex(&fee_instance_hex)?)
        };
      
        let fee_per_byte_value = obj["fee_per_byte"].clone();
        let fee_per_byte_str: String = json::from_value(fee_per_byte_value)?;
        let fee_per_byte = if fee_per_byte_str.is_empty() {
            None
        } else {
            Some(Amount::from_string(&fee_per_byte_str)?)
        };

        let session = Session {
            id: id,
            version: version,
            network_type: network_type,
            secret_key: secret_key,
            public_key: public_key,
            max_size: max_size,
            pow_difficulty: pow_difficulty,
            fee_instance: fee_instance,
            fee_per_byte: fee_per_byte,
        };

        Ok(session)
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
