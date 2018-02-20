// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `resource_type` module provides the network resource types.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use hex;

use error::ErrorKind;
use result::Result;
use traits::{BinarySerialize, HexSerialize};

/// The message resource types.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ResourceType {
    Peer=0,
    Transaction=1,
    WriteOp=2,
    DeleteOp=3,
    UnspentCoin=4,
    SpentCoin=5,
    UnspentOutput=6,
    SpentOutput=7,
    UndeletedData=8,
}

impl Default for ResourceType {
    fn default() -> ResourceType {
        ResourceType::Peer
    }
}

impl BinarySerialize for ResourceType {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write_u32::<BigEndian>(*self as u32)?;

        Ok(buf)
    }

    fn from_bytes(b: &[u8]) -> Result<ResourceType> {
        let len = b.len();
        if len != 4 {
            return Err(ErrorKind::InvalidLength.into());
        }

        let n: u32 = BigEndian::read_u32(b);

        match n {
            0 => Ok(ResourceType::Peer),
            1 => Ok(ResourceType::Transaction),
            2 => Ok(ResourceType::WriteOp),
            3 => Ok(ResourceType::DeleteOp),
            4 => Ok(ResourceType::UnspentCoin),
            5 => Ok(ResourceType::SpentCoin),
            6 => Ok(ResourceType::UnspentOutput),
            7 => Ok(ResourceType::SpentOutput),
            8 => Ok(ResourceType::UndeletedData),
            _ => Err(ErrorKind::UnknownResource.into()),
        }
    }
}

impl HexSerialize for ResourceType {
    fn from_hex(s: &str) -> Result<ResourceType> {
        if s.is_empty() {
            return Err(ErrorKind::InvalidLength.into());
        }
    
        ResourceType::from_bytes(&hex::decode(s)?)
    }

    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes()?))
    }
}
