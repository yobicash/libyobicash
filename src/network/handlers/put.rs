// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `put` module provides the Yobicash network put handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::{Validate, Serialize};
use models::{Transaction, WriteOp, DeleteOp, Coin, Output, Data};
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::{Message, PutRequest, PutResponse};

/// The type used for handling `Put` messages.
#[derive(Clone, Debug)]
pub struct PutHandler;

impl PutHandler {
    /// Handles a put request.
    pub fn handle<S: Store>(node: &mut Node<S>, session: &Session, req: &PutRequest) -> Result<Message> {
        session.validate()?;

        if req.id != session.id {
            return Err(ErrorKind::InvalidID.into());
        }

        if req.network_type != session.network_type {
            return Err(ErrorKind::InvalidSession.into());
        }

        if session.max_size.is_none() {
            return Err(ErrorKind::InvalidSession.into());
        }

        if req.max_size != session.max_size.unwrap() {
            return Err(ErrorKind::InvalidLength.into());
        }

        req.validate()?;

        let resource_type = req.resource_type;

        match resource_type {
            ResourceType::Transaction => {
                let transaction = Transaction::from_bytes(&req.resource)?;

                transaction.validate()?;

                if transaction.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
                
                node.add_transaction(&transaction)?;

                let resource_id = transaction.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            ResourceType::WriteOp => {
                let write_op = WriteOp::from_bytes(&req.resource)?;

                write_op.validate()?;

                if write_op.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
                
                node.add_write_op(&write_op)?;

                let resource_id = write_op.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            ResourceType::DeleteOp => {
                let delete_op = DeleteOp::from_bytes(&req.resource)?;

                delete_op.validate()?;

                if delete_op.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
                
                node.add_delete_op(&delete_op)?;

                let resource_id = delete_op.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            ResourceType::UnspentCoin => {
                if req.source.is_some() || req.source_id.is_some() || req.write_id.is_some() {
                    return Err(ErrorKind::InvalidMessage.into());
                }

                let coin = Coin::from_bytes(&req.resource)?;

                coin.validate()?;

                if coin.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
                
                node.add_unspent_coin(&coin)?;

                let resource_id = coin.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            ResourceType::UnspentOutput => {
                if req.source.is_none() {
                    return Err(ErrorKind::InvalidMessage.into());
                }
                    
                if req.source_id.is_none() {
                    return Err(ErrorKind::InvalidMessage.into());
                }

                if req.write_id.is_some() {
                    return Err(ErrorKind::InvalidMessage.into());
                }

                let source = req.source.unwrap();
                let source_id = req.source_id.unwrap();

                let output = Output::from_bytes(&req.resource)?;

                output.validate()?;

                if output.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }
                
                node.add_unspent_output(source, source_id, &output)?;

                let resource_id = output.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            ResourceType::UndeletedData => {
                if req.source.is_some() || req.source_id.is_some() {
                    return Err(ErrorKind::InvalidMessage.into());
                }

                if req.write_id.is_none() {
                    return Err(ErrorKind::InvalidMessage.into());
                }
                    
                let data = Data::from_bytes(&req.resource)?;

                data.validate()?;

                if data.network_type != req.network_type {
                    return Err(ErrorKind::InvalidNetwork.into());
                }

                let write_id = req.write_id.unwrap();
                
                node.add_undeleted_data(write_id, &data)?;

                let resource_id = data.id;

                let res = PutResponse::new(session, resource_type, resource_id)?;

                let message = Message::PutResponse(res);

                Ok(message)
            },
            _ => Err(ErrorKind::InvalidResource.into()),
        }
    }
}
