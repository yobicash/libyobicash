// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `syn_ack` module provides the Yobicash network syn-ack message type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use constants::MAX_CHUNK_SIZE;
use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use crypto::{Digest, BalloonParams, PoW};
use crypto::HexSerialize as CryptoHexSerialize;
use utils::{Version, NetworkType};
use network::session::Session;

/// The type used for syn-ack messages in the Yobicash handshake.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct SynAck {
    /// Id of the session.
    pub id: u32,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// Maximum size of a `Resource` message.
    pub max_size: u32,
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
    /// Difficulty of the proof-of-work.
    pub pow_difficulty: u32,
    /// Nonce of the proof-of-work.
    pub pow_nonce: u64,
    /// Digest of the proof-of-work.
    pub pow_digest: Option<Digest>,
}

impl SynAck {
    /// Creates a new `SynAck`.
    pub fn new(session: &Session,
               plain_size: u32,
               cyph_size: u32,
               plain_digest: Digest,
               cyph_digest: Digest) -> Result<SynAck> {
        session.validate()?;

        if session.max_size.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }

        if session.pow_difficulty.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }

        let id = session.id;
        let version = session.version.clone();
        let network_type = session.network_type;
        let max_size = session.max_size.unwrap();
        let pow_difficulty = session.pow_difficulty.unwrap();

        if plain_size > cyph_size {
            return Err(ErrorKind::OutOfBound.into());
        }

        let plain_padding = plain_size % 16;

        if plain_size + plain_padding > max_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        let padding = cyph_size % MAX_CHUNK_SIZE;
        let chunks_count = (cyph_size + padding) / MAX_CHUNK_SIZE;
        
        if plain_digest == cyph_digest {
            return Err(ErrorKind::InvalidDigest.into());
        }

        let mut syn_ack = SynAck {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            plain_size: plain_size,
            cyph_size: cyph_size,
            padding: padding,
            chunks_count: chunks_count,
            plain_digest: plain_digest,
            cyph_digest: cyph_digest,
            pow_difficulty: pow_difficulty,
            pow_nonce: 0,
            pow_digest: None,
        };

        let pow_salt = Digest::hash(&syn_ack.to_bytes()?);

        let pow_params = BalloonParams::from_memory(cyph_size)?;

        let mut pow = PoW::new(pow_salt, pow_params, pow_difficulty)?;

        pow.mine()?;

        if !pow.verify()? {
            return Err(ErrorKind::NotFound.into());
        }

        syn_ack.pow_nonce = pow.nonce.unwrap();
        syn_ack.pow_digest = pow.digest;

        Ok(syn_ack)
    }
}

impl Validate for SynAck {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        let max_size = self.max_size;
        let plain_size = self.plain_size;
        let cyph_size = self.cyph_size;
        let padding = self.padding;
        let chunks_count = self.chunks_count;
        let plain_digest = self.plain_digest;
        let cyph_digest = self.cyph_digest;
        let pow_difficulty = self.pow_difficulty;
        let pow_nonce = self.pow_nonce;

        if self.pow_digest.is_none() {
            return Err(ErrorKind::InvalidMessage.into());
        }

        let pow_digest = self.pow_digest.unwrap();

        if plain_size > cyph_size {
            return Err(ErrorKind::OutOfBound.into());
        }

        let plain_padding = plain_size % 16;

        if plain_size + plain_padding > max_size {
            return Err(ErrorKind::InvalidLength.into());
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

        let mut pow_syn_ack = self.clone();
        pow_syn_ack.pow_nonce = 0;
        pow_syn_ack.pow_digest = None;

        let pow_salt = Digest::hash(&pow_syn_ack.to_bytes()?);

        let pow_params = BalloonParams::from_memory(cyph_size)?;

        let pow = PoW {
            salt: pow_salt,
            params: pow_params,
            difficulty: pow_difficulty,
            nonce: Some(pow_nonce),
            digest: Some(pow_digest),
        };

        if !pow.verify()? {
            return Err(ErrorKind::InvalidMessage.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for SynAck {
    fn to_json(&self) -> Result<String> {
        let pow_digest_hex = if self.pow_digest.is_none() {
            String::from("")
        } else {
            self.pow_digest.unwrap().to_hex()?
        };
        
        let obj = json!({
            "id": self.id,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "max_size": self.max_size,
            "plain_size": self.plain_size,
            "cyph_size": self.cyph_size,
            "padding": self.padding,
            "chunks_count": self.chunks_count,
            "plain_digest": self.plain_digest.to_hex()?,
            "cyph_digest": self.cyph_digest.to_hex()?,
            "pow_nonce": self.pow_nonce,
            "pow_digest": pow_digest_hex,
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

        let pow_difficulty_value = obj["pow_difficulty"].clone();
        let pow_difficulty: u32 = json::from_value(pow_difficulty_value)?;

        let pow_nonce_value = obj["pow_nonce"].clone();
        let pow_nonce: u64 = json::from_value(pow_nonce_value)?;
      
        let pow_digest_value = obj["pow_digest"].clone();
        let pow_digest_hex: String = json::from_value(pow_digest_value)?;
        let pow_digest = if pow_digest_hex.is_empty() {
            None
        } else {
            Some(Digest::from_hex(&pow_digest_hex)?)
        };

        let syn_ack = SynAck {
            id: id,
            version: version,
            network_type: network_type,
            max_size: max_size,
            plain_size: plain_size,
            cyph_size: cyph_size,
            padding: padding,
            chunks_count: chunks_count,
            plain_digest: plain_digest,
            cyph_digest: cyph_digest,
            pow_difficulty: pow_difficulty,
            pow_nonce: pow_nonce,
            pow_digest: pow_digest,
        };

        Ok(syn_ack)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let syn_ack = messagepack::from_slice(b)?;

        Ok(syn_ack)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
