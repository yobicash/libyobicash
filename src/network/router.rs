// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `resource` module provides the Yobicash network resource message type and methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use store::Store;
use node::Node;
use network::session::Session;
use network::message::{Message, ErrorResponse};
use network::handlers::*;

/// The type used for routing message to the node.
#[derive(Clone, Debug)]
pub struct Router<S: Store> {
    /// The node the router is wrapping.
    node: Node<S>,
}

impl<S: Store> Router<S> {
    /// Creates a new `Router`.
    pub fn new(node: Node<S>) -> Router<S> {
        Router { node: node }
    }

    /// Routes a message to the inner node, producing a new `Message` or an `Error`.
    pub fn route(&mut self, session: &Session, message: &Message) -> Result<Message> {
        session.validate()?;

        match message {
            &Message::Ping(ref req) => {
                match PingHandler::handle(session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            &Message::ListRequest(ref req) => {
                match ListHandler::handle(&mut self.node, session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            &Message::SampleRequest(ref req) => {
                match SampleHandler::handle(&mut self.node, session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            &Message::GetRequest(ref req) => {
                match GetHandler::handle(&mut self.node, session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            &Message::LookupRequest(ref req) => {
                match LookupHandler::handle(&mut self.node, session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            &Message::PutRequest(ref req) => {
                match PutHandler::handle(&mut self.node, session, req) {
                    Ok(message) => Ok(message),
                    Err(e) => {
                        let err_response = ErrorResponse::new(session, &format!("{}", e))?;
                        Ok(Message::ErrorResponse(err_response))
                    },
                }
            },
            _ => Err(ErrorKind::InvalidMessage.into())
        }
    }
}
