// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `network_types` module provides the network types used throughout the library.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hex;

use constants::{MAINPORT, TESTPORT, REGTESTPORT};
use error::ErrorKind;
use result::Result;
use traits::{BinarySerialize, HexSerialize};

/// The possible network types in Yobicash.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum NetworkType {
    MainNet=0,
    TestNet=1,
    RegTest=2,
}

impl NetworkType {
    /// Returns the network type port.
    pub fn port(&self) -> u16 {
        match *self {
            NetworkType::MainNet => MAINPORT,
            NetworkType::TestNet => TESTPORT,
            NetworkType::RegTest => REGTESTPORT,
        }
    }
}

impl Default for NetworkType {
    fn default() -> NetworkType {
        NetworkType::TestNet
    }
}

impl BinarySerialize for NetworkType {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write_u32::<BigEndian>(*self as u32)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<NetworkType> {
        let len = b.len();
        if len != 4 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let n: u32 = BigEndian::read_u32(b);

        match n {
            0 => Ok(NetworkType::MainNet),
            1 => Ok(NetworkType::TestNet),
            2 => Ok(NetworkType::RegTest),
            _ => Err(ErrorKind::UnknownNetwork.into()),
        }
    }
}

impl HexSerialize for NetworkType {
    fn from_hex(s: &str) -> Result<NetworkType> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        NetworkType::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}
