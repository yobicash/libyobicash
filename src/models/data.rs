// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `data` module provides the data types and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;
use byteorder::{BigEndian, WriteBytesExt};

use error::ErrorKind;
use result::Result;
use traits::{Identify, Validate, BinarySerialize, HexSerialize, Serialize};
use crypto::{Digest, SecretKey, PublicKey};
use crypto::{assym_encrypt, assym_decrypt};
use crypto::BinarySerialize as CryptoBinarySerialize;
use crypto::HexSerialize as CryptoSerialize;
use utils::{Version, NetworkType};

use std::io::Write;

/// Data is encrypted data written with a `WriteOp`.
#[derive(Clone, Eq, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Data {
    /// The id of the data.
    pub id: Digest,
    /// The protocol version.
    pub version: Version,
    /// The protocol network type.
    pub network_type: NetworkType,
    /// The from is the public key used by the sender from encrypt the data.
    pub from: PublicKey,
    /// The to is the public key that can be used by the receiver to decrypt the data.
    pub to: PublicKey,
    /// The plaintext size.
    pub plain_size: u32,
    /// The cyphertext size.
    pub cyph_size: u32,
    /// The cyphertext is the encrypted data.
    pub cyphertext: Vec<u8>,
}

impl Data {
    /// Creates a new `Data`.
    pub fn new(network_type: NetworkType, sk: SecretKey, pk: PublicKey, plaintext: &[u8]) -> Result<Data> {
        let cyphertext = assym_encrypt(sk, pk, plaintext)?;

        let mut data = Data::default();

        data.network_type = network_type;
        data.from = sk.to_public();
        data.to = pk;
        data.plain_size = plaintext.len() as u32;
        data.cyph_size = cyphertext.len() as u32;
        data.cyphertext = cyphertext;
        data.id = data.id()?;

        Ok(data)
    }

    /// Decrypts the `Data` cyphertext.
    pub fn decrypt(&self, sk: SecretKey) -> Result<Vec<u8>> {
        self.validate()?;

        if sk.to_public() != self.to {
            return Err(ErrorKind::InvalidSecretKey.into());
        }

        Ok(assym_decrypt(sk, self.from, &self.cyphertext, self.plain_size)?)
    }
}

impl<'a> Identify<'a> for Data {
    type ID = Digest;

    fn id(&self) -> Result<Self::ID> {
        let mut buf = Vec::new();

        buf.write_all(&self.version.to_bytes()?)?;
        buf.write_all(&self.network_type.to_bytes()?)?;
        buf.write_all(&self.from.to_bytes()?)?;
        buf.write_all(&self.to.to_bytes()?)?;
        buf.write_u32::<BigEndian>(self.plain_size)?;
        buf.write_u32::<BigEndian>(self.cyph_size)?;
        buf.write_all(&self.cyphertext)?;

        Ok(Digest::hash(&buf))
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        Ok(Digest::from_bytes(b)?)
    }

    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>> {
        Ok(id.to_bytes()?)
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        let id = self.id()?;

        Self::id_to_bytes(id)
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        Ok(Digest::from_hex(s)?)
    }

    fn id_to_string(id: Self::ID) -> Result<String> {
        Ok(id.to_hex()?)
    }

    fn string_id(&self) -> Result<String> {
        let id = self.id()?;

        Self::id_to_string(id)
    }
}

impl Validate for Data {
    fn validate(&self) -> Result<()> {
        self.version.validate()?;

        if self.from == self.to {
            return Err(ErrorKind::InvalidPublicKey.into());
        }

        let plain_size = self.plain_size;
        let cyph_size = self.cyph_size;

        if plain_size > cyph_size {
            return Err(ErrorKind::InvalidLength.into());
        }

        if plain_size % 16 == 0 {
            return Err(ErrorKind::InvalidLength.into());
        }

        if cyph_size % 16 != 0 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let padding = 16 - (plain_size % 16);

        if cyph_size != plain_size + padding {
            return Err(ErrorKind::InvalidLength.into());
        }

        if self.cyph_size as usize != self.cyphertext.len() {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(())
    }
}

impl<'a> Serialize<'a> for Data {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "id": self.string_id()?,
            "version": self.version.to_string(),
            "network_type": self.network_type.to_hex()?,
            "from": self.from.to_hex()?,
            "to": self.to.to_hex()?,
            "plain_size": self.plain_size,
            "cyph_size": self.cyph_size,
            "cyphertext": hex::encode(&self.cyphertext),
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let id_value = obj["id"].clone();
        let id_str: String = json::from_value(id_value)?;
        let id = Data::id_from_string(&id_str)?;
        
        let version_value = obj["version"].clone();
        let version_string: String = json::from_value(version_value)?;
        let version = Version::from_string(&version_string)?;
        
        let network_type_value = obj["network_type"].clone();
        let network_type_hex: String = json::from_value(network_type_value)?;
        let network_type = NetworkType::from_hex(&network_type_hex)?;
        
        let from_value = obj["from"].clone();
        let from_hex: String = json::from_value(from_value)?;
        let from = PublicKey::from_hex(&from_hex)?;
        
        let to_value = obj["to"].clone();
        let to_hex: String = json::from_value(to_value)?;
        let to = PublicKey::from_hex(&to_hex)?;

        let plain_size_value = obj["plain_size"].clone();
        let plain_size: u32 = json::from_value(plain_size_value)?;

        let cyph_size_value = obj["cyph_size"].clone();
        let cyph_size: u32 = json::from_value(cyph_size_value)?;
        
        let cyphertext_value = obj["cyphertext"].clone();
        let cyphertext_hex: String = json::from_value(cyphertext_value)?;
        let cyphertext = hex::decode(&cyphertext_hex)?;

        let data = Data {
            id: id,
            version: version,
            network_type: network_type,
            from: from,
            to: to,
            plain_size: plain_size,
            cyph_size: cyph_size,
            cyphertext: cyphertext,
        };

        Ok(data)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let data = messagepack::from_slice(b)?;

        Ok(data)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
