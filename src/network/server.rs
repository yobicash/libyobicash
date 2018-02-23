// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `server` module provides the Yobicash network server type and methods.

use constants::MAX_CHUNK_SIZE;
use result::Result;
use error::ErrorKind;
use traits::{Validate, Serialize};
use crypto::{Digest, assym_encrypt, assym_decrypt};
use utils::NetworkType;
use store::Store;
use node::Node;
use network::traits::Connection;
use network::session::Session;
use network::message::handshake::{Ack, Syn, SynAck};
use network::message::ResponseHeader;

use std::io::{Read, Write};
use std::net::Shutdown;

/// A network client in the Yobicash protocol.
pub struct Server<S: Store> {
    /// The server network type.
    network_type: NetworkType,
    /// The server maximum read/write buffer size per connection.
    max_size: u32,
    /// The server underlying node.
    node: Node<S>,
}

impl<S: Store> Server<S> {
    /// Creates a new `Server`.
    pub fn new(network_type: NetworkType,
               node: Node<S>,
               max_size: u32) -> Server<S> {
        Server {
            network_type: network_type,
            node: node,
            max_size: max_size,
        }
    }

    /// Handles a connection.
    fn handle<T: Connection>(&mut self, transport: &mut T, pow_difficulty: u32) -> Result<()> {
        unreachable!()
    }
}
