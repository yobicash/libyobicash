// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `traits` module provides the library traits.

use serde;

use result::Result;

/// Trait for types that can be validated.
pub trait Validate {
    /// Validate the object.
    fn validate(&self) -> Result<()>;
}

/// A trait used for identifying univocally an item.
pub trait Identify<'a> {
    /// Type of the identifier.
    type ID: serde::Serialize + serde::Deserialize<'a>;

    /// Returns the `ID` of the implementor.
    fn id(&self) -> Result<Self::ID>;

    /// Converts a binary to an `ID`.
    fn id_from_bytes(b: &[u8]) -> Result<Self::ID>;
    
    /// Converts an `ID` to bytes.
    fn id_to_bytes(id: Self::ID) -> Result<Vec<u8>>;
    
    /// Returns the binary representation of the `ID` of the object.
    fn binary_id(&self) -> Result<Vec<u8>>;

    /// Converts a string to an `ID`.
    fn id_from_string(s: &str) -> Result<Self::ID>;
    
    /// Converts an `ID` to string.
    fn id_to_string(id: Self::ID) -> Result<String>;
    
    /// Returns the string representation of the `ID` of the object.
    fn string_id(&self) -> Result<String>;
}

/// Trait for object that can be serialized from and to binary.
pub trait BinarySerialize: Sized {
    /// Serialize to a binary.
    fn to_bytes(&self) -> Result<Vec<u8>>;

    /// Deserialize from a binary.
    fn from_bytes(b: &[u8]) -> Result<Self>;
}

/// Trait for object that can be serialized from and to hex.
pub trait HexSerialize: Sized {
    /// Serialize to a binary.
    fn to_hex(&self) -> Result<String>;

    /// Deserialize from a binary.
    fn from_hex(b: &str) -> Result<Self>;
}

/// A trait for object that can be serialized in JSON and bytes.
pub trait Serialize<'a>: Sized + serde::Serialize + serde::Deserialize<'a> {
    /// Serialize to a json string.
    fn to_json(&self) -> Result<String>;
    
    /// Deserialize from a json string.
    fn from_json(s: &str) -> Result<Self>;
    
    /// Serialize to a binary.
    fn to_bytes(&self) -> Result<Vec<u8>>;
    
    /// Deserialize from a binary.
    fn from_bytes(b: &[u8]) -> Result<Self>;
    
    /// Serialize to a hex.
    fn to_hex(&self) -> Result<String>;

    /// Deserialize from a hex.
    fn from_hex(s: &str) -> Result<Self>;
}
