// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `error` module provides the Yobicash network error response message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use utils::{Version, NetworkType};
use network::session::Session;

/// The error_response message type.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The maximum size of the `ErrorResponse` message.
    pub max_size: u32,
    /// The error message.
    pub message: String,
}

impl ErrorResponse {
    /// Creates a new `ErrorResponse`.
    pub fn new(session: &Session, message: &str) -> Result<ErrorResponse> {
        session.validate()?;

        if session.max_size.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }
        
        let max_size = session.max_size.unwrap();

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;

        let error_response = ErrorResponse {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            message: String::from(message),
        };

        if error_response.to_bytes()?.len() as u32 > max_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(error_response)
    }
}

impl Validate for ErrorResponse {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        if self.to_bytes()?.len() as u32 > self.max_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for ErrorResponse {
    fn to_json(&self) -> Result<String> {

        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "max_size": self.max_size,
            "message": self.message,
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

        let message_value = obj["message"].clone();
        let message: String = json::from_value(message_value)?;

        let error_response = ErrorResponse {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            message: message,
        };

        Ok(error_response)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let error_response = messagepack::from_slice(b)?;

        Ok(error_response)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
