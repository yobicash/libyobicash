// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `response_header` module provides the Yobicash network response-header message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use constants::MAX_CHUNK_SIZE;
use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use crypto::Digest;
use crypto::HexSerialize as CryptoHexSerialize;
use utils::{Version, NetworkType};
use network::session::Session;

/// The type used for syn-ack messages in the Yobicash handshake.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct ResponseHeader {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// Size of the plaintext.
    pub plain_size: u32,
    /// Size of the cyphertext.
    pub cyph_size: u32,
    /// Padding of the cyphertext.
    pub padding: u32,
    /// Number of incoming chunks.
    pub chunks_count: u32,
    /// Checksum of the plaintext.
    pub plain_digest: Digest,
    /// Checksum of the cyphertext.
    pub cyph_digest: Digest,
}

impl ResponseHeader {
    /// Creates a new `ResponseHeader`.
    pub fn new(session: &Session,
               plain_size: u32,
               cyph_size: u32,
               padding: u32,
               plain_digest: Digest,
               cyph_digest: Digest) -> Result<ResponseHeader> {
        session.validate()?;

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;

        if plain_size > cyph_size {
            return Err(ErrorKind::OutOfBound.into());
        }

        if padding != cyph_size % MAX_CHUNK_SIZE {
            return Err(ErrorKind::InvalidLength.into());
        }

        let chunks_count = (cyph_size + padding) / MAX_CHUNK_SIZE;
        
        if plain_digest == cyph_digest {
            return Err(ErrorKind::InvalidDigest.into());
        }

        let response_header = ResponseHeader {
            id: id,
            version: version,
            network_type: network_type,
            plain_size: plain_size,
            cyph_size: cyph_size,
            padding: padding,
            chunks_count: chunks_count,
            plain_digest: plain_digest,
            cyph_digest: cyph_digest,
        };

        Ok(response_header)
    }
}

impl Validate for ResponseHeader {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        let plain_size = self.plain_size;
        let cyph_size = self.cyph_size;
        let padding = self.padding;
        let chunks_count = self.chunks_count;
        let plain_digest = self.plain_digest;
        let cyph_digest = self.cyph_digest;

        if plain_size > cyph_size {
            return Err(ErrorKind::OutOfBound.into());
        }

        if padding != cyph_size % MAX_CHUNK_SIZE {
            return Err(ErrorKind::InvalidLength.into());
        }

        if chunks_count != (cyph_size + padding) / MAX_CHUNK_SIZE {
            return Err(ErrorKind::InvalidLength.into());
        }
        
        if plain_digest == cyph_digest {
            return Err(ErrorKind::InvalidDigest.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for ResponseHeader {
    fn to_json(&self) -> Result<String> {
        
        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "plain_size": self.plain_size,
            "cyph_size": self.cyph_size,
            "padding": self.padding,
            "chunks_count": self.chunks_count,
            "plain_digest": self.plain_digest.to_hex()?,
            "cyph_digest": self.cyph_digest.to_hex()?,
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

        let plain_size_value = obj["plain_size"].clone();
        let plain_size: u32 = json::from_value(plain_size_value)?;

        let cyph_size_value = obj["cyph_size"].clone();
        let cyph_size: u32 = json::from_value(cyph_size_value)?;

        let padding_value = obj["padding"].clone();
        let padding: u32 = json::from_value(padding_value)?;

        let chunks_count_value = obj["chunks_count"].clone();
        let chunks_count: u32 = json::from_value(chunks_count_value)?;
      
        let plain_digest_value = obj["plain_digest"].clone();
        let plain_digest_hex: String = json::from_value(plain_digest_value)?;
        let plain_digest = Digest::from_hex(&plain_digest_hex)?;
      
        let cyph_digest_value = obj["cyph_digest"].clone();
        let cyph_digest_hex: String = json::from_value(cyph_digest_value)?;
        let cyph_digest = Digest::from_hex(&cyph_digest_hex)?;

        let response_header = ResponseHeader {
            id: id,
            version: version,
            network_type: network_type,
            plain_size: plain_size,
            cyph_size: cyph_size,
            padding: padding,
            chunks_count: chunks_count,
            plain_digest: plain_digest,
            cyph_digest: cyph_digest,
        };

        Ok(response_header)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let response_header = messagepack::from_slice(b)?;

        Ok(response_header)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
