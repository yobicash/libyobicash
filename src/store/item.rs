// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `item` module provides the store item type and methods.

use serde_json as json;
use rmp_serde as messagepack;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Identify, HexSerialize, Serialize};
use crypto::Key;
use store::key::StoreKey;
use store::value::StoreValue;

/// A store item.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct StoreItem {
    /// Store key.
    pub key: StoreKey,
    /// Store value.
    pub value: StoreValue,
}

impl StoreItem {
    /// Creates a `StoreItem` from an object T.
    pub fn from_object<'a, T: Identify<'a> + Serialize<'a>>(obj: &T, key: Key) -> Result<StoreItem> {
        let id = obj.id()?;
        let store_key = StoreKey::from_id::<T>(id)?;
        let store_value = StoreValue::from_object(obj, key)?;

        let store_item: StoreItem = StoreItem {
            key: store_key,
            value: store_value,
        };

        Ok(store_item)
    }

    /// Converts a `StoreItem` to an object T.
    pub fn to_object<'a, T: Identify<'a> + Serialize<'a>>(&self, key: Key) -> Result<T> {
        let obj = self.value.to_object::<T>(key)?;
        let id = self.key.to_id::<T>()?;

        if obj.binary_id()? != T::id_to_bytes(id)? {
            return Err(ErrorKind::InvalidID.into());
        }

        Ok(obj)
    }
}

impl<'a> Serialize<'a> for StoreItem {
    fn to_json(&self) -> Result<String> {
        let obj = json!({
            "key": self.key.to_hex()?,
            "value": self.value.to_json()?,
        });

        let s = obj.to_string();

        Ok(s)
    }
    
    fn from_json(s: &str) -> Result<Self> {
        let obj: json::Value = json::from_str(s)?;
        
        let key_value = obj["key"].clone();
        let key_hex: String = json::from_value(key_value)?;
        let key = StoreKey::from_hex(&key_hex)?;

        let value_value = obj["value"].clone();
        let value_json: String = json::from_value(value_value)?;
        let value = StoreValue::from_json(&value_json)?;

        let item = StoreItem {
            key: key,
            value: value,
        };

        Ok(item)
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let buf = messagepack::to_vec(self)?;

        Ok(buf)
    }
    
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let item = messagepack::from_slice(b)?;

        Ok(item)
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    fn from_hex(s: &str) -> Result<Self> {
        Self::from_bytes(&hex::decode(s)?)
    }
}
