// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `syn` module provides the Yobicash network syn message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use crypto::PublicKey;
use crypto::Validate as CryptoValidate;
use crypto::HexSerialize as CryptoHexSerialize;
use utils::{Version, NetworkType};
use network::session::Session;

/// The type used for syn messages in the Yobicash handshake.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Syn {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// Public key of the client.
    pub public_key: PublicKey,
}

impl Syn {
    /// Create a new `Syn`.
    pub fn new(session: &Session) -> Result<Syn> {
        session.validate()?;
        
        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;
        let public_key = session.secret_key.to_public();

        let syn = Syn {
            id: id,
            version: version,
            network_type: network_type,
            public_key: public_key,
        };

        Ok(syn)
    }
}

impl Validate for Syn {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;
        self.public_key.validate()?;

        // TODO: import validations from client.rs

        Ok(())
    }
}

impl<'a> Serialize<'a> for Syn {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "public_key": self.public_key.to_hex()?,
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

        let syn = Syn {
            id: id,
            version: version,
            network_type: network_type,
            public_key: public_key,
        };

        Ok(syn)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let syn = messagepack::from_slice(b)?;

        Ok(syn)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
