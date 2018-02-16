// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `peer` module provides the peer type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, Serialize};
use utils::Timestamp;

/// A peer is a node on the Yobicash peer-to-peer network.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Peer {
    /// The address of the peer.
    pub address: String,
    /// Blacklist flag.
    pub blacklisted: bool,
    /// The unix timestamp of the time the peer has been created.
    pub created_at: Timestamp,
    /// The unix timestamp of the last time the peer has been updated.
    pub updated_at: Timestamp,
}

impl Peer {
    /// Creates a new `Peer`.
    pub fn new(address: &str) -> Result<Peer> {
        let mut peer = Peer::default();
        peer.address = address.into();

        Ok(peer)
    }

    /// Blacklist the peer.
    pub fn blacklist(&mut self) -> Result<()> {
        self.blacklisted = true;

        Ok(())
    }

    /// Whitelist the peer.
    pub fn whitelist(&mut self) -> Result<()> {
        self.blacklisted = false;

        Ok(())
    }

    /// Updates the last time the `Peer` has been seen.
    pub fn seen(&mut self) -> Result<()> {
        self.validate()?;
        
        self.updated_at = Timestamp::now();

        Ok(())
    }
}

impl<'a> Serialize<'a> for Peer {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "address": self.address,
            "blacklisted": self.blacklisted,
            "created_at": self.created_at.to_string(),
            "updated_at": self.updated_at.to_string(),
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
       
        let address_value = obj["address"].clone();
        let address: String = json::from_value(address_value)?;

        let blacklisted_value = obj["blacklisted"].clone();
        let blacklisted: bool = json::from_value(blacklisted_value)?;
        
        let created_at_value = obj["created_at"].clone();
        let created_at_str: String = json::from_value(created_at_value)?;
        let created_at = Timestamp::from_string(&created_at_str)?;
        
        let updated_at_value = obj["updated_at"].clone();
        let updated_at_str: String = json::from_value(updated_at_value)?;
        let updated_at = Timestamp::from_string(&updated_at_str)?;

        let peer = Peer {
            address: address,
            blacklisted: blacklisted,
            created_at: created_at,
            updated_at: updated_at,
        };

        Ok(peer)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let peer = messagepack::from_slice(b)?;

        Ok(peer)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}

impl Default for Peer {
    fn default() -> Peer {
        let now = Timestamp::now();

        Peer {
            address: String::new(),
            blacklisted: false,
            created_at: now,
            updated_at: now,
        }
    }
}

impl<'a> Identify<'a> for Peer {
    type ID = String;

    fn id(&self) -> Result<Self::ID> {
        Ok(self.address.clone())
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
        Ok(id.into_bytes())
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        let mut utf8 = Vec::new();
        utf8.extend_from_slice(b);

        Ok(String::from_utf8(utf8)?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        let id = self.id()?;

        Self::id_to_bytes(id)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(String::from(s))
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id)
    }

    fn string_id(&self) -> Result<String> {
        Ok(self.address.clone())
    }
}

impl Validate for Peer {
    fn validate(&self) -> Result<()> {
        self.created_at.validate()?;
        self.updated_at.validate()?;

        if self.created_at > self.updated_at {
            return Err(ErrorKind::InvalidTimestamp.into());
        }

        Ok(())
    }
}
