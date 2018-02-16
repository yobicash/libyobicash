// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `mode` module provides the store mode type.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hex;

use error::ErrorKind;
use result::Result;
use traits::{BinarySerialize, HexSerialize};

/// The store mode.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum StoreMode {
    /// Memory mode.
    Memory=0,
    /// Persistent mode.
    Persistent=1,
}

impl Default for StoreMode {
    fn default() -> StoreMode {
        StoreMode::Memory
    }
}

impl BinarySerialize for StoreMode {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write_u32::<BigEndian>(*self as u32)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<StoreMode> {
        let len = b.len();
        if len != 4 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let n: u32 = BigEndian::read_u32(b);

        match n {
            0 => Ok(StoreMode::Memory),
            1 => Ok(StoreMode::Persistent),
            _ => Err(ErrorKind::UnknownMode.into()),
        }
    }
}

impl HexSerialize for StoreMode {
    fn from_hex(s: &str) -> Result<StoreMode> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        StoreMode::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}
