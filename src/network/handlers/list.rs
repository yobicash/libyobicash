// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `list` module provides the Yobicash network list handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::{Message, ListRequest, ListResponse};

/// The type used for handling `List` messages.
#[derive(Clone, Debug)]
pub struct ListHandler;

impl ListHandler {
    /// Handles a list request.
    pub fn handle<S: Store>(node: &mut Node<S>, session: &Session, req: &ListRequest) -> Result<Message> {
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
            ResourceType::Peer => {
                let addresses = node.list_peers()?;
              
                let mut resources = Vec::new();

                for address in addresses {
                    resources.push(address.into_bytes());
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::Transaction => {
                let ids = node.list_transactions()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::WriteOp => {
                let ids = node.list_write_ops()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::UnspentCoin => {
                let ids = node.list_unspent_coins()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::SpentCoin => {
                let ids = node.list_spent_coins()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::UnspentOutput => {
                let ids = node.list_unspent_outputs()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::SpentOutput => {
                let ids = node.list_spent_outputs()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
            ResourceType::UndeletedData => {
                let ids = node.list_undeleted_data()?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = ListResponse::new(session, resource_type, &resources)?;

                let message = Message::ListResponse(res);

                Ok(message)
            },
        }
    }
}
