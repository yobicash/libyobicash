// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `key` module provides the store key type and methods.

use hex;

use error::ErrorKind;
use result::Result;
use traits::{Identify, BinarySerialize, HexSerialize};

/// A store key.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct StoreKey(pub Vec<u8>);

impl StoreKey {
    /// Creates a `StoreKey` from an `ID`.
    pub fn from_id<'a, T: Identify<'a>>(id: T::ID) -> Result<StoreKey> {
        let _key = T::id_to_bytes(id)?;
        Ok(StoreKey(_key))
    }

    /// Converts a `StoreKey` to an object `ID`.
    pub fn to_id<'a, T: Identify<'a>>(&self) -> Result<T::ID> {
        T::id_from_bytes(&self.0)
    }
}

impl BinarySerialize for StoreKey {
    fn from_bytes(b: &[u8]) -> Result<StoreKey> {
        if b.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }

        let mut _key = Vec::new();
        _key.extend_from_slice(b);

        Ok(StoreKey(_key))
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.clone())
    }
}

impl HexSerialize for StoreKey {
    fn from_hex(s: &str) -> Result<StoreKey> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        StoreKey::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}
