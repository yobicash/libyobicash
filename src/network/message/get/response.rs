// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `response` module provides the Yobicash network get response message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use utils::{Version, NetworkType};
use network::session::Session;
use network::resource_type::ResourceType;

/// The successful response to a get request message.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetResponse {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The resource type.
    pub resource_type: ResourceType,
    /// The resource payload.
    pub resource: Vec<u8>,
}

impl GetResponse {
    /// Creates a new `GetResponse`.
    pub fn new(session: &Session,
               resource_type: ResourceType,
               resource: &[u8]) -> Result<GetResponse> {
        session.validate()?;

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;

        let get_response = GetResponse {
            id: id,
            version: version,
            network_type: network_type,
            resource_type: resource_type,
            resource: Vec::from(resource),
        };

        Ok(get_response)
    }
}

impl Validate for GetResponse {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        Ok(())
    }
}

impl<'a> Serialize<'a> for GetResponse {
    fn to_json(&self) -> Result<String> {

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "resource_type": self.resource_type.to_hex()?,
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
      
        let resource_value = obj["resource"].clone();
        let resource_hex: String = json::from_value(resource_value)?;
        let resource = hex::decode(&resource_hex)?;

        let get_response = GetResponse {
            id: id,
            version: version,
            network_type: network_type,
            resource_type: resource_type,
            resource: resource,
        };

        Ok(get_response)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let get_response = messagepack::from_slice(b)?;

        Ok(get_response)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
