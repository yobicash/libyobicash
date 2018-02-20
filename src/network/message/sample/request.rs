// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `request` module provides the Yobicash network list request message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use utils::{Version, NetworkType};
use crypto::Digest;
use network::session::Session;
use network::resource_type::ResourceType;

/// The request for sampling a resource.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SampleRequest {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The maximum size of the `SampleRequest` message.
    pub max_size: u32,
    /// The resource type.
    pub resource_type: ResourceType,
    /// The maximum number of retrieved resources.
    pub count: u32,
}

impl SampleRequest {
    /// Creates a new `SampleRequest`.
    pub fn new(session: &Session,
               resource_type: ResourceType,
               count: u32) -> Result<SampleRequest> {
        session.validate()?;

        if session.max_size.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }
        
        let max_size = session.max_size.unwrap();

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;

        let sample_request = SampleRequest {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            resource_type: resource_type,
            count: count,
        };

        if sample_request.to_bytes()?.len() as u32 > max_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(sample_request)
    }
}

impl Validate for SampleRequest {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        if self.to_bytes()?.len() as u32 > self.max_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for SampleRequest {
    fn to_json(&self) -> Result<String> {

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "max_size": self.max_size,
            "resource_type": self.resource_type.to_hex()?,
            "count": self.count,
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

        let max_size_value = obj["max_size"].clone();
        let max_size: u32 = json::from_value(max_size_value)?;
      
        let resource_type_value = obj["resource_type"].clone();
        let resource_type_hex: String = json::from_value(resource_type_value)?;
        let resource_type = ResourceType::from_hex(&resource_type_hex)?;

        let count_value = obj["count"].clone();
        let count: u32 = json::from_value(count_value)?;

        let sample_request = SampleRequest {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            resource_type: resource_type,
            count: count,
        };

        Ok(sample_request)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let sample_request = messagepack::from_slice(b)?;

        Ok(sample_request)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
