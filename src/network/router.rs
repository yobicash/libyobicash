// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `resource` module provides the Yobicash network resource message type and methods.

use serde_json as json;
use rmp_serde as messagepresource;
use hex;

use error::ErrorKind;
use result::Result;
use traits::{Validate, HexSerialize, Serialize};
use utils::{Version, NetworkType};
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::Message;
use network::handlers::*;

/// The type used for routing message to the node.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Router<S: Store> {
    /// The node the router is wrapping.
    node: Node<S>,
}

impl<S: Store> Router<S> {
    /// Creates a new `Router`.
    pub fn new(node: &Node<S>) -> Router<S> {
        Router { node: node.clone() }
    }

    /// Routes a message to the inner node, producing a new `Message` or an `Error`.
    pub fn route<S: Store>(&mut self, message: &Message) -> Result<Message> {
        match message {
            &Message::Ping(ping) => PingHandler::handle(&mut self.node, ping),
            &Message::ListRequest(req) => ListHandler::handle(&mut self.node, req),
            &Message::SampleRequest(req) => SampleHandler::handle(&mut self.node, req),
            &Message::GetRequest(req) => GetHandler::handle(&mut self.node, req),
            &Message::LookupRequest(req) => LookupHandler::handle(&mut self.node, req),
            &Message::PutRequest(req) => PutHandler::handle(&mut self.node, req),
        }
    }
}
