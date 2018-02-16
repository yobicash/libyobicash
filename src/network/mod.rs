// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `network` module provides the Yobicash network traits, types and methods.

use result::Result;
use traits::{Validate, Serialize};
use dagchain::Dagchain;
use store::Store;
use filter::Filter;

/*
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ProtocolMessage {
    /// The id of the message.
    pub id: Digest,
    /// The method called.
    pub method_type: RPCMethodType,
    /// The size of the content.
    pub content_size: u32,
    /// The content of the message.
    pub content: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct NetworkMessage {
    /// The id of the message.
    pub id: Digest,
    /// The version of the protocol.
    pub version: Version,
    /// The network type of the protocol.
    pub network_type: NetworkType,
    /// The client public key.
    pub client: PublicKey,
    /// The server public key.
    pub server: PublicKey,
    /// The total size of the ProtocolMessage.
    pub total_size: u32,
    /// The chunks count.
    pub chunks_count: u32,
    /// The chunk index.
    pub chunk_index: u32,
    /// The chunk plaintext size.
    pub plain_size: u32,
    /// The chunk cyphertext size.
    pub cyph_size: u32,
    /// The chunk cyphertext.
    pub cyph: Vec<u8>,
}

pub struct ConnectionConfig {}

pub trait Connection {}

pub struct ServerConfig {}

pub struct Server {}

pub struct ClientConfig {}

pub struct Client {}

*/
