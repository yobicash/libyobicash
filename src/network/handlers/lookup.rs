// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `lookup` module provides the Yobicash network lookup handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::{Message, LookupRequest, LookupResponse};

/// The type used for handling `Lookup` messages.
#[derive(Clone, Debug)]
pub struct LookupHandler;

impl LookupHandler {
    /// Handles a lookup request.
    pub fn handle<S: Store>(node: &mut Node<S>, session: &Session, req: &LookupRequest) -> Result<Message> {
        session.validate()?;

        if req.id != session.id {
            return Err(ErrorKind::InvalidID.into());
        }

        if req.network_type != session.network_type {
            return Err(ErrorKind::InvalidSession.into());
        }

        req.validate()?;

        let resource_type = req.resource_type;

        match resource_type {
            ResourceType::Transaction => {
                let resource_id = req.resource_id;

                let found = node.lookup_transaction(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::WriteOp => {
                let resource_id = req.resource_id;

                let found = node.lookup_write_op(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::UnspentCoin => {
                let resource_id = req.resource_id;

                let found = node.lookup_unspent_coin(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::SpentCoin => {
                let resource_id = req.resource_id;

                let found = node.lookup_spent_coin(resource_id)?;
                
                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::UnspentOutput => {
                let resource_id = req.resource_id;

                let found = node.lookup_unspent_output(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::SpentOutput => {
                let resource_id = req.resource_id;

                let found = node.lookup_spent_output(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            ResourceType::UndeletedData => {
                let resource_id = req.resource_id;

                let found = node.lookup_undeleted_data(resource_id)?;

                let res = LookupResponse::new(session, resource_type, found)?;

                let message = Message::LookupResponse(res);

                Ok(message)
            },
            _ => Err(ErrorKind::InvalidResource.into()),
        }
    }
}
