// Copyright 2018 Yobicash Ltd. See the COPYRIGHT file at the top-level directory
// of this distribution.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! Libyobicash test mocks.

use libyobicash::error::ErrorKind;
use libyobicash::result::Result;
use libyobicash::traits::*;

/// Mocked type used in testing with no interesting features.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Unit;

impl Validate for Unit {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl<'a> Identify<'a> for Unit {
    type ID = Unit;

    fn id(&self) -> Result<Self::ID> {
        Ok(Unit {})
    }

    fn id_from_bytes(b: &[u8]) -> Result<Self::ID> {
        if b != [1u8] {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(Unit {})
    }

    fn id_to_bytes(_id: Self::ID) -> Result<Vec<u8>> {
        Ok(vec![1u8])
    }

    fn binary_id(&self) -> Result<Vec<u8>> {
        Ok(vec![1u8])
    }

    fn id_from_string(s: &str) -> Result<Self::ID> {
        if s != "1" {
            return Err(ErrorKind::DeserializationFailure.into());
        }

        Ok(Unit {})
    }

    fn id_to_string(_id: Self::ID) -> Result<String> {
        Ok(String::from(""))
    }

    fn string_id(&self) -> Result<String> {
        Ok(String::from(""))
    }
}

impl BinarySerialize for Unit {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(vec![1u8])
    }

    fn from_bytes(b: &[u8]) -> Result<Self> {
        if b != [1u8] {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(Unit {})
    }
}

impl<'a> Serialize<'a> for Unit {
    fn to_json(&self) -> Result<String> {
        Ok(String::from("1"))
    }

    fn from_json(s: &str) -> Result<Self> {
        if s != "1" {
            return Err(ErrorKind::DeserializationFailure.into())
        }

        Ok(Unit {})
    }
    
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(vec![1u8])
    }

    fn from_bytes(b: &[u8]) -> Result<Self> {
        if b != [1u8] {
            return Err(ErrorKind::DeserializationFailure.into())
        }

        Ok(Unit {})
    }
    
    fn to_hex(&self) -> Result<String> {
        Ok(String::from("1"))
    }

    fn from_hex(s: &str) -> Result<Self> {
        if s != "1" {
            return Err(ErrorKind::InvalidLength.into());
        }

        Ok(Unit {})
    }
}
