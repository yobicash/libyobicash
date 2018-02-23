// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `get` module provides the Yobicash network get handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::{Validate, Serialize};
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::{Message, GetRequest, GetResponse};

/// The type used for handling `Get` messages.
#[derive(Clone, Debug)]
pub struct GetHandler;

impl GetHandler {
    /// Handles a get request.
    pub fn handle<S: Store>(node: &mut Node<S>, session: &Session, req: &GetRequest) -> Result<Message> {
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

                let transaction = node.get_transaction(resource_id)?;
                
                transaction.validate()?;

                if transaction.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = transaction.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::WriteOp => {
                let resource_id = req.resource_id;

                let write_op = node.get_write_op(resource_id)?;
                
                write_op.validate()?;

                if write_op.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = write_op.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::UnspentCoin => {
                let resource_id = req.resource_id;

                let coin = node.get_unspent_coin(resource_id)?;
                
                coin.validate()?;

                if coin.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = coin.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::SpentCoin => {
                let resource_id = req.resource_id;

                let coin = node.get_spent_coin(resource_id)?;
                
                coin.validate()?;

                if coin.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = coin.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::UnspentOutput => {
                let resource_id = req.resource_id;

                let output = node.get_unspent_output(resource_id)?;
                
                output.validate()?;

                if output.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = output.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::SpentOutput => {
                let resource_id = req.resource_id;

                let output = node.get_spent_output(resource_id)?;
                
                output.validate()?;

                if output.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = output.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            ResourceType::UndeletedData => {
                let resource_id = req.resource_id;

                let data = node.get_undeleted_data(resource_id)?;
                
                data.validate()?;

                if data.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let resource = data.to_bytes()?;

                let res = GetResponse::new(session, resource_type, &resource)?;

                let message = Message::GetResponse(res);

                Ok(message)
            },
            _ => Err(ErrorKind::InvalidResource.into()),
        }
    }
}
