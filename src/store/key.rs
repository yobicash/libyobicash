// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `key` module provides the store key type and methods.

use hex;

use result::Result;
use traits::{BinarySerialize, HexSerialize};

/// A store key.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct StoreKey(pub Vec<u8>);

impl StoreKey {
    /// Creates a new `StoreKey`.
    pub fn new(key: &[u8]) -> StoreKey {
        StoreKey(Vec::from(key))
    }
}

impl BinarySerialize for StoreKey {
    fn from_bytes(b: &[u8]) -> Result<StoreKey> {
        Ok(StoreKey(Vec::from(b)))
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.clone())
    }
}

impl HexSerialize for StoreKey {
    fn from_hex(s: &str) -> Result<StoreKey> {
        StoreKey::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}
