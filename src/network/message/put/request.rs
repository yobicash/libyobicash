// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `request` module provides the Yobicash network put request message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use utils::{Version, NetworkType};
use crypto::Digest;
use crypto::HexSerialize as CryptoHexSerialize;
use models::CoinSource;
use network::session::Session;
use network::resource_type::ResourceType;

/// The request to put a resource.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PutRequest {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The resource type.
    pub resource_type: ResourceType,
    /// The source if any.
    pub source: Option<CoinSource>,
    /// The source id if any.
    pub source_id: Option<Digest>,
    /// The write id if any.
    pub write_id: Option<Digest>,
    /// The resource payload.
    pub resource: Vec<u8>,
}

impl PutRequest {
    /// Creates a new `PutRequest`.
    pub fn new(session: &Session,
               resource_type: ResourceType,
               source: Option<CoinSource>,
               source_id: Option<Digest>,
               write_id: Option<Digest>,
               resource: &[u8]) -> Result<PutRequest> {
        session.validate()?;

        if source.is_some() ^ source.is_some() {
            return Err(ErrorKind::InvalidSource.into());
        }

        if source.is_some() && write_id.is_some() {
            return Err(ErrorKind::InvalidSource.into());
        }

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;

        let put_request = PutRequest {
            id: id,
            version: version,
            network_type: network_type,
            resource_type: resource_type,
            source: source,
            source_id: source_id,
            write_id: write_id,
            resource: Vec::from(resource),
        };

        Ok(put_request)
    }
}

impl Validate for PutRequest {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        if self.source.is_some() ^ self.source.is_some() {
            return Err(ErrorKind::InvalidMessage.into());
        }

        if self.source.is_some() && self.write_id.is_some() {
            return Err(ErrorKind::InvalidMessage.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for PutRequest {
    fn to_json(&self) -> Result<String> {

        let source_hex = if self.source.is_none() {
            String::from("")
        } else {
            self.source.unwrap().to_hex()?
        };

        let source_id_hex = if self.source_id.is_none() {
            String::from("")
        } else {
            self.source_id.unwrap().to_hex()?
        };

        let write_id_hex = if self.write_id.is_none() {
            String::from("")
        } else {
            self.write_id.unwrap().to_hex()?
        };

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "resource_type": self.resource_type.to_hex()?,
            "source": source_hex,
            "source_id": source_id_hex,
            "write_id": write_id_hex,
            "resource": hex::encode(&self.resource),
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

        let resource_type_value = obj["resource_type"].clone();
        let resource_type_hex: String = json::from_value(resource_type_value)?;
        let resource_type = ResourceType::from_hex(&resource_type_hex)?;
      
        let source_value = obj["source"].clone();
        let source_hex: String = json::from_value(source_value)?;
        let source = if source_hex.is_empty() {
            None
        } else {
            Some(CoinSource::from_hex(&source_hex)?)
        };
      
        let source_id_value = obj["source_id"].clone();
        let source_id_hex: String = json::from_value(source_id_value)?;
        let source_id = if source_id_hex.is_empty() {
            None
        } else {
            Some(Digest::from_hex(&source_id_hex)?)
        };
      
        let write_id_value = obj["write_id"].clone();
        let write_id_hex: String = json::from_value(write_id_value)?;
        let write_id = if write_id_hex.is_empty() {
            None
        } else {
            Some(Digest::from_hex(&write_id_hex)?)
        };
      
        let resource_value = obj["resource"].clone();
        let resource_hex: String = json::from_value(resource_value)?;
        let resource = hex::decode(&resource_hex)?;

        let put_request = PutRequest {
            id: id,
            version: version,
            network_type: network_type,
            resource_type: resource_type,
            source: source,
            source_id: source_id,
            write_id: write_id,
            resource: resource,
        };

        Ok(put_request)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let put_request = messagepack::from_slice(b)?;

        Ok(put_request)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
