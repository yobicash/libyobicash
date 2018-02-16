// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `value` module provides the store value type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use result::Result;
use traits::Serialize;
use crypto::{Key, sym_encrypt, sym_decrypt};

/// A store value.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct StoreValue {
    /// Plaintext size.
    pub size: u32,
    /// Encrypted value.
    pub cyph: Vec<u8>,
}

impl StoreValue {
    /// Creates a `StoreValue` from an object T.
    pub fn from_object<'a, T: Serialize<'a>>(obj: &T, key: Key) -> Result<StoreValue> {
        let msg = obj.to_bytes()?;
        let size = msg.len() as u32;

        let cyph = sym_encrypt(key, &msg)?;

        let value = StoreValue {
            size: size,
            cyph: cyph,
        };
        
        Ok(value)
    }

    /// Converts a `StoreValue` to an object T.
    pub fn to_object<'a, T: Serialize<'a>>(&self, key: Key) -> Result<T> {
        let msg = sym_decrypt(key, &self.cyph, self.size)?;

        T::from_bytes(&msg)
    }
}

impl<'a> Serialize<'a> for StoreValue {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "size": self.size,
            "cyph": &hex::encode(self.cyph.clone()),
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let size_value = obj["size"].clone();
        let size: u32 = json::from_value(size_value)?;

        let cyph_value = obj["cyph"].clone();
        let cyph_hex: String = json::from_value(cyph_value)?;
        let cyph = hex::decode(&cyph_hex)?;

        let value = StoreValue {
            size: size,
            cyph: cyph,
        };

        Ok(value)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let value = messagepack::from_slice(b)?;

        Ok(value)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
